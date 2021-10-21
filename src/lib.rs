use std::convert::TryFrom;

/// A data structure for parsing and managing a human readable duration representation
pub struct HumanReadableDuration {
    time_in_seconds: u64,
}

impl HumanReadableDuration {
    /// Get the duration time in seconds
    ///
    /// # Example
    /// ```
    /// use std::convert::TryFrom;
    /// use human_readable_time::HumanReadableDuration;
    ///
    /// let duration = HumanReadableDuration::try_from("10s");
    ///
    /// assert_eq!(10, duration.unwrap().seconds());
    /// ```
    pub fn seconds(&self) -> u64 {
        self.time_in_seconds
    }
}

impl TryFrom<&str> for HumanReadableDuration {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(HumanReadableDuration {
            time_in_seconds: value.split("s").next().unwrap().parse::<u64>().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::HumanReadableDuration;
    use std::convert::TryFrom;

    #[test]
    fn from_10s_works() {
        let representation = HumanReadableDuration::try_from("10s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(10, representation.unwrap().seconds());
    }

    #[test]
    fn from_60s_works() {
        let representation = HumanReadableDuration::try_from("60");
        assert_eq!(true, representation.is_ok());
        assert_eq!(60, representation.unwrap().seconds());
    }

    #[test]
    fn from_61s_works() {
        let representation = HumanReadableDuration::try_from("61s");
        assert_eq!(true, representation.is_ok());
        assert_eq!(61, representation.unwrap().seconds());
    }
}
