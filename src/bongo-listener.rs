use rdev::{EventType, Key, listen};
use std::io::Write;
use std::os::unix::net::UnixStream;

fn is_left_side(key: Key) -> bool {
    use Key::*;
    matches!(
        key,
        KeyQ | KeyW
            | KeyE
            | KeyR
            | KeyT
            | KeyA
            | KeyS
            | KeyD
            | KeyF
            | KeyG
            | KeyZ
            | KeyX
            | KeyC
            | KeyV
            | KeyB
            | Num1
            | Num2
            | Num3
            | Num4
            | Num5
            | Num6
            | Tab
            | CapsLock
            | ShiftLeft
            | ControlLeft
            | Alt
            | MetaLeft
    )
}

fn main() {
    // Connect to the Unix socket created by the bongo_cat process
    let mut stream = UnixStream::connect("/tmp/bongocat.sock")
        .expect("Cannot connect to bongo-cat. Run it first.");

    // This blocks the main thread – perfect, because we want the listener here.
    if let Err(error) = listen(move |event| {
        let (side, pressed) = match event.event_type {
            EventType::KeyPress(key) => (if is_left_side(key) { "left" } else { "right" }, true),
            EventType::KeyRelease(key) => (if is_left_side(key) { "left" } else { "right" }, false),
            _ => return,
        };
        // Send a simple line: "left:true" or "right:false"
        let _ = writeln!(stream, "{side}:{pressed}");
    }) {
        eprintln!("rdev error: {error:?}");
    }
}
