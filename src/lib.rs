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

/// The internally used time units which are supported.
enum InternalTimeUnit {
    Seconds,
    Minutes,
}

impl FromStr for InternalTimeUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ensure that the string has to be at least one character long
        if s.len() < 1 {
            return Err(());
        }

        // match the first character to the corresponding unit
        match s.to_lowercase().chars().next().unwrap() {
            's' => Ok(InternalTimeUnit::Seconds),
            'm' => Ok(InternalTimeUnit::Minutes),
            _ => Err(()),
        }
    }
}

/// A tuple of a time unit and the corresponding value (only for internal use).
struct InternalTime(u64, InternalTimeUnit);

/// A method for extracting the containing time information from a string. This method should
/// only be used internally.
fn extract_time_information(_: &str) -> Vec<InternalTime> {
    vec![]
}

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
        // try to get the time information from the passed string
        let time_information = extract_time_information(value);

        // if we could not extract any information, return an error
        if time_information.is_empty() {
            return Err(ParseHumanReadableDurationError);
        }

        // sum up the seconds and return corresponding object
        let mut seconds = 0;
        for current_time_object in time_information {
            match current_time_object.1 {
                InternalTimeUnit::Seconds => seconds += current_time_object.0,
                InternalTimeUnit::Minutes => seconds += current_time_object.0 * 60,
            }
        }
        return Ok(HumanReadableDuration {
            time_in_seconds: seconds,
        });
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

/// Used to do value-to-value conversions while consuming the input value. It is the reciprocal of
/// [`Into`].
impl From<u16> for HumanReadableDuration {
    /// Create an instance for [`HumanReadableDuration`] from a `u16` field representing the seconds
    ///
    /// # Example
    /// ```
    /// use human_readable_time::HumanReadableDuration;
    ///
    /// let seconds: u16 = 300;
    /// let representation = HumanReadableDuration::from(seconds);
    ///
    /// assert_eq!(300, representation.seconds());
    /// assert_eq!(5, representation.minutes());
    /// ```
    fn from(value: u16) -> Self {
        HumanReadableDuration {
            time_in_seconds: value as u64,
        }
    }
}

/// Used to do value-to-value conversions while consuming the input value. It is the reciprocal of
/// [`Into`].
impl From<u8> for HumanReadableDuration {
    /// Create an instance for [`HumanReadableDuration`] from a `u8` field representing the seconds
    ///
    /// # Example
    /// ```
    /// use human_readable_time::HumanReadableDuration;
    ///
    /// let seconds: u8 = 120;
    /// let representation = HumanReadableDuration::from(seconds);
    ///
    /// assert_eq!(120, representation.seconds());
    /// assert_eq!(2, representation.minutes());
    /// ```
    fn from(value: u8) -> Self {
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
    fn from_str_10_s_works() {
        let representation = HumanReadableDuration::from_str("10 s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(10, representation.as_ref().unwrap().seconds());
        assert_eq!(0, representation.as_ref().unwrap().minutes());
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
        let representation = HumanReadableDuration::from_str("60s");
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

    #[test]
    fn from_str_5_m_works() {
        let representation = HumanReadableDuration::from_str("5 m");
        assert_eq!(true, representation.is_ok());
        assert_eq!(300, representation.as_ref().unwrap().seconds());
        assert_eq!(5, representation.as_ref().unwrap().minutes());
    }

    #[test]
    fn from_str_5m_works() {
        let representation = HumanReadableDuration::from_str("5m");
        assert_eq!(true, representation.is_ok());
        assert_eq!(300, representation.as_ref().unwrap().seconds());
        assert_eq!(5, representation.as_ref().unwrap().minutes());
    }

    #[test]
    fn from_str_60m_works() {
        let representation = HumanReadableDuration::from_str("60m");
        assert_eq!(true, representation.is_ok());
        assert_eq!(3600, representation.as_ref().unwrap().seconds());
        assert_eq!(60, representation.as_ref().unwrap().minutes());
    }

    #[test]
    fn from_str_61m_works() {
        let representation = HumanReadableDuration::from_str("61m");
        assert_eq!(true, representation.is_ok());
        assert_eq!(3660, representation.as_ref().unwrap().seconds());
        assert_eq!(61, representation.as_ref().unwrap().minutes());
    }

    #[test]
    fn from_str_4_m_10_s_works() {
        let representation = HumanReadableDuration::from_str("4 m 10 s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(250, representation.as_ref().unwrap().seconds());
        assert_eq!(4, representation.as_ref().unwrap().minutes());
    }

    #[test]
    fn from_str_4m_10s_works() {
        let representation = HumanReadableDuration::from_str("4m 10s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(250, representation.as_ref().unwrap().seconds());
        assert_eq!(4, representation.as_ref().unwrap().minutes());
    }

    #[test]
    fn from_str_4m10s_works() {
        let representation = HumanReadableDuration::from_str("4m10s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(250, representation.as_ref().unwrap().seconds());
        assert_eq!(4, representation.as_ref().unwrap().minutes());
    }

    #[test]
    fn from_str_3m60s_works() {
        let representation = HumanReadableDuration::from_str("3m60s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(240, representation.as_ref().unwrap().seconds());
        assert_eq!(4, representation.as_ref().unwrap().minutes());
    }

    #[test]
    fn from_str_3m61s_works() {
        let representation = HumanReadableDuration::from_str("3m61s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(241, representation.as_ref().unwrap().seconds());
        assert_eq!(4, representation.as_ref().unwrap().minutes());
    }
}
