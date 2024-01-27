extern crate image;
use std::env;
use rayon::prelude::*;
use std::time::Instant;

use image::{GenericImageView, DynamicImage, imageops};
use image::Rgba;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Running an example: {}", args[1]);
    if args.len() < 2{
        println!("Enter an example....");
        std::process::exit(1);
    }
    let mut final_image = String::from("1");
    if args[1] == "1" || args[1] == "1-1"{
        final_image = String::from("1");
    }else if args[1] == "2" || args[1] == "2-1" {
        final_image = String::from("2");
    }else if args[1] == "3"{
        final_image = String::from("3");
    }else if args[1] == "4"{
        final_image = String::from("4");
    }else if args[1] == "5"{
        final_image = String::from("5");
    }

    let start_time = Instant::now();

    let input_image = image::open(format!("../examples/picture{}.jpg",final_image)).expect("Failed to load the input image.");
    let images = get_images(&args[1]);
    let mut img_for_share = images.clone();
    println!("Length {} images.",images.len());

    let (final_width, final_height) = input_image.dimensions();
    let mut assembled_image = DynamicImage::ImageRgba8(image::ImageBuffer::new(final_width, final_height));
    let mut x_offset = 0;
    let mut y_offset = 0;
    for img in &images {
        if img.dimensions().0 == 1 && img.dimensions().1 ==1 {
            img_for_share.retain(|i| i != img);
            continue;
        }
        let img = find_best_solution(&input_image,x_offset,y_offset, &img_for_share.clone());
        imageops::overlay(&mut assembled_image, &img, x_offset, y_offset);
        img_for_share.retain(|i| i != &img);
        if x_offset + img.dimensions().0 <= final_width - 10{
            x_offset += img.dimensions().0; 
        }else{
            if y_offset + img.dimensions().1 <= final_height - 10{
                y_offset += img.dimensions().1;
                x_offset = 0;
            }else{
                x_offset = 0;
                y_offset = final_height - img.dimensions().1;
            }
        }
    }
    let elapsed_time = start_time.elapsed();
    println!("Time taken: {:?}", elapsed_time);

    if let Err(err) = assembled_image.save("./result.png") {
        eprintln!("Error saving assembled image: {}", err);
    }
    
}

fn find_best_solution(init_image: &DynamicImage, x_offset: u32, y_offset: u32, images: &Vec<DynamicImage>) -> DynamicImage {
    let (best_solution, _) = images.par_iter()  // Use par_iter() for parallel iteration
        .map(|img| {
            let diff = calculate_total_diff(&init_image, x_offset, y_offset, &img);
            (img, diff)
        })
        .reduce(|| (&images[0], f32::INFINITY), |acc, x| {
            if x.1 < acc.1 {
                (x.0, x.1)
            } else {
                acc
            }
        });

    best_solution.clone()
}

fn calculate_total_diff(init_image: &DynamicImage, x_offset: u32, y_offset: u32, target_image: &DynamicImage) -> f32 {
    let total_sum: f32 = (0..target_image.dimensions().1)
        .into_par_iter()  // Parallelize the outer loop
        .map(|y| {
            (0..target_image.dimensions().0)
                .map(|x| {
                    euclidean_distance(
                        &init_image.get_pixel(x_offset + x, y_offset + y),
                        &target_image.get_pixel(x, y),
                    )
                })
                .sum::<f32>()
        })
        .sum();

    total_sum
}

fn euclidean_distance(pixel1: &Rgba<u8>, pixel2: &Rgba<u8>) -> f32 {
    ((pixel1[0] as f32 - pixel2[0] as f32).powi(2)
        + (pixel1[1] as f32 - pixel2[1] as f32).powi(2)
        + (pixel1[2] as f32 - pixel2[2] as f32).powi(2)
        + (pixel1[3] as f32 - pixel2[3] as f32).powi(2))
        .sqrt()
}


