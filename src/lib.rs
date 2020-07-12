use clap::App;
use clap::Arg;

use image::RgbaImage;
use image::DynamicImage;

use rand::Rng;

use std::process;
use std::collections::HashMap;

mod error;
mod image_process;

pub fn run() -> error::Result<()> {
    let matches = App::new("soup")
                    .version("0.1.0")
                    .author("Alexander")
                    .about("tool find the most dominant colors in an image")
                    .arg(Arg::with_name("centroid_count")
                         .help("Set number of centroids")
                         .short("k")
                         .takes_value(true)
                         .default_value("3"))
                    .arg(Arg::with_name("is_mean")
                         .short("m")
                         .long("mean")
                         .help("Calculate using mean instead of median"))
                    .arg(Arg::with_name("image_path")
                         .help("Path to image file")
                         .required(true))
                    .arg(Arg::with_name("is_cropping")
                         .short("c")
                         .long("crop")
                         .help("crop image borders by 25% (for images with object at center)"))
                    .arg(Arg::with_name("is_debug")
                         .long("debug")
                         .help("Save processed image to ./.tmp/ directory"))
                    .get_matches();

    let k: usize;
    match matches.value_of("centroid_count") {
        None => k = 3,
        Some(val) => k = val.parse::<usize>()?,
    }

    let config = Configuration {
        is_debug: matches.is_present("is_debug"),
        k,
        is_mean: matches.is_present("is_mean"),
        is_cropping: matches.is_present("is_cropping"),
        image_path: matches.value_of("image_path").unwrap().to_owned(),
    };

    let image: DynamicImage = image::open(&config.image_path)?;

    let processed_image: RgbaImage = image_process::process_image(image, &config);
    for c in kmean(&config, processed_image)? {
        println!("{}", c.to_hex_string());
    }
    Ok(())
}

#[derive(Debug)]
pub struct Configuration {
    pub is_cropping: bool,
    pub is_debug: bool,
    pub is_mean: bool,
    pub k: usize,
    pub image_path: String,
}

#[derive(Debug, Copy, Clone)] 
struct ColorContainer {
    r: u8,
    g: u8,
    b: u8,
    count: u32,
}

impl ColorContainer {
    fn to_hex_string(&self) -> String {
        format!("{:0>2X}{:0>2X}{:0>2X}", self.r, self.g, self.b,)
    }
}

fn kmean(config: &Configuration, image: RgbaImage) -> error::Result<Vec<ColorContainer>> {
    let k = config.k;
    let is_mean = config.is_mean;

    let unique_colors: Vec<ColorContainer> = get_unique_colors(image);

    if unique_colors.is_empty() {
        eprintln!("Couldn't find any colors. All colors are transparent.");
        process::exit(1);
    }

    if unique_colors.len() < k {
        eprintln!("Failed: k={} while only {} colors were found in the image.", k, unique_colors.len());
        process::exit(1);
    }
    let mut centroids: Vec<ColorContainer> = kmeans_seeds(k, &unique_colors);

    let mut buffer: Vec<Vec<ColorContainer>> = Vec::with_capacity(k);
    buffer.push(unique_colors);
    for _i in 1..k {
        buffer.push(Vec::new());
    }

    let max_iterations: i32 = 1000;
    for iteration in 1..=max_iterations {
        let mut change: bool = false;
        let mut temp_buffer: Vec<Vec<ColorContainer>> = Vec::with_capacity(k);
        for _i in 0..k {
            temp_buffer.push(Vec::new());
        }
        for i in 0..k {
            for c in buffer[i].iter() {
                let closest: usize = closest_centroid(c, &centroids);
                temp_buffer[closest].push(c.clone());
                if closest != i {
                    change = true;
                }
            }
        }

        if change && iteration == max_iterations {
            return Err(error::SoupError::MaxIterations(max_iterations))
        }

        buffer = temp_buffer;
        centroids = calculate_centroids(&buffer, is_mean);
    }

   Ok(centroids) 
}

fn get_unique_colors(image: RgbaImage) -> Vec<ColorContainer> {
    let mut color_map: HashMap<String, ColorContainer> = HashMap::new();
    for p in image.pixels() {
        // skip transparent colors
        if p[3] == 0 {
            continue;
        }

        let container = ColorContainer {
            r: p[0],
            g: p[1],
            b: p[2],
            count: 0,
        };

        let hex = container.to_hex_string();
        let c = color_map.entry(hex).or_insert(container);
        c.count += 1;
    }

    let mut colors: Vec<ColorContainer> = Vec::new();
    for (_, value) in color_map {
       colors.push(value);
    }
    colors
}

