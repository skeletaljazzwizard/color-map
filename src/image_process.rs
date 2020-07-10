use image::RgbaImage;
use image::GenericImageView;
use image::Rgba;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct BackgroundMask {
    red: bool,
    green: bool,
    blue: bool,
    threshold: u8,
}

const DEFAULT_MASKS: [BackgroundMask; 2] = [
        BackgroundMask {
            red: true,
            green: true,
            blue: true,
            threshold: 200,
        },
        BackgroundMask {
            red: false,
            green: false,
            blue: false,
            threshold: 55,
        }
    ];
struct Point(u32, u32);
pub fn process_image(img: &mut RgbaImage, is_debug: bool) {
    let (width, height) = img.dimensions();
    if height == 0 || width == 0 {
        // exit program because image is 0 height or 0 width
    }

    println!("bounds => {:?}", img.bounds());
    let mut points: Vec<Point> = vec![Point(0, 0), Point(width-1, 0), Point(0, height-1), Point(width-1, height-1)];
    let masks: Vec<&BackgroundMask> = DEFAULT_MASKS.iter().filter(|m: &&BackgroundMask| points.iter().all(|p: &Point| debug_is_ignorable(img.get_pixel(p.0, p.1), m))).collect::<Vec<_>>();

    println!("all masks = {:?}", masks);

    if masks.is_empty() {
        return;
    }

    let selected_mask = masks.first().unwrap();
    if is_debug {
        println!("Mask image using {:?}", selected_mask);
    }


    // process image outline to outline image from it's background according to found mask.
    while points.len() > 0 {
        let p = points.pop().unwrap();
        let px = img.get_pixel_mut(p.0, p.1);
        if !is_transparent(px) && is_ignorable(px, selected_mask) {
            mark_pixel(px);
            if !is_transparent(px) {
                eprintln!("Error marking pixel {:?} at {}, {}", px, p.0, p.1);
            }

            if 0 < p.0 {
                if !is_transparent(img.get_pixel(p.0-1, p.1)) {
                    points.push(Point(p.0-1, p.1));
                }
            }

            if p.0 < width-1 {
                if !is_transparent(img.get_pixel(p.0+1, p.1)) {
                    points.push(Point(p.0+1, p.1));
                }
            }

            if 0 < p.1 {
                if !is_transparent(img.get_pixel(p.0, p.1-1)) {
                    points.push(Point(p.0, p.1-1));
                }
            }

            if p.1 < height-1 {
                if !is_transparent(img.get_pixel(p.0, p.1+1)) {
                    points.push(Point(p.0, p.1+1));
                }
            }
        }
    }

    if is_debug {
        // unwrapping here because debug mode.
        let temp_dir = "./.tmp";
        fs::create_dir_all(temp_dir).unwrap();
        let filename = format!("{}/tmp_{}.jpg", temp_dir, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
        img.save(filename).unwrap();
    }
}

fn debug_is_ignorable(p: &Rgba<u8>, mask: &BackgroundMask) -> bool {
    println!("mask = {:?}", mask);
    println!("pixel = {:?}", p);
    is_ignorable(p, mask)
}

fn is_ignorable(p: &Rgba<u8>, mask: &BackgroundMask) -> bool {
    if p[3] == 0 {
        return true
    }


    if !(mask.red || mask.green || mask.blue) {
        if p[0] > mask.threshold || p[1] > mask.threshold || p[2] > mask.threshold {
            return false
        }
    }

     if mask.red && mask.green && mask.blue {
        if p[0] < mask.threshold || p[1] < mask.threshold || p[2] < mask.threshold {
            return false
        }
    }
    true
}

// does this reference need to be mutable?
fn mark_pixel(point: &mut Rgba<u8>) {
    point[0] = 255;
    point[1] = 0;
    point[2] = 255;
    point[3] = 0;
}

fn is_transparent(p: &Rgba<u8>) -> bool {
    p[3] == 0
}
