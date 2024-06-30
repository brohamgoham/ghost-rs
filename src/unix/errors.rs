use std::{error::Error, fmt};

/// Gives info as to why the executable cannot
/// be deleted 
#[derive(Debug)]
pub enum GhostError {
    CouldNotGetExe(String),
    CouldNotUnlinkExe(String),
    UnsupportedPlatform,
    CouldNotFindFile(String)
}

impl Error for GhostError {}

impl fmt::Display for GhostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // we use &*self to create a temporary value that can be 
        // passed to the method
        // &*self effectively takes a ref to
        // the value of self, this seems redundant but
        // we want to call a method that takes ownership of the Object
        match &*self {
            GhostError::UnsupportedPlatform => write!(f, "UNSUPPORTED ON THIS PLATFORM"),
            GhostError::CouldNotGetExe(error) => {
                write!(f, "FAILED TO GET EXE PATH ERROR: {}", error)
            },
            GhostError::CouldNotFindFile(error) => {
                write!(f, "Could not find file to dispose: {}", error)
            },
            GhostError::CouldNotUnlinkExe(error) => write!(f, "FAILED TO UNLINK THE CURRENT EXE FOR CURRENT PROCESS: {}", error),
        }
    }
}