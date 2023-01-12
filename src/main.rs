// use math::*;
// use render_3d::Drawer;
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

    flags::TerminalFlags::set_raw_mode(true);

    crate::run();

    // let triangle_0 = render_3d::Triangle(
    //     math::vec3(0.716, 0.326, -1.175),
    //     math::vec3(1.097, -0.768, 0.455),
    //     math::vec3(-0.716, -0.326, 1.175),
    // );
    // let triangle_1 = render_3d::Triangle(
    //     math::vec3(0.716, 0.326, -1.175),
    //     math::vec3(-0.716, -0.326, 1.175),
    //     math::vec3(-1.097, 0.768, -0.455),
    // );

    // let normal_0 = triangle_0.normal();
    // // let normal_1 = triangle_1.normal();

    // print_crlf!("tri: {: >7.4}", triangle_0);
    // print_crlf!("normal: {: >7.4}", normal_0);

    // let diff_1 = triangle_0.1 - triangle_0.0;
    // let diff_2 = triangle_0.2 - triangle_0.0;
    // print_crlf!("tri.1 - tri.0 = {: >7.4}", diff_1);
    // print_crlf!("tri.2 - tri.0 = {: >7.4}", diff_2);
    // let cross = diff_1.cross_product(diff_2);
    // print_crlf!(
    //     "(tri.1 - tri.0).cross_product(tri.2 - tri.0) = {: >7.4}",
    //     cross
    // );

    // let norm = cross.normalize();
    // print_crlf!("normalized = {: >7.4}", norm);

    // print_crlf!("tri 1: {: >7.4}", triangle_1);
    // print_crlf!("normal 1: {: >7.4}", normal_1);

    // loop {
    //     INPUT.wait_for_key(input::KeyCode::Enter);

    //     println!("Size: {:?}", crossterm::terminal::size());
    // }

    // let drawer = render_3d::AaLineDrawer::new(render_3d::udimensions(10, 10));
    // drawer.draw(
    //     render_3d::LineDrawParams(vec2(-0.1, -0.9), vec2(0.9, 0.9), 1.),
    //     |coords, color| {
    //         println!("Drew at ({}, {}) with color {}", coords.x, coords.y, color);
    //     },
    // )

    // dbg!(frame_intersection(
    //     Line(
    //         Vec2 {
    //             x: 0.49587078018294695,
    //             y: 0.9928293264526806
    //         },
    //         Vec2 {
    //             x: 0.5136107387251343,
    //             y: -1.029058840329496
    //         }
    //     ),
    //     (Vec2 { x: -1.0, y: -1.0 }, Vec2 { x: 1.0, y: 1.0 })
    // ));
}

pub fn panic_handler(info: &std::panic::PanicInfo) {
    clean_up();

    print!("{}", info);
}
