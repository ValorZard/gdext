mod as_arg;
mod base;
mod instance_id;
mod obj;
mod registry;
mod storage;

pub mod builder;
pub mod macros;
pub mod property_info;
pub mod traits;

pub use as_arg::*;
pub use base::*;
pub use instance_id::*;
pub use obj::*;
pub use registry::*;
pub use traits::*;

use gdext_sys as sys;

mod gen {
    #[allow(unused_imports)]
    pub(crate) mod classes;
    pub mod utilities;
}

pub mod api {
    pub use super::gen::classes::*;
    pub use super::gen::utilities;
}

#[doc(hidden)]
pub mod private {
    pub use crate::storage::as_storage;
}

#[cfg(feature = "trace")]
#[macro_export]
macro_rules! out {
    ()                          => (eprintln!());
    ($fmt:literal)              => (eprintln!($fmt));
    ($fmt:literal, $($arg:tt)*) => (eprintln!($fmt, $($arg)*);)
}

#[cfg(not(feature = "trace"))]
// TODO find a better way than sink-writing to avoid warnings, #[allow(unused_variables)] doesn't work
#[macro_export]
macro_rules! out {
    ()                          => ({});
    ($fmt:literal)              => ({ use std::io::{sink, Write}; let _ = write!(sink(), $fmt); });
    ($fmt:literal, $($arg:tt)*) => ({ use std::io::{sink, Write}; let _ = write!(sink(), $fmt, $($arg)*); };)
}