fn kmeans_seeds(k: usize, colors: &Vec<ColorContainer>) -> Vec<ColorContainer> {
    let mut centroid_seeds: Vec<ColorContainer> = Vec::new();

    let selected_index = rand::thread_rng().gen_range(0, colors.len());
    centroid_seeds.push(colors[selected_index].clone()); // FIXME is deref here bad?

    for _i in 1..k {
        let mut distances: Vec<f64> = Vec::new();
        let mut total_squared_distance = 0.0;

        for c in colors {
            let mut min_distance = 0.0;
            for (l, centroid) in &mut centroid_seeds.iter().enumerate() {
                let distance = distance(&centroid, &c);
                if l == 0 || distance < min_distance {
                    min_distance = distance;
                }
            }
            total_squared_distance += min_distance;
            distances.push(min_distance);
        }

        let prob_point = rand::thread_rng().gen::<f64>() * total_squared_distance;
        let mut cumulative_sum = 0.0;
        for (index, val) in distances.iter().enumerate() {
            if prob_point < cumulative_sum {
                centroid_seeds.push(colors[index].clone());
                break;
            }
            cumulative_sum += val;
        }
    }

    centroid_seeds
}

fn distance(centroid: &ColorContainer, point: &ColorContainer) -> f64 {
    // FIXME ugly casting
    let r2 = point.r as f64;
    let g2 = point.g as f64;
    let b2 = point.b as f64;

    let r1 = centroid.r as f64;
    let g1 = centroid.g as f64;
    let b1 = centroid.b as f64;

    (r2-r1).powi(2) + (g2-g1).powi(2) + (b2-b1).powi(2)
}

fn closest_centroid(point: &ColorContainer, centroids: &Vec<ColorContainer>) -> usize {
    let mut index: usize = 0;
    let mut min_distance = distance(&centroids[index], &point);
    for (i, c) in centroids.iter().enumerate() {
        let distance = distance(c, &point);
        if distance < min_distance {
            min_distance = distance;
            index = i;
        }
    }
    index
}

fn calculate_centroids(buffer: &Vec<Vec<ColorContainer>>, is_mean: bool) -> Vec<ColorContainer>{
    let mut new_centroids: Vec<ColorContainer> = Vec::new();

    for c in buffer {
        let centroid: ColorContainer;
        if is_mean {
            centroid = mean(c);
        } else {
            centroid = median(c);
        }
        new_centroids.push(centroid);
    }

    new_centroids
}

fn mean(colors: &Vec<ColorContainer>) -> ColorContainer{
    // FIXME ugly casting again
    let r: u32 = colors.iter().map(|c: &ColorContainer| c.r as u32).sum();
    let g: u32 = colors.iter().map(|c: &ColorContainer| c.g as u32).sum();
    let b: u32 = colors.iter().map(|c: &ColorContainer| c.b as u32).sum();
    let bucket_count: u32 = colors.iter().map(|c: &ColorContainer| c.count).sum();
    let color_length: u32 = colors.len() as u32;
    ColorContainer {
        r: (r/color_length) as u8,
        g: (g/color_length) as u8,
        b: (b/color_length) as u8,
        count: bucket_count,
    }
}

fn median(colors: &Vec<ColorContainer>) -> ColorContainer{
    let mut rs: Vec<u8> = colors.iter().map(|c: &ColorContainer| c.r).collect();
    let mut gs: Vec<u8> = colors.iter().map(|c: &ColorContainer| c.g).collect();
    let mut bs: Vec<u8> = colors.iter().map(|c: &ColorContainer| c.b).collect();
    let bucket_count: u32 = colors.iter().map(|c: &ColorContainer| c.count).sum();

    let mut midpoint_r: u8 = 0;
    let mut midpoint_g: u8 = 0;
    let mut midpoint_b: u8 = 0;

    if !rs.is_empty() {
        rs.sort();
        midpoint_r = rs[rs.len()/2];
    }

    if !gs.is_empty() {
        gs.sort();
        midpoint_g = gs[gs.len()/2];
    }

    if !bs.is_empty() {
        bs.sort();
        midpoint_b = bs[bs.len()/2];
    }
    ColorContainer {
        r: midpoint_r,
        g: midpoint_g,
        b: midpoint_b,
        count: bucket_count,
    }
}
