use raylib::prelude::*;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixListener;
use std::process::{Child, Command};
use std::sync::mpsc;
use std::thread;

fn main() {
    // Clean up
    let _ = std::fs::remove_file("/tmp/bongocat.sock");

    let listener = UnixListener::bind("/tmp/bongocat.sock").expect("Failed to bind socket");

    // Spawn the accept thread BEFORE starting the child process
    let (tx, rx) = mpsc::channel::<(String, bool)>();
    thread::spawn(move || {
        // Blocking accept, waits until the child connects
        match listener.accept() {
            Ok((stream, _)) => {
                let reader = BufReader::new(stream);
                for line in reader.lines() {
                    if let Ok(msg) = line {
                        let parts: Vec<&str> = msg.split(':').collect();
                        if parts.len() == 2 {
                            let side = parts[0].to_string();
                            let pressed = parts[1] == "true";
                            let _ = tx.send((side, pressed));
                        }
                    }
                }
            }
            Err(e) => eprintln!("Accept error: {e}"),
        }
    });

    // Spawn the key_listener child
    let key_listener_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("bongo-listener");

    let mut child: Child = Command::new(key_listener_path)
        .spawn()
        .expect("Failed to start key listener");

    // Raylib on main thread
    let (mut rl, thread) = raylib::init()
        .size(397, 201)
        .title("Bongo Cat")
        .undecorated()
        .transparent()
        .topmost()
        .build();

    // Load textures (your existing loading code)
    let theme = "default";
    let base_url = format!("./assets/theme/{theme}");
    let cat_base_img = Image::load_image(&format!("{base_url}/base.png")).unwrap();
    let cat_left_down_img = Image::load_image(&format!("{base_url}/left-down.png")).unwrap();
    let cat_left_up_img = Image::load_image(&format!("{base_url}/left-up.png")).unwrap();
    let cat_right_down_img = Image::load_image(&format!("{base_url}/right-down.png")).unwrap();
    let cat_right_up_img = Image::load_image(&format!("{base_url}/right-up.png")).unwrap();

    let cat_base_tex = rl.load_texture_from_image(&thread, &cat_base_img).unwrap();
    let cat_left_down_tex = rl
        .load_texture_from_image(&thread, &cat_left_down_img)
        .unwrap();
    let cat_left_up_tex = rl
        .load_texture_from_image(&thread, &cat_left_up_img)
        .unwrap();
    let cat_right_down_tex = rl
        .load_texture_from_image(&thread, &cat_right_down_img)
        .unwrap();
    let cat_right_up_tex = rl
        .load_texture_from_image(&thread, &cat_right_up_img)
        .unwrap();

    let mut left_paw_down = false;
    let mut right_paw_down = false;
    let mut dragging = false;
    let mut drag_start_mouse = Vector2::new(0.0, 0.0);
    let mut drag_start_window = Vector2::new(0.0, 0.0);

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // Drain incoming key events
        while let Ok((side, pressed)) = rx.try_recv() {
            match side.as_str() {
                "left" => left_paw_down = pressed,
                "right" => right_paw_down = pressed,
                _ => {}
            }
        }
        // --- window dragging ---
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            dragging = true;
            drag_start_mouse = rl.get_mouse_position();
            drag_start_window = rl.get_window_position();
        }
        if dragging && rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let delta = rl.get_mouse_position() - drag_start_mouse;
            rl.set_window_position(
                (drag_start_window.x + delta.x) as i32,
                (drag_start_window.y + delta.y) as i32,
            );
        }
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            dragging = false;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLANK);
        d.draw_texture(&cat_base_tex, 0, 0, Color::WHITE);

        let left_tex = if left_paw_down {
            &cat_left_down_tex
        } else {
            &cat_left_up_tex
        };
        let right_tex = if right_paw_down {
            &cat_right_down_tex
        } else {
            &cat_right_up_tex
        };

        d.draw_texture(left_tex, 0, 0, Color::WHITE);
        d.draw_texture(right_tex, 0, 0, Color::WHITE);
    }

    // Cleanup
    let _ = child.kill();
    let _ = std::fs::remove_file("/tmp/bongocat.sock");
}
