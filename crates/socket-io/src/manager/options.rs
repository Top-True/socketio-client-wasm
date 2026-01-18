use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub auto_connect: Option<bool>,
    pub parser: Option<super::parser::Parser>,
    pub randomization_factor: Option<f64>,
    pub reconnection: Option<bool>,
    pub reconnection_attempts: Option<ReconnectionAttempts>,
    pub reconnection_delay: Option<Duration>,
    pub reconnection_delay_max: Option<Duration>,
    pub timeout: Option<Duration>,
}

#[derive(Debug, Clone)]
pub enum ReconnectionAttempts {
    Infinity,
    U32(u32),
}
