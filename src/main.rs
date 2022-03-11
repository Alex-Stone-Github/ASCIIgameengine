mod ascii_engine {
    pub mod geometry {
        #[derive(Debug, Copy, Clone)]
        pub struct Vector3 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }
        impl Vector3 {
            pub fn new(x: f32, y: f32, z: f32) -> Self {
                Self {
                    x: x,
                    y: y,
                    z: z
                }
            }
            pub fn normalise(&mut self) {
                let length: f32 = (self.x*self.x+self.y*self.y+self.z*self.z).sqrt();
                self.x /= length;
                self.y /= length;
                self.z /= length;
            }
            pub fn opposite(&self) -> Vector3 {
                Vector3::new(-self.x, -self.y, -self.z)
            }
        }
        fn ortho(vector: &Vector3) -> Vector3 {
            Vector3::new(
                vector.x,
                vector.y,
                0.0 
            )
        }
        // I worked out the rotation matrices here for algebra
        fn rotatex(input: &Vector3, angler: f32) -> Vector3 {
            Vector3::new(
                input.x,
                angler.cos() * input.y + -angler.sin() * input.z,
                angler.sin() * input.y + angler.cos() * input.z
            )
        }
        fn rotatey(input: &Vector3, angler: f32) -> Vector3 {
            Vector3::new(
                angler.cos() * input.x + angler.sin() * input.z,
                input.y,
                -angler.sin() * input.x + angler.cos() * input.z
            )
        }
        fn rotatez(input: &Vector3, angler: f32) -> Vector3 {
            Vector3::new(
                angler.cos() * input.x + -angler.sin() * input.y,
                angler.sin() * input.x + angler.cos() * input.y,
                input.z
            )
        }
        fn translate(input: &Vector3, translation: &Vector3) -> Vector3 {
            Vector3::new(
                input.x + translation.x,
                input.y + translation.y,
                input.z + translation.z,
            )
        }
        pub fn render_and_project(
            input: &Vector3, rotation: &Vector3,
            translation: &Vector3, camera_orientation: &Vector3,
            camera_position: &Vector3
        ) -> Vector3 {
            let mut position: Vector3 = input.clone(); // model cord

            //global moves
            // apply rotation
            position = rotatez(&position, rotation.z);
            position = rotatey(&position, rotation.y);
            position = rotatex(&position, rotation.x);
            // move to world cordinates
            position = translate(&position, translation);
            // camera
            // move by camera position
            position = translate(&position, &camera_position.opposite());
            // move by the camra rotation
            let orientation = camera_orientation.opposite();
            position = rotatez(&position, -orientation.z);
            position = rotatey(&position, -orientation.y);
            position = rotatex(&position, -orientation.x);

            // project
            ortho(&position)
        }
    }
    pub mod util {
        use std::{thread, time};
        pub fn lerp(x1: f32, x2: f32, t: f32) -> f32 {
            x1 + (x2 - x1) * t
        }
        pub fn sleep(ms: i32) {
            let duration = time::Duration::from_millis(ms as u64);
            thread::sleep(duration);
        }
    }
    pub mod render {
        /*
         * When using this module of the engine module you should allocate a char[] that has
         * SCR_SIZE * SCR_SIZE elements in it and pass it to all of the functions that draw stuff
         * in this module.
         */
        pub fn print_console_clear_char() {
            // or just you know move the cursor to 1;1
            print!("{escape}[2J{escape}[1;1H", escape = 27 as char);
        }
        pub fn print_framebuffer(framebuffer: &[char], screen_width: i32, screen_height: i32) {
            for i in 0..screen_height {
                for j in 0..screen_width {
                    // print two chars to acount for the fact
                    // that letters are taller than they are wide
                    print!("{}", framebuffer[(i * screen_width + j) as usize]);
                    print!("{}", framebuffer[(i * screen_width + j) as usize]);
                }
                // newline for rows
                println!();
            }
        }
        pub fn plot_pixel_to_framebuffer(framebuffer: &mut [char], screen_width: i32,
                                         screen_height: i32, x: i32, y: i32,
                                         d: char) {
            if x > -1 && x < screen_width && y > -1 && y < screen_height {
                let index: i32 = y * screen_width + x;
                framebuffer[index as usize] = d;
            }
        }
        pub fn plot_rect_to_framebuffer(framebuffer: &mut [char], screen_width: i32,
                                        screen_height: i32, x: i32, y: i32, w: i32,
                                        h: i32, d: char) {
            for _x in x..x+w {
                for _y in y..y+h {
                    plot_pixel_to_framebuffer(framebuffer, screen_width, screen_height, _x, _y, d);
                }
            }
        }
        pub fn plot_line_to_framebuffer(framebuffer: &mut [char], screen_width: i32,
                                        screen_height: i32, x1: i32, y1: i32,
                                        x2: i32, y2: i32, d: char) {
            const LINE_STEPS: i32 = 100;

            for i in 0..LINE_STEPS {
                let x = super::util::lerp(x1 as f32, x2 as f32, i as f32 / LINE_STEPS as f32);
                let y = super::util::lerp(y1 as f32, y2 as f32, i as f32 / LINE_STEPS as f32);
                plot_pixel_to_framebuffer(framebuffer, screen_width, screen_height, x as i32, y as i32, d);
            }
        }
        pub fn plot_triangle_to_framebuffer(framebuffer: &mut [char], screen_width: i32,
                                            screen_height: i32, x1: i32, y1: i32,
                                            x2: i32, y2: i32, x3: i32, y3: i32,
                                            d: char) {
            plot_line_to_framebuffer(framebuffer, screen_width, screen_height, x1, y1, x2, y2, d);
            plot_line_to_framebuffer(framebuffer, screen_width, screen_height, x2, y2, x3, y3, d);
            plot_line_to_framebuffer(framebuffer, screen_width, screen_height, x3, y3, x1, y1, d);
        }
    }
}

use ascii_engine::geometry;
use ascii_engine::util;
use ascii_engine::render;

fn main() {
    let vector: geometry::Vector3 = geometry::Vector3::new(3.3, 4.4, 5.5);

    let output_position: geometry::Vector3 = geometry::render_and_project(
        &vector, // starting position
        &geometry::Vector3::new(0.0, 0.0, 0.0), // object rotation
        &geometry::Vector3::new(0.0, 0.0, 0.0), // object word cords
        &geometry::Vector3::new(0.0, 0.0, 0.0), // camera orientation as angles ?
        &geometry::Vector3::new(0.0, 0.0, 0.0)  // camera position
        );
    println!("{:?}\n{:?}", vector, output_position);
}







