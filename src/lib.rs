pub use anyhow::{Error, Result};
pub use tonic::transport::{Channel, Endpoint};

pub mod lobby {
  tonic::include_proto!("lobby");
}

pub mod node {
  tonic::include_proto!("node");
}
