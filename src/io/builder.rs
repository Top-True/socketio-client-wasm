use super::options::Options as IOFactoryOptions;
use scw_js_raw::*;
use scw_socket_io::socket::Socket;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Builder {
    pub(crate) uri: String,
    pub(crate) io_factory_options: IOFactoryOptions,
    pub(crate) engine_io_options: scw_socket_io::engine_io::Options,
    pub(crate) manager_options: scw_socket_io::manager::options::Options,
    pub(crate) socket_options: scw_socket_io::socket::options::Options,
}

impl Builder {
    pub fn new(self) -> Socket {
        let options = JsObject::assign_many(
            &self.io_factory_options.to_js(),
            &[
                self.engine_io_options.to_js(),
                self.manager_options.to_js(),
                self.socket_options.to_js(),
            ],
        )
        .unwrap();
        let raw = global_io()
            .call2(&js_global(), &self.uri.as_str().into(), &options)
            .unwrap()
            .unchecked_into();
        scw_component_emitter::EmitterWithJsRaw::from_raw(raw)
    }
}

macro_rules! opt_impl {
    ($self: ident.$opt: ident($t: ty)) => {
        pub fn $opt(mut self, v: $t) -> Self {
            self.$self.$opt = JsOption::Some(v);
            self
        }
    };
}

impl Builder {
    opt_impl!(io_factory_options.force_new(bool));
    opt_impl!(io_factory_options.multiplex(bool));
    opt_impl!(engine_io_options.add_trailing_slash(bool));
    opt_impl!(engine_io_options.auto_unref(bool));
    opt_impl!(engine_io_options.close_on_beforeunload(bool));
    opt_impl!(engine_io_options.extra_headers(JsObject));
    opt_impl!(engine_io_options.force_base64(bool));
    opt_impl!(engine_io_options.path(String));
    opt_impl!(engine_io_options.protocols(Vec<String>));
    opt_impl!(engine_io_options.query(JsObject));
    opt_impl!(engine_io_options.remember_upgrade(bool));
    opt_impl!(engine_io_options.timestamp_param(String));
    opt_impl!(engine_io_options.timestamp_requests(bool));
    opt_impl!(engine_io_options.transport_options(JsObject));
    opt_impl!(engine_io_options.transports(scw_socket_io::engine_io::TransportOption));
    opt_impl!(engine_io_options.try_all_transports(bool));
    opt_impl!(engine_io_options.upgrade(bool));
    opt_impl!(engine_io_options.with_credentials(bool));
    opt_impl!(manager_options.auto_connect(bool));
    opt_impl!(manager_options.randomization_factor(f64));
    opt_impl!(manager_options.reconnection(bool));
    opt_impl!(
        manager_options
            .reconnection_attempts(scw_socket_io::manager::options::ReconnectionAttempts)
    );
    opt_impl!(manager_options.reconnection_delay(Duration));
    opt_impl!(manager_options.reconnection_delay_max(Duration));
    opt_impl!(manager_options.timeout(Duration));
    opt_impl!(socket_options.ack_timeout(Duration));
    opt_impl!(socket_options.auth(JsObject));
    opt_impl!(socket_options.retries(u32));
}
