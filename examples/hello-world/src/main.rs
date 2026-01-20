use socketio_client_wasm::prelude::*;

fn main() {
    let socket = io(&"http://localhost:3000/").new();
    gloo::console::log!(Into::<js_sys::wasm_bindgen::JsValue>::into(socket));
}
