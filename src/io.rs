pub mod builder;
pub mod options;

use js_raw::*;

pub fn protocol() -> i32 {
    JsReflect::get(&global_io(), &"protocol".into())
        .unwrap()
        .as_f64()
        .unwrap() as i32
}

pub fn io(uri: &impl AsRef<str>) -> builder::Builder {
    builder::Builder {
        uri: String::from(uri.as_ref()),
        io_factory_options: Default::default(),
        engine_io_options: Default::default(),
        manager_options: Default::default(),
        socket_options: Default::default(),
    }
}
