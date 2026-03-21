use crate::{
    Range,
    error::{EK, ErrorCollector, Phase, Pl, Sv},
};
use std::panic::Location;

impl ErrorCollector {
    /// Automatically emits a CompilerBug error with the provided range, phase, and dev message,
    /// combined with the location of the caller.
    #[track_caller]
    pub(crate) fn comp_bug(&mut self, range: Range, phase: Phase, dev_msg: &str) {
        // Get the location of the caller
        let loc = Location::caller();
        // Format the location string
        let loc_str = format!("{}:{}:{}", loc.file(), loc.line(), loc.column());
        // Combine the location and the dev message into one string and emit
        let full = format!("{} -- {}", loc_str, dev_msg);
        self.emit(
            EK::CompilerBug,
            range,
            phase,
            Sv::CompilerBug,
            Pl::Str(full),
        );
    }
}