fn get_images(str:&str) -> Vec<DynamicImage>{
    let mut image_paths:Vec<&str> = Vec::new();
    match str{
        "1" => image_paths = vec!["../examples/slika_1/part a.png", "../examples/slika_1/part b.png", "../examples/slika_1/part c.png","../examples/slika_1/part d.png", "../examples/slika_1/part e.png", "../examples/slika_1/part f.png","../examples/slika_1/part g.png", "../examples/slika_1/part h.png", "../examples/slika_1/part w.png","../examples/slika_1/part x.png", "../examples/slika_1/part y.png", "../examples/slika_1/part z.png"],
        "1-1" => image_paths = vec!["../examples/slika_3/image4x8.png","../examples/slika_3/image3x12.png","../examples/slika_3/image8x29.png","../examples/slika_3/image8x17.png","../examples/slika_3/image7x2.png","../examples/slika_3/image7x24.png","../examples/slika_3/image1x17.png","../examples/slika_3/image7x6.png","../examples/slika_3/image10x25.png","../examples/slika_3/image12x15.png","../examples/slika_3/image6x13.png","../examples/slika_3/image9x12.png","../examples/slika_3/image8x19.png","../examples/slika_3/image10x4.png","../examples/slika_3/image1x7.png","../examples/slika_3/image9x28.png","../examples/slika_3/image2x2.png","../examples/slika_3/image11x21.png","../examples/slika_3/image1x1.png","../examples/slika_3/image3x24.png","../examples/slika_3/image8x22.png","../examples/slika_3/image7x10.png","../examples/slika_3/image3x8.png","../examples/slika_3/image8x7.png","../examples/slika_3/image9x6.png","../examples/slika_3/image6x4.png","../examples/slika_3/image11x25.png","../examples/slika_3/image3x11.png","../examples/slika_3/image10x24.png","../examples/slika_3/image1x15.png","../examples/slika_3/image5x24.png","../examples/slika_3/image11x24.png","../examples/slika_3/image9x23.png","../examples/slika_3/image2x5.png","../examples/slika_3/image5x5.png","../examples/slika_3/image8x16.png","../examples/slika_3/image12x18.png","../examples/slika_3/image7x27.png","../examples/slika_3/image9x1.png","../examples/slika_3/image12x28.png","../examples/slika_3/image3x1.png","../examples/slika_3/image2x29.png","../examples/slika_3/image7x21.png","../examples/slika_3/image8x21.png","../examples/slika_3/image5x9.png","../examples/slika_3/image2x21.png","../examples/slika_3/image6x14.png","../examples/slika_3/image12x25.png","../examples/slika_3/image4x2.png","../examples/slika_3/image4x9.png","../examples/slika_3/image3x20.png","../examples/slika_3/image1x9.png","../examples/slika_3/image4x15.png","../examples/slika_3/image9x3.png","../examples/slika_3/image6x29.png","../examples/slika_3/image8x8.png","../examples/slika_3/image3x29.png","../examples/slika_3/image10x7.png","../examples/slika_3/image1x2.png","../examples/slika_3/image10x13.png","../examples/slika_3/image5x8.png","../examples/slika_3/image5x6.png","../examples/slika_3/image3x10.png","../examples/slika_3/image3x7.png","../examples/slika_3/image7x28.png","../examples/slika_3/image4x3.png","../examples/slika_3/image7x9.png","../examples/slika_3/image11x29.png","../examples/slika_3/image2x16.png","../examples/slika_3/image11x12.png","../examples/slika_3/image4x17.png","../examples/slika_3/image4x16.png","../examples/slika_3/image7x20.png","../examples/slika_3/image1x20.png","../examples/slika_3/image3x5.png","../examples/slika_3/image5x25.png","../examples/slika_3/image1x10.png","../examples/slika_3/image7x8.png","../examples/slika_3/image8x25.png","../examples/slika_3/image2x1.png","../examples/slika_3/image3x6.png","../examples/slika_3/image2x20.png","../examples/slika_3/image2x22.png","../examples/slika_3/image4x10.png","../examples/slika_3/image11x16.png","../examples/slika_3/image12x10.png","../examples/slika_3/image11x9.png","../examples/slika_3/image4x19.png","../examples/slika_3/image5x7.png","../examples/slika_3/image9x18.png","../examples/slika_3/image3x21.png","../examples/slika_3/image9x8.png","../examples/slika_3/image3x17.png","../examples/slika_3/image6x20.png","../examples/slika_3/image12x26.png","../examples/slika_3/image2x3.png","../examples/slika_3/image1x27.png","../examples/slika_3/image7x12.png","../examples/slika_3/image4x18.png","../examples/slika_3/image5x10.png","../examples/slika_3/image2x10.png","../examples/slika_3/image9x7.png","../examples/slika_3/image9x13.png","../examples/slika_3/image10x16.png","../examples/slika_3/image6x22.png","../examples/slika_3/image1x24.png","../examples/slika_3/image8x12.png","../examples/slika_3/image11x2.png","../examples/slika_3/image6x10.png","../examples/slika_3/image5x21.png","../examples/slika_3/image3x18.png","../examples/slika_3/image5x2.png","../examples/slika_3/image6x5.png","../examples/slika_3/image5x27.png","../examples/slika_3/image10x17.png","../examples/slika_3/image7x29.png","../examples/slika_3/image12x8.png","../examples/slika_3/image9x24.png","../examples/slika_3/image10x22.png","../examples/slika_3/image12x13.png","../examples/slika_3/image8x4.png","../examples/slika_3/image10x10.png","../examples/slika_3/image1x22.png","../examples/slika_3/image6x1.png","../examples/slika_3/image7x18.png","../examples/slika_3/image5x13.png","../examples/slika_3/image5x15.png","../examples/slika_3/image12x3.png","../examples/slika_3/image1x14.png","../examples/slika_3/image6x3.png","../examples/slika_3/image3x28.png","../examples/slika_3/image2x28.png","../examples/slika_3/image12x6.png","../examples/slika_3/image4x5.png","../examples/slika_3/image7x5.png","../examples/slika_3/image10x26.png","../examples/slika_3/image9x11.png","../examples/slika_3/image3x13.png","../examples/slika_3/image10x9.png","../examples/slika_3/image10x6.png","../examples/slika_3/image3x23.png","../examples/slika_3/image11x11.png","../examples/slika_3/image11x7.png","../examples/slika_3/image10x8.png","../examples/slika_3/image1x13.png","../examples/slika_3/image6x6.png","../examples/slika_3/image5x17.png","../examples/slika_3/image6x21.png","../examples/slika_3/image6x26.png","../examples/slika_3/image9x26.png","../examples/slika_3/image9x22.png","../examples/slika_3/image1x19.png","../examples/slika_3/image2x9.png","../examples/slika_3/image11x28.png","../examples/slika_3/image1x21.png","../examples/slika_3/image9x9.png","../examples/slika_3/image8x3.png","../examples/slika_3/image8x26.png","../examples/slika_3/image4x26.png","../examples/slika_3/image10x11.png","../examples/slika_3/image4x11.png","../examples/slika_3/image4x14.png","../examples/slika_3/image11x13.png","../examples/slika_3/image12x9.png","../examples/slika_3/image10x21.png","../examples/slika_3/image10x27.png","../examples/slika_3/image12x11.png","../examples/slika_3/image5x14.png","../examples/slika_3/image9x5.png","../examples/slika_3/image9x15.png","../examples/slika_3/image10x2.png","../examples/slika_3/image2x7.png","../examples/slika_3/image8x23.png","../examples/slika_3/image11x10.png","../examples/slika_3/image1x3.png","../examples/slika_3/image11x6.png","../examples/slika_3/image12x7.png","../examples/slika_3/image8x14.png","../examples/slika_3/image1x11.png","../examples/slika_3/image10x3.png","../examples/slika_3/image12x24.png","../examples/slika_3/image10x23.png","../examples/slika_3/image6x23.png","../examples/slika_3/image7x3.png","../examples/slika_3/image11x5.png","../examples/slika_3/image7x16.png","../examples/slika_3/image9x16.png","../examples/slika_3/image6x25.png","../examples/slika_3/image1x26.png","../examples/slika_3/image12x1.png","../examples/slika_3/image1x29.png","../examples/slika_3/image8x6.png","../examples/slika_3/image7x26.png","../examples/slika_3/image7x23.png","../examples/slika_3/image2x19.png","../examples/slika_3/image7x19.png","../examples/slika_3/image2x25.png","../examples/slika_3/image5x20.png","../examples/slika_3/image8x28.png","../examples/slika_3/image12x14.png","../examples/slika_3/image2x14.png","../examples/slika_3/image5x28.png","../examples/slika_3/image7x25.png","../examples/slika_3/image4x20.png","../examples/slika_3/image12x5.png","../examples/slika_3/image3x25.png","../examples/slika_3/image11x17.png","../examples/slika_3/image6x9.png","../examples/slika_3/image9x25.png","../examples/slika_3/image6x11.png","../examples/slika_3/image8x9.png","../examples/slika_3/image5x3.png","../examples/slika_3/image3x4.png","../examples/slika_3/image8x11.png","../examples/slika_3/image2x15.png","../examples/slika_3/image4x1.png","../examples/slika_3/image12x17.png","../examples/slika_3/image11x18.png","../examples/slika_3/image6x7.png","../examples/slika_3/image5x1.png","../examples/slika_3/image8x18.png","../examples/slika_3/image3x27.png","../examples/slika_3/image12x4.png","../examples/slika_3/image3x15.png","../examples/slika_3/image10x28.png","../examples/slika_3/image11x20.png","../examples/slika_3/image9x4.png","../examples/slika_3/image8x13.png","../examples/slika_3/image2x6.png","../examples/slika_3/image9x19.png","../examples/slika_3/image5x19.png","../examples/slika_3/image10x19.png","../examples/slika_3/image4x28.png","../examples/slika_3/image2x8.png","../examples/slika_3/image3x3.png","../examples/slika_3/image10x15.png","../examples/slika_3/image4x24.png","../examples/slika_3/image7x1.png","../examples/slika_3/image9x10.png","../examples/slika_3/image9x27.png","../examples/slika_3/image11x4.png","../examples/slika_3/image5x11.png","../examples/slika_3/image2x12.png","../examples/slika_3/image2x26.png","../examples/slika_3/image4x25.png","../examples/slika_3/image9x14.png","../examples/slika_3/image11x3.png","../examples/slika_3/image11x23.png","../examples/slika_3/image3x14.png","../examples/slika_3/image6x2.png","../examples/slika_3/image6x17.png","../examples/slika_3/image2x11.png","../examples/slika_3/image3x2.png","../examples/slika_3/image8x27.png","../examples/slika_3/image11x8.png","../examples/slika_3/image8x2.png","../examples/slika_3/image11x15.png","../examples/slika_3/image7x4.png","../examples/slika_3/image7x13.png","../examples/slika_3/image12x21.png","../examples/slika_3/image10x20.png","../examples/slika_3/image1x25.png","../examples/slika_3/image5x23.png","../examples/slika_3/image4x12.png","../examples/slika_3/image4x22.png","../examples/slika_3/image12x2.png","../examples/slika_3/image4x6.png","../examples/slika_3/image1x23.png","../examples/slika_3/image5x4.png","../examples/slika_3/image10x29.png","../examples/slika_3/image9x29.png","../examples/slika_3/image11x14.png","../examples/slika_3/image11x26.png","../examples/slika_3/image2x23.png","../examples/slika_3/image8x20.png","../examples/slika_3/image6x28.png","../examples/slika_3/image10x5.png","../examples/slika_3/image6x8.png","../examples/slika_3/image4x21.png","../examples/slika_3/image8x1.png","../examples/slika_3/image1x4.png","../examples/slika_3/image5x26.png","../examples/slika_3/image6x27.png","../examples/slika_3/image11x19.png","../examples/slika_3/image8x10.png","../examples/slika_3/image6x15.png","../examples/slika_3/image8x15.png","../examples/slika_3/image1x5.png","../examples/slika_3/image5x12.png","../examples/slika_3/image3x16.png","../examples/slika_3/image12x16.png","../examples/slika_3/image10x18.png","../examples/slika_3/image8x24.png","../examples/slika_3/image6x19.png","../examples/slika_3/image5x22.png","../examples/slika_3/image4x4.png","../examples/slika_3/image2x24.png","../examples/slika_3/image4x29.png","../examples/slika_3/image2x17.png","../examples/slika_3/image3x22.png","../examples/slika_3/image9x2.png","../examples/slika_3/image11x27.png","../examples/slika_3/image12x12.png","../examples/slika_3/image12x22.png","../examples/slika_3/image9x20.png","../examples/slika_3/image3x19.png","../examples/slika_3/image4x13.png","../examples/slika_3/image7x22.png","../examples/slika_3/image1x12.png","../examples/slika_3/image4x23.png","../examples/slika_3/image3x26.png","../examples/slika_3/image3x9.png","../examples/slika_3/image5x16.png","../examples/slika_3/image4x27.png","../examples/slika_3/image7x17.png","../examples/slika_3/image7x11.png","../examples/slika_3/image1x16.png","../examples/slika_3/image4x7.png","../examples/slika_3/image7x14.png","../examples/slika_3/image1x18.png","../examples/slika_3/image10x14.png","../examples/slika_3/image9x21.png","../examples/slika_3/image2x18.png","../examples/slika_3/image2x13.png","../examples/slika_3/image6x16.png","../examples/slika_3/image6x18.png","../examples/slika_3/image1x6.png","../examples/slika_3/image11x22.png","../examples/slika_3/image8x5.png","../examples/slika_3/image12x29.png","../examples/slika_3/image10x12.png","../examples/slika_3/image5x29.png","../examples/slika_3/image9x17.png","../examples/slika_3/image2x4.png","../examples/slika_3/image12x19.png","../examples/slika_3/image11x1.png","../examples/slika_3/image10x1.png","../examples/slika_3/image6x12.png","../examples/slika_3/image7x7.png","../examples/slika_3/image12x20.png","../examples/slika_3/image1x8.png","../examples/slika_3/image7x15.png","../examples/slika_3/image5x18.png","../examples/slika_3/image12x23.png","../examples/slika_3/image12x27.png","../examples/slika_3/image6x24.png","../examples/slika_3/image1x28.png","../examples/slika_3/image2x27.png"],
        "2" => image_paths = vec!["../examples/slika_2/image1x2.png", "../examples/slika_2/image1x3.png", "../examples/slika_2/image1x5.png","../examples/slika_2/image1x9.png", "../examples/slika_2/image1x10.png", "../examples/slika_2/image1x11.png","../examples/slika_2/image1x12.png", "../examples/slika_2/image1x13.png", "../examples/slika_2/pok7.png","../examples/slika_2/pok34.png", "../examples/slika_2/pok 1.png", "../examples/slika_2/pok 4.png", "../examples/slika_2/pok 10.png", "../examples/slika_2/pok 45.png", "../examples/slika_2/pok a.png"],
        "4" => image_paths = vec!["../examples/slika_4/img 78.png","../examples/slika_4/img l.png","../examples/slika_4/image9x1.png","../examples/slika_4/image3x1.png","../examples/slika_4/image13x1.png","../examples/slika_4/img a.png","../examples/slika_4/img mn.png","../examples/slika_4/img k.png","../examples/slika_4/img 7.png","../examples/slika_4/image14x1.png","../examples/slika_4/img 23.png","../examples/slika_4/img 4.png","../examples/slika_4/image8x1.png","../examples/slika_4/img 8.png","../examples/slika_4/img 89.png"],
        "5" => image_paths = vec!["../examples/slika_5/img ab (13).png",
        "../examples/slika_5/img 7 (15).png",
        "../examples/slika_5/img 7 (35).png",
        "../examples/slika_5/26.png",
        "../examples/slika_5/img 7 (8).png",
        "../examples/slika_5/img 7 (4).png",
        "../examples/slika_5/img ab (25).png",
        "../examples/slika_5/img 7 (10).png",
        "../examples/slika_5/55.png",
        "../examples/slika_5/img ab (6).png",
        "../examples/slika_5/img 7 (21).png",
        "../examples/slika_5/28.png",
        "../examples/slika_5/46.png",
        "../examples/slika_5/50.png",
        "../examples/slika_5/51.png",
        "../examples/slika_5/img 7 (6).png",
        "../examples/slika_5/img ab (9).png",
        "../examples/slika_5/img 7 (34).png",
        "../examples/slika_5/img ab (23).png",
        "../examples/slika_5/39.png",
        "../examples/slika_5/32.png",
        "../examples/slika_5/img 7 (16).png",
        "../examples/slika_5/img 7 (22).png",
        "../examples/slika_5/42.png",
        "../examples/slika_5/57.png",
        "../examples/slika_5/40.png",
        "../examples/slika_5/33.png",
        "../examples/slika_5/41.png",
        "../examples/slika_5/27.png",
        "../examples/slika_5/img 7 (17).png",
        "../examples/slika_5/img 7 (30).png",
        "../examples/slika_5/47.png",
        "../examples/slika_5/img ab (20).png",
        "../examples/slika_5/img 7 (18).png",
        "../examples/slika_5/img ab (12).png",
        "../examples/slika_5/img 7 (14).png",
        "../examples/slika_5/34.png",
        "../examples/slika_5/53.png",
        "../examples/slika_5/img ab (22).png",
        "../examples/slika_5/img 7 (28).png",
        "../examples/slika_5/img 7 (3).png",
        "../examples/slika_5/img 7 (7).png",
        "../examples/slika_5/29.png",
        "../examples/slika_5/img 7 (31).png",
        "../examples/slika_5/img 7 (20).png",
        "../examples/slika_5/img ab (11).png",
        "../examples/slika_5/37.png",
        "../examples/slika_5/img ab (1).png",
        "../examples/slika_5/img 7 (12).png",
        "../examples/slika_5/54.png",
        "../examples/slika_5/img 7 (27).png",
        "../examples/slika_5/img ab (16).png",
        "../examples/slika_5/52.png",
        "../examples/slika_5/img ab (10).png",
        "../examples/slika_5/img 7 (33).png",
        "../examples/slika_5/img ab (8).png",
        "../examples/slika_5/img ab (19).png",
        "../examples/slika_5/35.png",
        "../examples/slika_5/img 7 (29).png",
        "../examples/slika_5/45.png",
        "../examples/slika_5/img 7 (1).png",
        "../examples/slika_5/img ab (3).png",
        "../examples/slika_5/30.png",
        "../examples/slika_5/img 7 (19).png",
        "../examples/slika_5/38.png",
        "../examples/slika_5/img 7 (36).png",
        "../examples/slika_5/60.png",
        "../examples/slika_5/31.png",
        "../examples/slika_5/img ab (18).png",
        "../examples/slika_5/59.png",
        "../examples/slika_5/img 7 (24).png",
        "../examples/slika_5/img 7 (26).png",
        "../examples/slika_5/img 7 (11).png",
        "../examples/slika_5/img ab (4).png",
        "../examples/slika_5/44.png",
        "../examples/slika_5/img ab (15).png",
        "../examples/slika_5/43.png",
        "../examples/slika_5/58.png",
        "../examples/slika_5/img ab (24).png",
        "../examples/slika_5/img ab (2).png",
        "../examples/slika_5/49.png",
        "../examples/slika_5/img 7 (9).png",
        "../examples/slika_5/img 7 (23).png",
        "../examples/slika_5/36.png",
        "../examples/slika_5/48.png",
        "../examples/slika_5/img ab (21).png",
        "../examples/slika_5/img ab (7).png",
        "../examples/slika_5/img 7 (13).png",
        "../examples/slika_5/img 7 (5).png",
        "../examples/slika_5/img 7 (25).png",
        "../examples/slika_5/img 7 (32).png",
        "../examples/slika_5/56.png",
        "../examples/slika_5/img 7 (2).png",
        "../examples/slika_5/img ab (17).png",
        "../examples/slika_5/img ab (5).png",
        "../examples/slika_5/img ab (14).png"
        ],
        "2-1" => image_paths = vec!["../examples/slika_6/62.png",
        "../examples/slika_6/part a.png",
        "../examples/slika_6/96.png",
        "../examples/slika_6/153.png",
        "../examples/slika_6/180.png",
        "../examples/slika_6/103.png",
        "../examples/slika_6/86.png",
        "../examples/slika_6/245.png",
        "../examples/slika_6/05.png",
        "../examples/slika_6/225.png",
        "../examples/slika_6/192.png",
        "../examples/slika_6/part ch.png",
        "../examples/slika_6/66.png",
        "../examples/slika_6/26.png",
        "../examples/slika_6/250.png",
        "../examples/slika_6/95.png",
        "../examples/slika_6/21.png",
        "../examples/slika_6/272.png",
        "../examples/slika_6/65.png",
        "../examples/slika_6/part yx.png",
        "../examples/slika_6/173.png",
        "../examples/slika_6/251.png",
        "../examples/slika_6/125.png",
        "../examples/slika_6/186.png",
        "../examples/slika_6/220.png",
        "../examples/slika_6/10.png",
        "../examples/slika_6/100.png",
        "../examples/slika_6/55.png",
        "../examples/slika_6/02.png",
        "../examples/slika_6/22.png",
        "../examples/slika_6/143.png",
        "../examples/slika_6/117.png",
        "../examples/slika_6/224.png",
        "../examples/slika_6/17.png",
        "../examples/slika_6/212.png",
        "../examples/slika_6/46.png",
        "../examples/slika_6/151.png",
        "../examples/slika_6/247.png",
        "../examples/slika_6/50.png",
        "../examples/slika_6/164.png",
        "../examples/slika_6/51.png",
        "../examples/slika_6/77.png",
        "../examples/slika_6/204.png",
        "../examples/slika_6/102.png",
        "../examples/slika_6/133.png",
        "../examples/slika_6/177.png",
        "../examples/slika_6/91.png",
        "../examples/slika_6/221.png",
        "../examples/slika_6/231.png",
        "../examples/slika_6/237.png",
        "../examples/slika_6/61.png",
        "../examples/slika_6/04.png",
        "../examples/slika_6/126.png",
        "../examples/slika_6/32.png",
        "../examples/slika_6/part d.png",
        "../examples/slika_6/274.png",
        "../examples/slika_6/121.png",
        "../examples/slika_6/12.png",
        "../examples/slika_6/81.png",
        "../examples/slika_6/235.png",
        "../examples/slika_6/71.png",
        "../examples/slika_6/176.png",
        "../examples/slika_6/137.png",
        "../examples/slika_6/194.png",
        "../examples/slika_6/270.png",
        "../examples/slika_6/63.png",
        "../examples/slika_6/115.png",
        "../examples/slika_6/207.png",
        "../examples/slika_6/127.png",
        "../examples/slika_6/174.png",
        "../examples/slika_6/42.png",
        "../examples/slika_6/160.png",
        "../examples/slika_6/131.png",
        "../examples/slika_6/57.png",
        "../examples/slika_6/13.png",
        "../examples/slika_6/87.png",
        "../examples/slika_6/40.png",
        "../examples/slika_6/33.png",
        "../examples/slika_6/41.png",
        "../examples/slika_6/27.png",
        "../examples/slika_6/97.png",
        "../examples/slika_6/276.png",
        "../examples/slika_6/136.png",
        "../examples/slika_6/01.png",
        "../examples/slika_6/213.png",
        "../examples/slika_6/110.png",
        "../examples/slika_6/242.png",
        "../examples/slika_6/263.png",
        "../examples/slika_6/141.png",
        "../examples/slika_6/47.png",
        "../examples/slika_6/246.png",
        "../examples/slika_6/67.png",
        "../examples/slika_6/142.png",
        "../examples/slika_6/277.png",
        "../examples/slika_6/236.png",
        "../examples/slika_6/167.png",
        "../examples/slika_6/64.png",
        "../examples/slika_6/94.png",
        "../examples/slika_6/34.png",
        "../examples/slika_6/104.png",
        "../examples/slika_6/53.png",
        "../examples/slika_6/116.png",
        "../examples/slika_6/230.png",
        "../examples/slika_6/part 89.png",
        "../examples/slika_6/166.png",
        "../examples/slika_6/203.png",
        "../examples/slika_6/130.png",
        "../examples/slika_6/92.png",
        "../examples/slika_6/254.png",
        "../examples/slika_6/24.png",
        "../examples/slika_6/165.png",
        "../examples/slika_6/193.png",
        "../examples/slika_6/114.png",
        "../examples/slika_6/267.png",
        "../examples/slika_6/82.png",
        "../examples/slika_6/84.png",
        "../examples/slika_6/262.png",
        "../examples/slika_6/211.png",
        "../examples/slika_6/233.png",
        "../examples/slika_6/113.png",
        "../examples/slika_6/202.png",
        "../examples/slika_6/37.png",
        "../examples/slika_6/06.png",
        "../examples/slika_6/11.png",
        "../examples/slika_6/157.png",
        "../examples/slika_6/226.png",
        "../examples/slika_6/152.png",
        "../examples/slika_6/16.png",
        "../examples/slika_6/part m.png",
        "../examples/slika_6/54.png",
        "../examples/slika_6/155.png",
        "../examples/slika_6/140.png",
        "../examples/slika_6/240.png",
        "../examples/slika_6/07.png",
        "../examples/slika_6/25.png",
        "../examples/slika_6/73.png",
        "../examples/slika_6/part z.png",
        "../examples/slika_6/76.png",
        "../examples/slika_6/85.png",
        "../examples/slika_6/124.png",
        "../examples/slika_6/111.png",
        "../examples/slika_6/162.png",
        "../examples/slika_6/52.png",
        "../examples/slika_6/227.png",
        "../examples/slika_6/35.png",
        "../examples/slika_6/134.png",
        "../examples/slika_6/184.png",
        "../examples/slika_6/234.png",
        "../examples/slika_6/200.png",
        "../examples/slika_6/75.png",
        "../examples/slika_6/181.png",
        "../examples/slika_6/232.png",
        "../examples/slika_6/216.png",
        "../examples/slika_6/205.png",
        "../examples/slika_6/201.png",
        "../examples/slika_6/45.png",
        "../examples/slika_6/256.png",
        "../examples/slika_6/146.png",
        "../examples/slika_6/30.png",
        "../examples/slika_6/93.png",
        "../examples/slika_6/172.png",
        "../examples/slika_6/190.png",
        "../examples/slika_6/70.png",
        "../examples/slika_6/214.png",
        "../examples/slika_6/217.png",
        "../examples/slika_6/210.png",
        "../examples/slika_6/223.png",
        "../examples/slika_6/195.png",
        "../examples/slika_6/163.png",
        "../examples/slika_6/257.png",
        "../examples/slika_6/60.png",
        "../examples/slika_6/266.png",
        "../examples/slika_6/185.png",
        "../examples/slika_6/31.png",
        "../examples/slika_6/255.png",
        "../examples/slika_6/150.png",
        "../examples/slika_6/part t.png",
        "../examples/slika_6/271.png",
        "../examples/slika_6/264.png",
        "../examples/slika_6/147.png",
        "../examples/slika_6/183.png",
        "../examples/slika_6/15.png",
        "../examples/slika_6/253.png",
        "../examples/slika_6/171.png",
        "../examples/slika_6/275.png",
        "../examples/slika_6/197.png",
        "../examples/slika_6/part x.png",
        "../examples/slika_6/23.png",
        "../examples/slika_6/182.png",
        "../examples/slika_6/196.png",
        "../examples/slika_6/215.png",
        "../examples/slika_6/132.png",
        "../examples/slika_6/44.png",
        "../examples/slika_6/43.png",
        "../examples/slika_6/72.png",
        "../examples/slika_6/144.png",
        "../examples/slika_6/135.png",
        "../examples/slika_6/252.png",
        "../examples/slika_6/206.png",
        "../examples/slika_6/156.png",
        "../examples/slika_6/222.png",
        "../examples/slika_6/101.png",
        "../examples/slika_6/241.png",
        "../examples/slika_6/36.png",
        "../examples/slika_6/20.png",
        "../examples/slika_6/243.png",
        "../examples/slika_6/260.png",
        "../examples/slika_6/105.png",
        "../examples/slika_6/122.png",
        "../examples/slika_6/261.png",
        "../examples/slika_6/120.png",
        "../examples/slika_6/part nn.png",
        "../examples/slika_6/107.png",
        "../examples/slika_6/56.png",
        "../examples/slika_6/154.png",
        "../examples/slika_6/244.png",
        "../examples/slika_6/161.png",
        "../examples/slika_6/187.png",
        "../examples/slika_6/170.png",
        "../examples/slika_6/106.png",
        "../examples/slika_6/14.png",
        "../examples/slika_6/90.png",
        "../examples/slika_6/175.png",
        "../examples/slika_6/145.png"
        ],
        "3" => image_paths = vec!["../examples/slika_7/img 8 (45).png",
        "../examples/slika_7/img 8 (135).png",
        "../examples/slika_7/img 8 (7).png",
        "../examples/slika_7/img 8 (115).png",
        "../examples/slika_7/img 8 (31).png",
        "../examples/slika_7/img 8 (192).png",
        "../examples/slika_7/img 8 (181).png",
        "../examples/slika_7/img 8 (2).png",
        "../examples/slika_7/img 8 (4).png",
        "../examples/slika_7/img 8 (52).png",
        "../examples/slika_7/img 8 (29).png",
        "../examples/slika_7/img 8 (8).png",
        "../examples/slika_7/img 8 (10).png",
        "../examples/slika_7/img 8 (130).png",
        "../examples/slika_7/img 8 (165).png",
        "../examples/slika_7/img 8 (129).png",
        "../examples/slika_7/img 8 (108).png",
        "../examples/slika_7/img 8 (121).png",
        "../examples/slika_7/img 8 (125).png",
        "../examples/slika_7/img 8 (117).png",
        "../examples/slika_7/img 8 (97).png",
        "../examples/slika_7/img 8 (118).png",
        "../examples/slika_7/img 8 (40).png",
        "../examples/slika_7/img 8 (43).png",
        "../examples/slika_7/img 8 (140).png",
        "../examples/slika_7/img 8 (79).png",
        "../examples/slika_7/img 8 (101).png",
        "../examples/slika_7/img 8 (166).png",
        "../examples/slika_7/img 8 (107).png",
        "../examples/slika_7/img 8 (72).png",
        "../examples/slika_7/img 8 (73).png",
        "../examples/slika_7/img 8 (5).png",
        "../examples/slika_7/img 8 (173).png",
        "../examples/slika_7/img 8 (170).png",
        "../examples/slika_7/img 8 (15).png",
        "../examples/slika_7/img 8 (37).png",
        "../examples/slika_7/img 8 (104).png",
        "../examples/slika_7/img 8 (157).png",
        "../examples/slika_7/img 8 (149).png",
        "../examples/slika_7/img 8 (154).png",
        "../examples/slika_7/img 8 (169).png",
        "../examples/slika_7/img 8 (116).png",
        "../examples/slika_7/img 8 (41).png",
        "../examples/slika_7/img 8 (122).png",
        "../examples/slika_7/img 8 (126).png",
        "../examples/slika_7/img 8 (34).png",
        "../examples/slika_7/img 8 (179).png",
        "../examples/slika_7/img 8 (12).png",
        "../examples/slika_7/img 8 (65).png",
        "../examples/slika_7/img 8 (68).png",
        "../examples/slika_7/img 8 (139).png",
        "../examples/slika_7/img 8 (49).png",
        "../examples/slika_7/img 8 (14).png",
        "../examples/slika_7/img 8 (164).png",
        "../examples/slika_7/img 8 (59).png",
        "../examples/slika_7/img 8 (58).png",
        "../examples/slika_7/img 8 (132).png",
        "../examples/slika_7/img 8 (109).png",
        "../examples/slika_7/img 8 (162).png",
        "../examples/slika_7/img 8 (78).png",
        "../examples/slika_7/img 8 (80).png",
        "../examples/slika_7/img 8 (141).png",
        "../examples/slika_7/img 8 (93).png",
        "../examples/slika_7/img 8 (76).png",
        "../examples/slika_7/img 8 (185).png",
        "../examples/slika_7/img 8 (38).png",
        "../examples/slika_7/img 8 (94).png",
        "../examples/slika_7/img 8 (70).png",
        "../examples/slika_7/img 8 (17).png",
        "../examples/slika_7/img 8 (153).png",
        "../examples/slika_7/img 8 (144).png",
        "../examples/slika_7/img 8 (48).png",
        "../examples/slika_7/img 8 (83).png",
        "../examples/slika_7/img 8 (138).png",
        "../examples/slika_7/img 8 (191).png",
        "../examples/slika_7/img 8 (32).png",
        "../examples/slika_7/img 8 (106).png",
        "../examples/slika_7/img 8 (26).png",
        "../examples/slika_7/img 8 (111).png",
        "../examples/slika_7/img 8 (67).png",
        "../examples/slika_7/img 8 (22).png",
        "../examples/slika_7/img 8 (18).png",
        "../examples/slika_7/img 8 (98).png",
        "../examples/slika_7/img 8 (100).png",
        "../examples/slika_7/img 8 (1).png",
        "../examples/slika_7/img 8 (54).png",
        "../examples/slika_7/img 8 (24).png",
        "../examples/slika_7/img 8 (137).png",
        "../examples/slika_7/img 8 (171).png",
        "../examples/slika_7/img 8 (66).png",
        "../examples/slika_7/img 8 (28).png",
        "../examples/slika_7/img 8 (25).png",
        "../examples/slika_7/img 8 (159).png",
        "../examples/slika_7/img 8 (19).png",
        "../examples/slika_7/img 8 (103).png",
        "../examples/slika_7/img 8 (88).png",
        "../examples/slika_7/img 8 (190).png",
        "../examples/slika_7/img 8 (64).png",
        "../examples/slika_7/img 8 (155).png",
        "../examples/slika_7/img 8 (176).png",
        "../examples/slika_7/img 8 (119).png",
        "../examples/slika_7/img 8 (36).png",
        "../examples/slika_7/img 8 (99).png",
        "../examples/slika_7/img 8 (172).png",
        "../examples/slika_7/img 8 (9).png",
        "../examples/slika_7/img 8 (143).png",
        "../examples/slika_7/img 8 (163).png",
        "../examples/slika_7/img 8 (184).png",
        "../examples/slika_7/img 8 (152).png",
        "../examples/slika_7/image 103 (1).png",
        "../examples/slika_7/img 8 (71).png",
        "../examples/slika_7/img 8 (189).png",
        "../examples/slika_7/img 8 (20).png",
        "../examples/slika_7/img 8 (96).png",
        "../examples/slika_7/img 8 (85).png",
        "../examples/slika_7/img 8 (188).png",
        "../examples/slika_7/img 8 (57).png",
        "../examples/slika_7/img 8 (124).png",
        "../examples/slika_7/img 8 (110).png",
        "../examples/slika_7/img 8 (42).png",
        "../examples/slika_7/img 8 (84).png",
        "../examples/slika_7/img 8 (156).png",
        "../examples/slika_7/img 8 (30).png",
        "../examples/slika_7/img 8 (75).png",
        "../examples/slika_7/img 8 (47).png",
        "../examples/slika_7/img 8 (90).png",
        "../examples/slika_7/img 8 (145).png",
        "../examples/slika_7/img 8 (160).png",
        "../examples/slika_7/img 8 (161).png",
        "../examples/slika_7/img 8 (168).png",
        "../examples/slika_7/img 8 (113).png",
        "../examples/slika_7/img 8 (187).png",
        "../examples/slika_7/img 8 (50).png",
        "../examples/slika_7/img 8 (39).png",
        "../examples/slika_7/img 8 (3).png",
        "../examples/slika_7/img 8 (21).png",
        "../examples/slika_7/img 8 (120).png",
        "../examples/slika_7/img 8 (77).png",
        "../examples/slika_7/img 8 (105).png",
        "../examples/slika_7/img 8 (35).png",
        "../examples/slika_7/img 8 (134).png",
        "../examples/slika_7/img 8 (62).png",
        "../examples/slika_7/img 8 (174).png",
        "../examples/slika_7/img 8 (158).png",
        "../examples/slika_7/img 8 (147).png",
        "../examples/slika_7/img 8 (175).png",
        "../examples/slika_7/img 8 (74).png",
        "../examples/slika_7/img 8 (182).png",
        "../examples/slika_7/img 8 (92).png",
        "../examples/slika_7/img 8 (53).png",
        "../examples/slika_7/img 8 (150).png",
        "../examples/slika_7/img 8 (151).png",
        "../examples/slika_7/img 8 (95).png",
        "../examples/slika_7/img 8 (167).png",
        "../examples/slika_7/img 8 (183).png",
        "../examples/slika_7/img 8 (87).png",
        "../examples/slika_7/img 8 (102).png",
        "../examples/slika_7/img 8 (56).png",
        "../examples/slika_7/img 8 (180).png",
        "../examples/slika_7/img 8 (114).png",
        "../examples/slika_7/img 8 (136).png",
        "../examples/slika_7/img 8 (51).png",
        "../examples/slika_7/img 8 (44).png",
        "../examples/slika_7/img 8 (6).png",
        "../examples/slika_7/img 8 (131).png",
        "../examples/slika_7/img 8 (81).png",
        "../examples/slika_7/img 8 (177).png",
        "../examples/slika_7/img 8 (133).png",
        "../examples/slika_7/img 8 (91).png",
        "../examples/slika_7/img 8 (86).png",
        "../examples/slika_7/img 8 (69).png",
        "../examples/slika_7/img 8 (142).png",
        "../examples/slika_7/img 8 (127).png",
        "../examples/slika_7/img 8 (146).png",
        "../examples/slika_7/img 8 (178).png",
        "../examples/slika_7/img 8 (61).png",
        "../examples/slika_7/img 8 (186).png",
        "../examples/slika_7/img 8 (128).png",
        "../examples/slika_7/img 8 (82).png",
        "../examples/slika_7/image 103 (2).png",
        "../examples/slika_7/img 8 (55).png",
        "../examples/slika_7/img 8 (11).png",
        "../examples/slika_7/img 8 (46).png",
        "../examples/slika_7/img 8 (63).png",
        "../examples/slika_7/img 8 (16).png",
        "../examples/slika_7/img 8 (27).png",
        "../examples/slika_7/img 8 (112).png",
        "../examples/slika_7/img 8 (60).png",
        "../examples/slika_7/img 8 (33).png",
        "../examples/slika_7/img 8 (89).png",
        "../examples/slika_7/img 8 (123).png",
        "../examples/slika_7/img 8 (148).png"
        ],
        _ => println!("Wrong input for load images."),
    }
    // let start_time = Instant::now();
    let images: Vec<DynamicImage> = image_paths
        .par_iter()
        .filter_map(|&path| {
            match image::open(path) {
                Ok(img) => Some(img),
                Err(_) => {
                    eprintln!("Error loading image: {}", path);
                    None
                }
            }
        })
        .collect();
    // let elapsed_time = start_time.elapsed();
    // println!("Time taken: {:?}", elapsed_time);
    if images.is_empty() {
        eprintln!("No valid images found.");
    }
    images
}
