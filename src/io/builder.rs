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
        // todo
        // let options = JsObject::assign3(
        //     &self.io_factory_options.to_js(),
        //     &self.engine_io_options.to_js(),
        //     &self.manager_options.to_js(),
        //     &self.socket_options.to_js(),
        // );
        let options = JsObject::new();
        let raw = global_io()
            .call2(&js_global(), &self.uri.as_str().into(), &options)
            .unwrap()
            .unchecked_into();
        component_emitter::EmitterWithJsRaw::from_raw(raw)
    }
}
