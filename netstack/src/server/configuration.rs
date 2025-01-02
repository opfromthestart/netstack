use crate::security::Secret;

pub struct Configuration {
    pub max_connections: usize,
    /// Timeout after `timeout` updates, should be several lengths of `heartbeat`
    pub timeout: usize,
    pub reserved_timeout: usize,
    /// Send a heartbeat message every `heartbeat` updates
    pub heartbeat: usize,
    /// Allows all connections with a given secret
    pub allow_all: Option<Secret>,
}
