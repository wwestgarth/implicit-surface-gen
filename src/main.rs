use colour::{write_colour, Colour};
use std::io;

mod vec3;
mod colour;


fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);


    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining {}", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let pixel = Colour::new(r,g,b);
            write_colour(&mut io::stdout(), pixel);
        }
    }

    eprint!("\nDone!\n")

}
