//! Socket creation and configuration utilities.

pub mod igmp;
pub mod mld;

use std::io;

/// Common socket setup result type
pub type SocketResult<T> = io::Result<T>;
