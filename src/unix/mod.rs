pub(crate) mod errors;

use errors::GhostError;
use std::{env::current_exe, fs::remove_file};

pub fn ninja() -> Result<(), GhostError> {
    #[cfg(feature = "debug")]
    println!("[*] Getting current exe");
    let filename = match current_exe() {
        Ok(p) => p,
        Err(e) => return Err(GhostError::CouldNotGetExe(e.to_string()))
    };

    #[cfg(feature = "debug")]
    println!("  > {:?}", filename.clone().into_os_string());

    #[cfg(feature = "debug")]
    println!("[*] Attempting to unlink file");
    return match remove_file(filename) {
        Ok(_) => Ok(()),
        Err(e) => Err(GhostError::CouldNotUnlinkExe(e.to_string())),
    };
}