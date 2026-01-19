use socketio_client_wasm::prelude::*;

fn main() {
    Manager::new(&"localhost:3000", Default::default());
}
