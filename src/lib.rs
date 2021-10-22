use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// A data structure for parsing and managing a human readable duration representation
pub struct HumanReadableDuration {
    time_in_seconds: u64,
}

impl HumanReadableDuration {
    /// Get the duration time in seconds
    ///
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use human_readable_time::HumanReadableDuration;
    ///
    /// let duration = HumanReadableDuration::from_str("10s");
    ///
    /// assert_eq!(10, duration.unwrap().seconds());
    /// ```
    pub fn seconds(&self) -> u64 {
        self.time_in_seconds
    }

    /// Get the duration time in full minutes
    ///
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use human_readable_time::HumanReadableDuration;
    ///
    /// let duration = HumanReadableDuration::from_str("65s");
    ///
    /// assert_eq!(1, duration.unwrap().minutes());
    /// ```
    pub fn minutes(&self) -> u64 {
        let divisor = self.time_in_seconds as f32;
        let result = divisor / 60.0f32;
        return result as u64;
    }
}

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

/// Parse a value from a string
///
/// `FromStr`'s `from_str` method is often used implicitly, through
/// `str`'s `parse` method. See `parse`'s documentation for examples.
impl FromStr for HumanReadableDuration {
    type Err = ParseHumanReadableDurationError;

    /// Parses a string `s` to return a value of the [`HumanReadableDuration`] type.
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return an error specific to the
    /// inside [`Err`].
    ///
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use human_readable_time::HumanReadableDuration;
    ///
    /// let s = "50s";
    /// let x = HumanReadableDuration::from_str(s).unwrap();
    ///
    /// assert_eq!(50, x.seconds());
    /// ```
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(HumanReadableDuration {
            time_in_seconds: value.split("s").next().unwrap().parse::<u64>().unwrap(),
        })
    }
}

/// Used to do value-to-value conversions while consuming the input value. It is the reciprocal of
/// [`Into`].
impl From<u64> for HumanReadableDuration {
    /// Create an instance for [`HumanReadableDuration`] from a `u64` field representing the seconds
    ///
    /// # Example
    /// ```
    /// use human_readable_time::HumanReadableDuration;
    ///
    /// let seconds: u64 = 300;
    /// let representation = HumanReadableDuration::from(seconds);
    ///
    /// assert_eq!(300, representation.seconds());
    /// assert_eq!(5, representation.minutes());
    /// ```
    fn from(value: u64) -> Self {
        HumanReadableDuration {
            time_in_seconds: value,
        }
    }
}

/// Used to do value-to-value conversions while consuming the input value. It is the reciprocal of
/// [`Into`].
impl From<u32> for HumanReadableDuration {
    /// Create an instance for [`HumanReadableDuration`] from a `u32` field representing the seconds
    ///
    /// # Example
    /// ```
    /// use human_readable_time::HumanReadableDuration;
    ///
    /// let seconds: u32 = 300;
    /// let representation = HumanReadableDuration::from(seconds);
    ///
    /// assert_eq!(300, representation.seconds());
    /// assert_eq!(5, representation.minutes());
    /// ```
    fn from(value: u32) -> Self {
        HumanReadableDuration {
            time_in_seconds: value as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HumanReadableDuration;
    use std::str::FromStr;

    #[test]
    fn from_u32_works() {
        let representation = HumanReadableDuration::from(300 as u32);
        assert_eq!(300, representation.seconds());
        assert_eq!(5, representation.minutes());
    }

    #[test]
    fn from_u64_works() {
        let representation = HumanReadableDuration::from(300 as u64);
        assert_eq!(300, representation.seconds());
        assert_eq!(5, representation.minutes());
    }

    #[test]
    fn from_str_10s_works() {
        let representation = HumanReadableDuration::from_str("10s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(10, representation.as_ref().unwrap().seconds());
        assert_eq!(0, representation.as_ref().unwrap().minutes());
    }

    #[test]
    fn from_str_60s_works() {
        let representation = HumanReadableDuration::from_str("60");
        assert_eq!(true, representation.is_ok());
        assert_eq!(60, representation.as_ref().unwrap().seconds());
        assert_eq!(1, representation.as_ref().unwrap().minutes());
    }

    #[test]
    fn from_str_61s_works() {
        let representation = HumanReadableDuration::from_str("61s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(61, representation.as_ref().unwrap().seconds());
        assert_eq!(1, representation.as_ref().unwrap().minutes());
    }
}
