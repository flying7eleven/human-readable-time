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
}

/// The error which will be returned, if a value could not be parsed into an `HumanReadableDuration`
pub struct ParseHumanReadableDurationError;

impl Debug for ParseHumanReadableDurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseHumanReadableDurationError")
    }
}

impl Display for ParseHumanReadableDurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseHumanReadableDurationError")
    }
}

impl std::error::Error for ParseHumanReadableDurationError {}

impl FromStr for HumanReadableDuration {
    type Err = ParseHumanReadableDurationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(HumanReadableDuration {
            time_in_seconds: value.split("s").next().unwrap().parse::<u64>().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::HumanReadableDuration;
    use std::str::FromStr;

    #[test]
    fn from_str_10s_works() {
        let representation = HumanReadableDuration::from_str("10s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(10, representation.unwrap().seconds());
    }

    #[test]
    fn from_str_60s_works() {
        let representation = HumanReadableDuration::from_str("60");
        assert_eq!(true, representation.is_ok());
        assert_eq!(60, representation.unwrap().seconds());
    }

    #[test]
    fn from_str_61s_works() {
        let representation = HumanReadableDuration::from_str("61s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(61, representation.unwrap().seconds());
    }
}
