//! Demo of SWT and scratchpad for learning rust
extern crate image;
extern crate imageproc;


use std::env;
use std::path::Path;
use std::fs;
use std::f32;
use image::{open, GrayImage, ImageBuffer, Luma};
use imageproc::edges::canny;
use imageproc::gradients::{vertical_sobel, horizontal_sobel};

pub struct Point{
    x: u32,
    y: u32
}

impl Point {
    fn distance(&self) -> f32 {
        let x2: f32 = (self.x as f32).powi(2);
        let y2: f32 = (self.y as f32).powi(2);
        let dist: f32 = (x2 + y2).sqrt();
        dist
    }
}


// Ray useful for extending from edge pixels.
// Includes an iterator that can help extend from one edge pixel to another in a loop.
pub struct Ray {
    origin: Point,
    current: Point,
    max_pt: Point,
    direction: f32,
    iteration: i16 // Note for edge detection we should really not use a velocity higher than 2
}

impl Ray {
    fn new(x: u32, y: u32, direction: f32, max_x: u32, max_y: u32) -> Ray {
        Ray {
            origin: Point{x: x, y: y},
            max_pt: Point{x: max_x, y: max_y},
            current: Point{x: x, y: y},
            direction: direction,
            iteration: 0
        }
    }
}

impl Iterator for Ray {
    // Ray traversal mechanism (assuming positional steps of one)
    type Item = (Point);

    fn next(&mut self) -> Option<Point> {
        // theta = adj/hyp
        let x = ((self.iteration as f32) * self.direction.cos()).round() as u32;
        let y = ((self.iteration as f32) * self.direction.sin()).round() as u32;
        let mut new_x = x + self.current.x;
        let mut new_y = y + self.current.y;
        if new_x < 0 || new_y < 0 {
            return None
        }
        let new_point = Point {x: new_x, y: new_y};
        self.current = new_point;
        self.iteration += 1;
        return Some(Point{x: new_x, y: new_y});
    }
}


fn main() {
    println!("Hello, world!");

    if env::args().len() != 3{
        panic!("Please input a file name and a target location")
    }
    let input_path = env::args().nth(1).unwrap();
    let output_folder = env::args().nth(2).unwrap();
    let input_path = Path::new(&input_path);
    let output_folder = Path::new(&output_folder);

    if !output_folder.is_dir(){
        fs::create_dir(output_folder).expect("Failed to create output folder.")
    }    

    if !input_path.is_file(){
        panic!("Input file does not exist")
    }

    // Load the image and cast it to a luma (grayscale)
    let input_image = open(input_path)
        .expect(&format!("Could not load image at {:?}", input_path))
        .to_luma();

    let gray_path = output_folder.join("grey.png");
    input_image.save(&gray_path).unwrap();

    let edges = canny(&input_image, 50., 100.);
    let edge_path = output_folder.join("edges.png");
    edges.save(&edge_path).unwrap();

    let grad_directions = gradient_direction(&input_image);

}


// Compute gradient image and return an angle for each pixel
fn gradient_direction(image: &GrayImage) -> ImageBuffer<Luma<f32>, Vec<f32>>  {
    let mut out = ImageBuffer::from_pixel(image.width(), image.height(), Luma {data: [0.0]});
    let gx = horizontal_sobel(image);
    let gy = vertical_sobel(image);

    for y in 1..out.height() - 1 {
        for x in 1..out.width() - 1 {
        let xgrad = gx[(x, y)][0] as f32;
        let ygrad = gy[(x, y)][0] as f32;
        // Get the gradient direction and store it in the array
        let mut angle = (ygrad).atan2(xgrad);
        assert!((angle <= 2.*f32::consts::PI)  && (angle >= 0.0)); // Just in case we get some weird angles...
        out[(x, y)][0] = angle as f32;
        }
    }
    out
}

fn stroke_width_transform(gradients: &ImageBuffer<Luma<f32>, Vec<f32>>, edges: &GrayImage) -> ImageBuffer<Luma<f32>, Vec<f32>> {
    let mut swt = ImageBuffer::from_pixel(edges.width(), edges.height(), Luma {data:[0.0]});
    // Iterate over all pixels in gradients/edges
    // For each pixel, create a ray that starts at the pixel and traverses along the gradient
    // When another edge pixel is located, compute the stroke width.
    // Associate original and final pixel both with a stroke width value
    for x in 0..swt.width() {
        for y in 0..swt.height() {
            let current_pixel = swt[(x,y)][0] as f32;
            // Check if the swt element has already been associated to a width
            if current_pixel >= 0.0 || edges[(x,y)][0] < 255 {
                continue;
            }
            let direction = gradients[(x,y)][0];
            let ray = Ray::new(x, y, direction, swt.width(), swt.height());
            // for element in ray {

            // }
        }
    }
    swt
}

// Traverse edges in canny image

// Look up the gradient at each non-zero edge pixel

// Traverse along the resultant ray until a new edge is found. 
//  Check new edge angle (must be approx 180*  from original angle)
// If no match, mark this pixel as not part of a uniform stroke. We can probably not disassociate middle pixels as they will
//  implicitly be left out if they don't map to anything by the end.

// Record stroke width. 
    // Need to associate all traversed pixels to this same stroke width
    // HOW TO LINK PIXELS? SEE PAPER or Cpp impl
    // How to store all associated sets of pixels???
    //      At the very least, need to store (x,y, stroke_width). Maybe also info as to what this pixel was connected with if it isn't an edge?
    //      Need to make sure there are no duplicate pixeles?
    //          WHAT IF A PIXEL IS ASSOCIATED AS PART OF TWO STROKE-WIDTHS? NON MAXIMAL SUPPRESSION REQUIRED???


// Group pixels with similar X,Y locations and similar stroke widths. This could be complicated. See paper
//      Probably need some sort of chain-based clustering?

// Drop groups with small numbers of pixels or massive numbers of pixels.

