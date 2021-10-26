use std::fmt::{Debug, Display, Formatter};

/// The error which will be returned, if a value could not be parsed into an `HumanReadableDuration`
pub struct ParseHumanReadableDurationError;

/// `?` formatting.
///
/// `Debug` should format the output in a programmer-facing, debugging context.
impl Debug for ParseHumanReadableDurationError {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseHumanReadableDurationError")
    }
}

/// Format trait for an empty format, `{}`.
///
/// `Display` is similar to [`Debug`], but `Display` is for user-facing
/// output.
impl Display for ParseHumanReadableDurationError {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseHumanReadableDurationError")
    }
}

impl std::error::Error for ParseHumanReadableDurationError {}
