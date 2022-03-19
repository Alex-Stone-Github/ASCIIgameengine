use super::util::{lerp};

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
pub fn plot_filled_rect_to_framebuffer(framebuffer: &mut [char], screen_width: i32,
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
        let x = lerp(x1 as f32, x2 as f32, i as f32 / LINE_STEPS as f32);
        let y = lerp(y1 as f32, y2 as f32, i as f32 / LINE_STEPS as f32);
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
pub fn plot_filled_triangle_to_framebuffer(framebuffer: &mut [char], screen_width: i32,
                                    screen_height: i32, x1: i32, y1: i32,
                                    x2: i32, y2: i32, x3: i32, y3: i32,
                                    d: char) {
    let STEPS: i32 = 100;
    for i in 0..STEPS {
        let p1x = lerp(x2 as f32, x1 as f32, i as f32 / STEPS as f32) as i32;
        let p1y = lerp(y2 as f32, y1 as f32, i as f32 / STEPS as f32) as i32;
        let p2x = lerp(x2 as f32, x3 as f32, i as f32 / STEPS as f32) as i32;
        let p2y = lerp(y2 as f32, y3 as f32, i as f32 / STEPS as f32) as i32;
        plot_line_to_framebuffer(framebuffer, screen_width, screen_height, p1x, p1y, p2x, p2y, d);
    }
}






