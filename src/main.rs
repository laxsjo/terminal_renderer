// use math::*;
// use render_3d::Drawer;
use crate::app;
use crate::has_value_changed;
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

    // let mut num = 0;

    // for i in 0..10 {
    //     if i % 2 == 0 {
    //         num = i;
    //     }
    //     if has_value_changed!(num) {
    //         println!("num changed to {}", num);
    //     }
    // }

    // TODO: Make better argument parsing system
    let mut args = env::args();

    if args.any(|str| str == "-t") {
        app::terminal_display::run();
    } else {
        app::gui_display::run();
    }
}

pub fn panic_handler(info: &std::panic::PanicInfo) {
    clean_up();

    print!("{}", info);
}
