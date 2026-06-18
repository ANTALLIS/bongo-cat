// use cocoa::appkit::NSWindow;
// use cocoa::base::{id, nil};
// use objc::{msg_send, sel, sel_impl};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World!")
        .undecorated()
        .transparent()
        .build();

    let cat_base_img =
        Image::load_image("./assets/resources/base.png").expect("Base image not found!");
    let cat_base_texture = rl
        .load_texture_from_image(&thread, &cat_base_img)
        .expect("Failed to create cat base texture!");

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLANK);
        d.draw_text("Hello, World!", 12, 12, 20, Color::WHITE);
        d.draw_texture(&cat_base_texture, 100, 100, Color::WHITE);
    }
}
