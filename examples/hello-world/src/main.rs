use socketio_client_wasm::prelude::*;

fn main() {
    let socket = io(&"http://localhost:3000/").new();
    socket.on_any(|_: &str| {
        gloo::console::log!("Hello, world!");
    });
    gloo::console::log!(socket.as_ref());
}
