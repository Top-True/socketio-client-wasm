mod global;
mod to_js;
mod undefined_option;
mod utils;

pub use global::{global as js_global, global_io};
pub use gloo; // todo
pub use js_sys::Array as JsArray;
pub use js_sys::Error as JsError;
pub use js_sys::Function as JsFunction;
pub use js_sys::JsString;
pub use js_sys::Number as JsNumber;
pub use js_sys::Object as JsObject;
pub use js_sys::Promise as JsPromise;
pub use js_sys::Reflect as JsReflect;
pub use js_sys::eval as js_eval;
pub use js_sys::wasm_bindgen;
pub use js_sys::wasm_bindgen::{JsCast, JsValue, closure::Closure as JsClosure};
pub use to_js::{DurationToJs, ToJs};
pub use undefined_option::JsUndefinedOption;
pub use utils::set_property;
pub use wasm_bindgen_futures::JsFuture;
