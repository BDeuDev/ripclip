#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

use std::io::{Read, Write};

#[cfg(unix)]
pub use unix::{IpcListener, IpcStream};

#[cfg(windows)]
pub use windows::{IpcListener, IpcStream};

pub trait IpcConnection: Read + Write + Send {}

impl<T: Read + Write + Send> IpcConnection for T {}

/// Alias para simplificar el uso
pub type Result<T> = std::io::Result<T>;
