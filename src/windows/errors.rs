use std::{
    error::Error,
    fmt,
    fmt::{Display, Formatter}
};
use windows::Win32::Foundation::HANDLE;

#[derive(Debug)]
pub enum GhostError {
    CouldNotGetModuleName,
    CouldNotAcquireHandle,
    CouldNotRenameToStream,
    CouldNotDisposeFile,
    CouldNotFindFile,
    CouldNotCloseHandle(HANDLE),
    UnsupportedPlatform
}

impl Error for GhostError {}

impl Display for GhostError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            GhostError::CouldNotGetModuleName => write!(f, "COULD NOT GET MODULE NAME"),
            GhostError::UnsupportedPlatform => write!(f, "PLATFORM UNSUPPORTED"),
            GhostError::CouldNotAcquireHandle => write!(f, "FAILED TO ACQUIRE HANDLE FOR CURRENT PROCESS"),
            GhostError::CouldNotRenameToStream => write!(f, "COULD NOT RENAME TO STREAM"),
            GhostError::CouldNotDisposeFile => write!(f, "COULD NOT DISPOSE OF FILE"),
            GhostError::CouldNotFindFile => write!("COULD NOT FIND FILE TO DISPOSE!"),
            GhostError::CouldNotCloseHandle(handle) => {
                write!(f, "could not close handle: {:?}", handle)
            },
        }
    }
}