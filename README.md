Command line tool to find k most dominant colors in an image. Simple impl with kmeans++ to select initial centroids and kmeans or kmedians.

```
tool find the most dominant colors in an image

USAGE:
    color_map [FLAGS] [OPTIONS] <image_path>

FLAGS:
    -h, --help       Prints help information
    -c, --crop       crop image borders by 25% (for images with object at center)
        --debug      Save processed image to ./.tmp/ directory
    -m, --mean       Calculate using mean instead of median
    -V, --version    Prints version information

OPTIONS:
    -k <centroid_count>        Set number of centroids [default: 3]

ARGS:
    <image_path>    Path to image file
```
