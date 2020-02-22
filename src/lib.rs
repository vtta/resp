#![deny(missing_docs)]

//! A crate for serialize and deserialize data
//! into RESP(REdis Serializable Protocol) representation

pub use de::{Deserializer, from_str};
pub use error::{Error, Result};
pub use ser::{Serializer, to_string};

mod de;
mod error;
mod ser;
