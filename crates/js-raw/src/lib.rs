mod global;
mod utils;
mod undefined_option;

pub use gloo; // todo
pub use global::{global_io, global as js_global};
pub use utils::set_property;
pub use js_sys::Array as JsArray;
pub use js_sys::Error as JsError;
pub use js_sys::Function as JsFunction;
pub use js_sys::JsString;
pub use js_sys::Number as JsNumber;
pub use js_sys::Object as JsObject;
pub use js_sys::Reflect as JsReflect;
pub use js_sys::eval as js_eval;
pub use js_sys::wasm_bindgen::{JsCast, JsValue, closure::Closure as JsClosure};
pub use undefined_option::JsUndefinedOption;
