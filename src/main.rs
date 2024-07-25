use macroquad::prelude::*;
use std::time::{Duration, Instant};

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;

fn rand_colour(seed: i32) -> Color {
    if seed == 0 {return Color::from_rgba(0, 0, 0, 255);}
    let red: u8 = (((seed * 856) >> 6) % 256) as u8;
    let green: u8 = (((seed * 685) >> 5) % 256) as u8;
    let blue: u8 = (((seed * 568) >> 7) % 256) as u8;
    return Color::from_rgba(red, green, blue, 255);
}

fn config_window() -> Conf {
    return Conf {
        window_title: "Scrolling Primes".to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

fn is_prime(num: i32) -> bool {
    if num == 0 || num == 1 || num == 4{ return false; }
    if num == 2 { return true; }
    for i in 2..num/2 {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}
fn field_filter(x: i32, y: i32) -> i32 {
    let xor_val = x | y;
    if is_prime(xor_val) {
        return xor_val;
    } else {
        return 0;
    }
}



fn render(num_array: Vec<Vec<i32>>, offset: i32) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let val = num_array[(y + offset) as usize][(x + offset) as usize];
            if val > 0 {
                draw_line(x as f32, y as f32, 1.0 + x as f32, y as f32, 1.0, rand_colour(val));
            }}}}


#[macroquad::main(config_window())]
async fn main() {
    println!("Hello, world!"); // line is here to make sure the program compiles
    // initialize image and texture
    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&mut image);

    let mut offset = 0; // offset of x and y
    let mut num_array: Vec<Vec<i32>> = vec![];
    for y in 0..HEIGHT {
        let mut row: Vec<i32> = vec![];
        for x in 0..WIDTH {
            row.push(field_filter(x, y));
        }
        num_array.push(row);
    }

    loop {
        clear_background(BLACK);

        let now = Instant::now();// <- these lines marked with // are for benchmarking

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                image.set_pixel(x as u32, y as u32, rand_colour(num_array[(y + offset) as usize][(x + offset) as usize]));
            }
        }
        texture.update(&image);
        draw_texture(&texture, 0.0, 0.0, WHITE);


        let render_time = now.elapsed().as_millis(); //
        let mut new_row: Vec<i32> = vec![];
        let now = Instant::now(); //
        for _ in 0..offset {
            new_row.push(0);
        }
        for x in 0..WIDTH {
            new_row.push(field_filter(offset + x, HEIGHT + offset));
        }
        num_array.push(new_row);
        offset += 1;
        for i in 0..offset {
            num_array[i as usize].push(0);
        }
        for y in 0..HEIGHT {
            num_array[(y + offset) as usize].push(field_filter(WIDTH + offset, y + offset));
        }
        let update_time = now.elapsed().as_millis(); //
        println!("Render time: {}ms | Update time: {}ms", render_time, update_time);

        let fps_text = format!("FPS: {}", 1.0 / (render_time as f32 / 1000.0));
        draw_text(fps_text.as_str(), 12.0, 17.0, 20.0, BLACK);
        draw_text(fps_text.as_str(), 10.0, 15.0, 20.0, WHITE);

        next_frame().await;
    }
}
