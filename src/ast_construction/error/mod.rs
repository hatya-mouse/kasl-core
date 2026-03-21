mod collector_wrappers;
pub mod error_collector;
pub mod error_kind;
pub mod error_record;
pub mod payload;

pub use error_collector::ErrorCollector;
pub use error_kind::ErrorKind;
pub use error_record::{ErrorKey, ErrorRecord, Phase, Severity};
pub use payload::Payload;

pub type EK = crate::error::ErrorKind;
pub type Ph = crate::error::Phase;
pub type Sv = crate::error::Severity;
pub type Pl = crate::error::Payload;
