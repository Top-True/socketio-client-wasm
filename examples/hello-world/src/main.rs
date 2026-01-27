use gloo::console;
use socketio_client_wasm::prelude::*;
use std::panic::set_hook;

fn main() {
    set_hook(Box::new(|info| {
        console::error!(format!("Rust panic: {}", info));
    }));
    let socket = io(&"http://localhost:3000/").auto_connect(false).new();
    let socket2 = socket.clone();
    socket.async_on_connect(async move || {
        console::log!("Connected");
        console::log!(
            "ack response:",
            socket2
                .emit_with_ack("some_event")
                .await
                .expect("Emit Failed")
        );
    });
    socket.on_disconnect(|reason| {
        console::log!(format!("Disconnected: {}", reason));
    });
    socket.connect();
}
