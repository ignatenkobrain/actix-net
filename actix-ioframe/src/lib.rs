#![deny(rust_2018_idioms, warnings)]
#![allow(clippy::type_complexity, clippy::too_many_arguments)]

mod connect;
mod dispatcher;
mod error;
mod item;
mod service;
mod sink;

pub use self::connect::{Connect, ConnectResult};
pub use self::error::ServiceError;
pub use self::item::Item;
pub use self::service::{Builder, NewServiceBuilder, ServiceBuilder};
pub use self::sink::Sink;
