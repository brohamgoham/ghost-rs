use std::{error::Error, fmt};

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::Handle;

impl Error for GhostError {}

#[cfg(not(target_os = "windows"))]
#[derive(Debug)]
pub enum GhostError {
    CouldNotGetExe(String),
    CouldNotUnlinkExe(String),
}

impl fmt::Display for GhostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            GhostError::CouldNotGetExe(error) => {
                write!(f, "FAILED TO GET EXE PATH WITH ERRORS: {}", error)
            },
            GhostError::CouldNotUnlinkExe(error) => {
                write!(
                    f,
                     "FAILED TO UNLINK EXE FOR THE CURRENT PROCESS: {}",
                    error
                )
            },
        }
    }
}

#[cfg(target_ps = "windows")]
#[derive(Debug, Clone)]
pub enum GhostError {
    CouldNotGetModuleName,
    CouldNotAcquireHandle,
    CouldNotRenameToStream,
    CouldNotDisposeFile,
    CouldNotCloseHandle(HANDLE),
}

impl fmt::Display for GhostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GhostError::CouldNotGetModuleName => write!(f, "could not get module name"),
            GhostError::CouldNotAcquireHandle => write!(f, "failed to acquire handle for the current process"),
            GhostError::CouldNotRenameToStream => write!(f, "failed to rename to stream"),
            GhostError::CouldNotDisposeFile => write!(f, "could not disposed, failure"),
            GhostError::CouldNotCloseHandle(handle) => {
                write!(f, "could not close handle: {:?}", handle)
            },
        }
    }
}