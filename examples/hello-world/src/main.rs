use gloo::console;
use socketio_client_wasm::prelude::*;
use wasm_bindgen_futures::spawn_local;

async fn emit_with_ack(_socket: Socket) {
    // socket.emit_with_ack("some_event").await;
    console::log!("ack");
}

fn main() {
    // fixme: async support
    let socket = io(&"http://localhost:3000/").auto_connect(false).new();
    console::log!(socket.as_ref());
    let socket2 = socket.clone();
    socket.once_connect(move || {
        console::log!("Connected");
        spawn_local(emit_with_ack(socket2.clone()));
    });
    socket.connect();
}
