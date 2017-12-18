//! Demo of SWT and scratchpad for learning rust
extern crate image;
extern crate imageproc;


use std::env;
use std::path::Path;
use std::fs;
use image::{open};
use imageproc::edges::canny;


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
}


// Compute gradient image

// Traver edges in canny image

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

