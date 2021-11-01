use crate::errors::ParseHumanReadableDurationError;
#[cfg(feature = "chrono")]
use crate::traits::AsDuration;
use crate::traits::{AsDays, AsHours, AsMinutes, AsSeconds};
use std::str::FromStr;

// the modules we have in this crate
pub mod errors;
pub mod traits;

/// A data structure for parsing and managing a human readable duration representation
pub struct HumanReadableDuration {
    time_in_seconds: u64,
}

impl AsSeconds for HumanReadableDuration {
    /// Get the duration time in seconds
    ///
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use human_readable_time::HumanReadableDuration;
    /// use human_readable_time::traits::AsSeconds;
    ///
    /// let duration = HumanReadableDuration::from_str("10s");
    ///
    /// assert_eq!(10, duration.unwrap().as_seconds());
    /// ```
    fn as_seconds(&self) -> u64 {
        self.time_in_seconds
    }
}

impl AsMinutes for HumanReadableDuration {
    /// Get the duration time in full minutes
    ///
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use human_readable_time::HumanReadableDuration;
    /// use human_readable_time::traits::AsMinutes;
    ///
    /// let duration = HumanReadableDuration::from_str("65s");
    ///
    /// assert_eq!(1, duration.unwrap().as_minutes());
    /// ```
    fn as_minutes(&self) -> u64 {
        let divisor = self.time_in_seconds as f32;
        let result = divisor / 60.0f32;
        return result as u64;
    }
}

impl AsHours for HumanReadableDuration {
    /// Get the duration time in full hours
    ///
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use human_readable_time::HumanReadableDuration;
    /// use human_readable_time::traits::AsHours;
    ///
    /// let duration = HumanReadableDuration::from_str("65m");
    ///
    /// assert_eq!(1, duration.unwrap().as_hours());
    /// ```
    fn as_hours(&self) -> u64 {
        let divisor = self.time_in_seconds as f32;
        let result = divisor / 3600.0f32;
        return result as u64;
    }
}

impl AsDays for HumanReadableDuration {
    /// Get the duration time in full days
    ///
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use human_readable_time::HumanReadableDuration;
    /// use human_readable_time::traits::AsDays;
    ///
    /// let duration = HumanReadableDuration::from_str("48h");
    ///
    /// assert_eq!(2, duration.unwrap().as_days());
    /// ```
    fn as_days(&self) -> u64 {
        let divisor = self.time_in_seconds as f32;
        let result = divisor / 86400.0f32;
        return result as u64;
    }
}

#[cfg(feature = "chrono")]
impl AsDuration for HumanReadableDuration {
    /// Convert the object to a [`chrono::Duration`]  representation.
    ///
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use human_readable_time::HumanReadableDuration;
    /// use human_readable_time::traits::AsDuration;
    ///
    /// let duration = HumanReadableDuration::from_str("65m").unwrap();
    ///
    /// assert_eq!(3900, duration.as_duration().num_seconds());
    /// assert_eq!(1, duration.as_duration().num_hours());
    /// ```
    fn as_duration(&self) -> chrono::Duration {
        chrono::Duration::seconds(self.time_in_seconds as i64) // TODO: check if `time_in_seconds` will fit in a i64
    }
}

/// The internally used time units which are supported.
enum InternalTimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
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
            'h' => Ok(InternalTimeUnit::Hours),
            'd' => Ok(InternalTimeUnit::Days),
            _ => Err(()),
        }
    }
}

/// A tuple of a time unit and the corresponding value (only for internal use).
struct InternalTime(u64, InternalTimeUnit);

