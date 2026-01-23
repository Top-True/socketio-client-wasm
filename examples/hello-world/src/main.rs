use socketio_client_wasm::prelude::*;

fn main() {
    let socket = io(&"http://localhost:3000/").new();
    gloo::console::log!(socket.as_ref());
}
