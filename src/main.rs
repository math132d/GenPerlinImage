extern crate clap;
extern crate rust_perlin;

use std::thread::{self, JoinHandle};
use std::sync::Arc;

use std::process;
use std::time::Instant;
use std::path::Path;

use rust_perlin::Perlin2D;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Perlin Generator")
                        .version("1.0")
                        .arg(Arg::with_name("Width")
                            .index(1)
                            .required(true))
                        .arg(Arg::with_name("Height")
                            .index(2)
                            .required(true))
                        .arg(Arg::with_name("Path")
                            .index(3)
                            .required(true))
                        .arg(Arg::with_name("Frequency")
                            .short("f")
                            .takes_value(true))
                        .arg(Arg::with_name("Octaves")
                            .short("o")
                            .takes_value(true))
                    .get_matches();

    let width: u32 = match matches.value_of("Width").unwrap().parse::<u32>() {
        Ok(width) => width,
        Err(err) => panic!(err),
    };

    let height: u32 = match matches.value_of("Height").unwrap().parse::<u32>() {
        Ok(height) => height, 
        Err(err) => panic!(err),
    };

    let freq: u32 = match matches.value_of("Frequency") {
        Some(x) => { 
            match x.parse::<u32>() {
                Ok(x) => x,
                Err(_) => {
                    println!("Frequency must be a valid number!");
                    4
                },
            }
        },
        None => 4,
    };

    let oct: u32 = match matches.value_of("Octaves") {
        Some(x) => { 
            match x.parse::<u32>() {
                Ok(x) => x,
                Err(_) => {
                    println!("Octaves must be a valid number!");
                    1
                },
            }
        },
        None => 1,
    };

    let path: &str = match matches.value_of("Path") {
        Some(path) => {
            if Path::new(path).exists() {
                println!("Image '{}' already exists, choose another name!", path);
                process::exit(1);
            }else{
                path
            }
        },
        None => panic!("Requre path to run")
    };

    let start = Instant::now();

    perlin_image(width, height, freq, oct).save(path).unwrap();

    println!("Generated image in {} ms - Saved as '{}'", Instant::now().duration_since(start).as_millis(), path);
}

pub fn perlin_image(width: u32, height: u32, frequency: u32, octaves: u32) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    
    let shortest_side = if width <= height { width } else { height };
    
    let perlin : Arc<Perlin2D> = Arc::new(
        Perlin2D::new(
            frequency,
            octaves,
        )
    );

    let mut raw_pixels : Vec<u8> = Vec::with_capacity((width * height) as usize);

    let thread_count = 4;
    let pixels_per_thread = raw_pixels.capacity() / thread_count;
    let mut thread_handles : Vec<JoinHandle<Vec<u8>>> = Vec::with_capacity(thread_count);

    for thread_idx in 0..thread_handles.capacity() {

        let thread_perlin = Arc::clone(&perlin);

        let thread = thread::spawn(move || {
            let mut raw_pixel_section : Vec<u8> = Vec::with_capacity(pixels_per_thread);

            let pixel_range = std::ops::Range {
                start:  pixels_per_thread * thread_idx,
                end:    pixels_per_thread * (thread_idx+1)
            };

            for pixel_idx in pixel_range {

                let x = pixel_idx % width as usize;
                let y = pixel_idx / width as usize;

                let gray = thread_perlin.noise(
                    x as f32 / shortest_side as f32,
                    y as f32 / shortest_side as f32,
                );
        
                raw_pixel_section.push(
                    (gray * 255.0).round() as u8
                );
            }

            raw_pixel_section
        });

        thread_handles.push(thread);
    }

    thread_handles.reverse();

    while thread_handles.len() > 0 {
        let handle = thread_handles.pop().unwrap();

        let mut raw_pixel_section = handle.join().unwrap();

        raw_pixels.append(&mut raw_pixel_section);
    }

    let img_buf: Option<image::ImageBuffer<image::Luma<u8>, _>> = image::ImageBuffer::from_raw(width, height, raw_pixels);

    match img_buf {
        Some(image) => image,
        None => panic!("Couldn't create image from container!"),
    }
}