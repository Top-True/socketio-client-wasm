use super::options::Options as IOFactoryOptions;

#[derive(Debug, Clone)]
pub struct Builder {
    pub(crate) uri: String,
    pub(crate) io_factory_options: IOFactoryOptions,
}

impl Builder {
    pub fn new(self) -> socket_io::socket::Socket {
        todo!()
    }
}
