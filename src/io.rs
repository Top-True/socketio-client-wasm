pub mod builder;
pub mod options;

use js_raw::*;

pub fn protocol() -> i32 {
    let io = JsReflect::get(&js_global(), &"io".into())
        .unwrap()
        .dyn_into::<JsFunction>()
        .unwrap();
    JsReflect::get(&io, &"protocol".into())
        .unwrap()
        .as_f64()
        .unwrap() as i32
}

pub fn io(uri: impl AsRef<str>) -> builder::Builder {
    builder::Builder {
        uri: String::from(uri.as_ref()),
        io_factory_options: Default::default(),
    }
}
