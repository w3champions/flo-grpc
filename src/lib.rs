pub use anyhow::{Error, Result};
pub use tonic::transport::{Channel, Endpoint};

pub mod player {
  tonic::include_proto!("player");
}

pub mod game {
  tonic::include_proto!("game");
}

pub mod auth {
  tonic::include_proto!("auth");
}

pub mod lobby {
  tonic::include_proto!("lobby");
}

pub mod connect {
  tonic::include_proto!("connect");
}
