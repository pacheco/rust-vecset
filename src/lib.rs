#[cfg(feature = "serde_support")]
extern crate serde;
#[cfg(feature = "serde_support")]
#[cfg_attr(feature = "serde_support", macro_use)]
extern crate serde_derive;

mod insord;
mod ord;

pub use insord::*;
pub use ord::*;
