/// [`Error`] - More user-friendly way of representing error codes.
pub enum Error {

    /// Successful operation. <br>
    /// Error code: `[0]`
    None,

    /// Tried to operate on a restricted ID. <br>
    /// Error code: `[1]`
    RestrictedID,

    /// Tried to get a pointer from map with not existing, or not extern ID. <br>
    /// Error code: `[2]`
    InvalidID,

    /// Handle to a process was invalid. <br>
    /// Error code: `[3]`
    InvalidHandle,

    /// WinAPI function `ReadProcessMemory` couldn't read any memory. <br>
    /// Error code: `[4]`
    ReadProcessMemoryFailed,

    /// Address to a pointer could not be saved into the registry. <br>
    /// Error code: `[5]`
    AddressNotSaved,
}

/// Converts [Error](Error) enum variants into `u32` error code.
impl Into<u32> for Error {
    fn into(self) -> u32 {
        match self {
            Error::None => 0,
            Error::RestrictedID => 1,
            Error::InvalidID => 2,
            Error::InvalidHandle => 3,
            Error::ReadProcessMemoryFailed => 4,
            Error::AddressNotSaved => 5,
        }
    }
}

/// Converts `u32` error code into [Error](Error) enum variant.
impl From<u32> for Error {
    fn from(ec: u32) -> Self {
        match ec {
            0 => Error::None,
            1 => Error::RestrictedID,
            2 => Error::InvalidID,
            3 => Error::InvalidHandle,
            4 => Error::ReadProcessMemoryFailed,
            5 => Error::AddressNotSaved,
            _ => Error::None,
        }
    }
}