/// A method for extracting the containing time information from a string. This method should
/// only be used internally.
fn extract_time_information(value: &str) -> Vec<InternalTime> {
    use lazy_static::lazy_static;
    use regex::Regex;

    // compile the regular expression for extracting the supported timings
    lazy_static! {
        static ref TIME_REGEX: Regex = Regex::from_str(r"([0-9]+)([dhms]){1}").unwrap();
    }

    // collect all found matches
    let mut found_matches = vec![];
    for capture in TIME_REGEX.captures_iter(value) {
        if let Ok(time) = u64::from_str(&capture[1]) {
            if let Ok(unit) = InternalTimeUnit::from_str(&capture[2]) {
                found_matches.push(InternalTime(time, unit))
            }
        }
    }

    // return the found matches
    found_matches
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
    /// use human_readable_time::traits::AsSeconds;
    ///
    /// let s = "50s";
    /// let x = HumanReadableDuration::from_str(s).unwrap();
    ///
    /// assert_eq!(50, x.as_seconds());
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
                InternalTimeUnit::Hours => seconds += current_time_object.0 * 3600,
                InternalTimeUnit::Days => seconds += current_time_object.0 * 86400,
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
    /// use human_readable_time::traits::{AsMinutes, AsSeconds};
    ///
    /// let seconds: u64 = 300;
    /// let representation = HumanReadableDuration::from(seconds);
    ///
    /// assert_eq!(300, representation.as_seconds());
    /// assert_eq!(5, representation.as_minutes());
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
    /// use human_readable_time::traits::{AsMinutes, AsSeconds};
    ///
    /// let seconds: u32 = 300;
    /// let representation = HumanReadableDuration::from(seconds);
    ///
    /// assert_eq!(300, representation.as_seconds());
    /// assert_eq!(5, representation.as_minutes());
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
    /// use human_readable_time::traits::{AsMinutes, AsSeconds};
    ///
    /// let seconds: u16 = 300;
    /// let representation = HumanReadableDuration::from(seconds);
    ///
    /// assert_eq!(300, representation.as_seconds());
    /// assert_eq!(5, representation.as_minutes());
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
    /// use human_readable_time::traits::{AsMinutes, AsSeconds};
    ///
    /// let seconds: u8 = 120;
    /// let representation = HumanReadableDuration::from(seconds);
    ///
    /// assert_eq!(120, representation.as_seconds());
    /// assert_eq!(2, representation.as_minutes());
    /// ```
    fn from(value: u8) -> Self {
        HumanReadableDuration {
            time_in_seconds: value as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::{AsDays, AsHours, AsMinutes, AsSeconds};
    use crate::HumanReadableDuration;
    use std::str::FromStr;

    #[test]
    fn from_u32_works() {
        let representation = HumanReadableDuration::from(300 as u32);
        assert_eq!(300, representation.as_seconds());
        assert_eq!(5, representation.as_minutes());
    }

    #[test]
    fn from_u64_works() {
        let representation = HumanReadableDuration::from(300 as u64);
        assert_eq!(300, representation.as_seconds());
        assert_eq!(5, representation.as_minutes());
    }

    #[test]
    fn from_str_with_empty_string_will_be_handled_gracefully() {
        let representation = HumanReadableDuration::from_str("");
        assert_eq!(true, representation.is_err());
    }

    #[test]
    fn from_str_10_s_will_be_handled_gracefully() {
        let representation = HumanReadableDuration::from_str("10 s");
        assert_eq!(true, representation.is_err());
    }

    #[test]
    fn from_str_10s_works() {
        let representation = HumanReadableDuration::from_str("10s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(10, representation.as_ref().unwrap().as_seconds());
        assert_eq!(0, representation.as_ref().unwrap().as_minutes());
    }

    #[test]
    fn from_str_60s_works() {
        let representation = HumanReadableDuration::from_str("60s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(60, representation.as_ref().unwrap().as_seconds());
        assert_eq!(1, representation.as_ref().unwrap().as_minutes());
    }

    #[test]
    fn from_str_61s_works() {
        let representation = HumanReadableDuration::from_str("61s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(61, representation.as_ref().unwrap().as_seconds());
        assert_eq!(1, representation.as_ref().unwrap().as_minutes());
    }

    #[test]
    fn from_str_5_m_will_be_handled_gracefully() {
        let representation = HumanReadableDuration::from_str("5 m");
        assert_eq!(true, representation.is_err());
    }

    #[test]
    fn from_str_5m_works() {
        let representation = HumanReadableDuration::from_str("5m");
        assert_eq!(true, representation.is_ok());
        assert_eq!(300, representation.as_ref().unwrap().as_seconds());
        assert_eq!(5, representation.as_ref().unwrap().as_minutes());
    }

    #[test]
    fn from_str_60m_works() {
        let representation = HumanReadableDuration::from_str("60m");
        assert_eq!(true, representation.is_ok());
        assert_eq!(3600, representation.as_ref().unwrap().as_seconds());
        assert_eq!(60, representation.as_ref().unwrap().as_minutes());
        assert_eq!(1, representation.as_ref().unwrap().as_hours());
    }

    #[test]
    fn from_str_61m_works() {
        let representation = HumanReadableDuration::from_str("61m");
        assert_eq!(true, representation.is_ok());
        assert_eq!(3660, representation.as_ref().unwrap().as_seconds());
        assert_eq!(61, representation.as_ref().unwrap().as_minutes());
        assert_eq!(1, representation.as_ref().unwrap().as_hours());
    }

    #[test]
    fn from_str_5_h_will_be_handled_gracefully() {
        let representation = HumanReadableDuration::from_str("5 h");
        assert_eq!(true, representation.is_err());
    }

    #[test]
    fn from_str_5h_works() {
        let representation = HumanReadableDuration::from_str("5h");
        assert_eq!(true, representation.is_ok());
        assert_eq!(18000, representation.as_ref().unwrap().as_seconds());
        assert_eq!(300, representation.as_ref().unwrap().as_minutes());
        assert_eq!(5, representation.as_ref().unwrap().as_hours());
    }

    #[test]
    fn from_str_24h_works() {
        let representation = HumanReadableDuration::from_str("24h");
        assert_eq!(true, representation.is_ok());
        assert_eq!(86400, representation.as_ref().unwrap().as_seconds());
        assert_eq!(1440, representation.as_ref().unwrap().as_minutes());
        assert_eq!(24, representation.as_ref().unwrap().as_hours());
    }

    #[test]
    fn from_str_25h_works() {
        let representation = HumanReadableDuration::from_str("25h");
        assert_eq!(true, representation.is_ok());
        assert_eq!(90000, representation.as_ref().unwrap().as_seconds());
        assert_eq!(1500, representation.as_ref().unwrap().as_minutes());
        assert_eq!(25, representation.as_ref().unwrap().as_hours());
    }

    #[test]
    fn from_str_5_d_will_be_handled_gracefully() {
        let representation = HumanReadableDuration::from_str("5 d");
        assert_eq!(true, representation.is_err());
    }

    #[test]
    fn from_str_5d_works() {
        let representation = HumanReadableDuration::from_str("5d");
        assert_eq!(true, representation.is_ok());
        assert_eq!(432000, representation.as_ref().unwrap().as_seconds());
        assert_eq!(7200, representation.as_ref().unwrap().as_minutes());
        assert_eq!(120, representation.as_ref().unwrap().as_hours());
        assert_eq!(5, representation.as_ref().unwrap().as_days());
    }

    #[test]
    fn from_str_32d_works() {
        let representation = HumanReadableDuration::from_str("32d");
        assert_eq!(true, representation.is_ok());
        assert_eq!(2764800, representation.as_ref().unwrap().as_seconds());
        assert_eq!(46080, representation.as_ref().unwrap().as_minutes());
        assert_eq!(768, representation.as_ref().unwrap().as_hours());
        assert_eq!(32, representation.as_ref().unwrap().as_days());
    }

    #[test]
    fn from_str_4_m_10_s_will_be_handled_gracefully() {
        let representation = HumanReadableDuration::from_str("4 m 10 s");
        assert_eq!(true, representation.is_err());
    }

    #[test]
    fn from_str_4m_10s_works() {
        let representation = HumanReadableDuration::from_str("4m 10s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(250, representation.as_ref().unwrap().as_seconds());
        assert_eq!(4, representation.as_ref().unwrap().as_minutes());
    }

    #[test]
    fn from_str_4m10s_works() {
        let representation = HumanReadableDuration::from_str("4m10s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(250, representation.as_ref().unwrap().as_seconds());
        assert_eq!(4, representation.as_ref().unwrap().as_minutes());
    }

    #[test]
    fn from_str_3m60s_works() {
        let representation = HumanReadableDuration::from_str("3m60s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(240, representation.as_ref().unwrap().as_seconds());
        assert_eq!(4, representation.as_ref().unwrap().as_minutes());
    }

    #[test]
    fn from_str_3m61s_works() {
        let representation = HumanReadableDuration::from_str("3m61s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(241, representation.as_ref().unwrap().as_seconds());
        assert_eq!(4, representation.as_ref().unwrap().as_minutes());
    }
}
