#![cfg_attr(rustfmt, rustfmt_skip)] // TODO: Remove attribute before final merge
mod lang;
mod en;
mod es;
mod fr;
mod uk;

pub use en::English;
pub use es::Spanish;
pub use fr::French;
pub use uk::Ukrainian;

pub use lang::to_language;
pub use lang::Lang;
pub use lang::Language;
