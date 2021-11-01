/// Used to get the number of seconds which represent a specific object which implements this trait.
pub trait AsSeconds {
    /// Get the duration time in seconds
    fn as_seconds(&self) -> u64;
}

/// Used to get the number of full minutes which represent a specific object which implements this trait.
pub trait AsMinutes {
    /// Get the duration time in full minutes
    fn as_minutes(&self) -> u64;
}

/// Used to get the number of full hours which represent a specific object which implements this trait.
pub trait AsHours {
    /// Get the duration time in full hours
    fn as_hours(&self) -> u64;
}

/// Used to get the number of full days which represent a specific object which implements this trait.
pub trait AsDays {
    /// Get the duration time in full hours
    fn as_days(&self) -> u64;
}

/// Used to convert an object to a [`chrono::Duration`]  representation.
#[cfg(feature = "chrono")]
pub trait AsDuration {
    /// Convert the object to a [`chrono::Duration`]  representation.
    fn as_duration(&self) -> chrono::Duration;
}
