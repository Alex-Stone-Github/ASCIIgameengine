mod ascii_engine;

use ascii_engine::drawing;
use ascii_engine::util;


fn main() {
    // constants
    const FPS: i32 = 15;
    const SCREEN_WIDTH: i32 = 30;
    const SCREEN_HEIGHT: i32 = 25;
    let mut framebuffer: [char;SCREEN_HEIGHT as usize*SCREEN_WIDTH as usize] = ['.';SCREEN_WIDTH as usize*SCREEN_HEIGHT as usize];

    // game loop
    loop {
        //logic
        //render
        drawing::print_console_clear_char();
        drawing::plot_filled_rect_to_framebuffer(&mut framebuffer, SCREEN_WIDTH, SCREEN_HEIGHT, 3, 3, 3, 3, '4');
        drawing::plot_filled_triangle_to_framebuffer(&mut framebuffer, SCREEN_WIDTH, SCREEN_HEIGHT, 6, 6, 20, 6, 6, 20, '&');
        drawing::print_framebuffer(&framebuffer, SCREEN_WIDTH, SCREEN_HEIGHT);
        util::sleep(1000 / FPS);
    }
}







