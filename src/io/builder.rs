use super::options::Options as IOFactoryOptions;
use js_raw::*;
use socket_io::socket::Socket;

#[derive(Debug, Clone)]
pub struct Builder {
    pub(crate) uri: String,
    pub(crate) io_factory_options: IOFactoryOptions,
    pub(crate) engine_io_options: socket_io::engine_io::Options,
    pub(crate) manager_options: socket_io::manager::options::Options,
    pub(crate) socket_options: socket_io::socket::options::Options,
}

impl Builder {
    pub fn new(self) -> Socket {
        let options = JsObject::assign3(
            &Into::<JsValue>::into(self.io_factory_options)
                .dyn_into()
                .unwrap(),
            &Into::<JsValue>::into(self.engine_io_options)
                .dyn_into()
                .unwrap(),
            &Into::<JsValue>::into(self.manager_options)
                .dyn_into()
                .unwrap(),
            &Into::<JsValue>::into(self.socket_options)
                .dyn_into()
                .unwrap(),
        );
        let raw = global_io()
            .call2(&js_global(), &self.uri.as_str().into(), &options)
            .unwrap()
            .dyn_into()
            .unwrap();
        Socket::new(raw)
    }
}
