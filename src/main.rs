// use math::*;
// use render_3d::Drawer;
use crate::{gui_display, terminal_display};
use std::env;
use terminal_renderer::*;

fn main() {
    std::panic::set_hook(Box::new(&panic_handler));

    let _clean_up = CleanUp;

    // Barely works for any terminals :(
    // execute!(
    //     io::stdout(),
    //     event::PushKeyboardEnhancementFlags(event::KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
    // )
    // .expect("couldn't push keyboard enhancement flags");

    // TODO: Make better argument parsing system
    let mut args = env::args();

    if args.any(|str| str == "-t") {
        terminal_display::run();
    } else {
        gui_display::run().expect("gui failed");
    }
}

pub fn panic_handler(info: &std::panic::PanicInfo) {
    clean_up();

    print!("{}", info);
}
