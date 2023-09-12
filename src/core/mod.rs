pub mod ev_handler;
pub mod events;
#[cfg(any(feature = "dynamic_output", feature = "static_output"))]
pub mod init;
#[cfg(feature = "search")]
pub mod search;
pub mod utils;

/// Define the modes in which minus can run
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RunMode {
    #[cfg(feature = "static_output")]
    Static,
    #[cfg(feature = "dynamic_output")]
    Dynamic,
    Uninitialized,
}

impl RunMode {
    /// Returns true if minus hasn't started
    ///
    /// # Example
    /// ```
    /// use minus::RunMode;
    ///
    /// let runmode = RunMode::Uninitialized;
    /// assert_eq!(runmode.is_uninitialized(), true);
    /// ```
    pub fn is_uninitialized(self) -> bool {
        self == Self::Uninitialized
    }
}
