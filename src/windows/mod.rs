pub(crate) mod errors;

use errors::GhostError;
use std::{ffi::c_void, mem::size_of, ptr::copy};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{CloseHandle, BOOLEAN, HANDLE, HINSTANCE, MAX_PATH},
        Storage::FileSystem::{
            CreateFileW, FileDispositionInfo, FileRenameInfo, SetFileInformationByHandle, DELETE,
            FILE_ATTRIBUTE_NORMAL, FILE_DISPOSITION_INFO, FILE_RENAME_INFO, FILE_RENAME_INFO_0,
            FILE_SHARE_NONE, OPEN_EXISTING,
        },
        System::LibraryLoader::GetModuleFileNameW,
    },
};

pub(crate) const DEFAULT_PLACEHOLDER: &[u8; 9] = b":svcmsrpc";

pub(crate) fn ninja(placeholder: &[u8; 9]) -> Result<(), GhostError> {
    let filename = get_filename()?;
    let handle = open(&filename)?;

    #[cfg(feature = "debug")]
    println!("[*] Attempting to rename file to stream");
    rename(placeholder, handle)?;

    #[cfg(feature = "debug")]
    println!("[*] SUCCESSFULLY RENAMED FILE PRIMARY: $DATA ADS TO SPECIFIED STREAM, CLOSING HANDLE");
    close(handle);

    let handle = open(&filename);

    dispose(handle)?;

    #[cfg(feature = "debug")]
    println!("[*]CLOSING HANDLE TO TRIGGER DELETION DEPOSITION");
    close(handle)?;

    Ok(())
}

fn open(path: &str) -> Result<HANDLE, GhostError> {
    let os_path: Vec<u16> = path.encode_utf16().collect();

    return match unsafe {
        CreateFileW(
            PCWSTR::from_raw(os_path.as_ptr()),
            DELETE,
            FILE_SHARE_NONE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            HANDLE::default(),
        )
    } {
        Ok(handle) => {
            #[cfg(feature = "debug")]
            println!("[*]  ACQUIRED HANDLE: {:?}", handle);

            Ok(handle)
        },
        Err(_) => Err(GhostError::CouldNotAcquireHandle),
    };
}

fn rename(placeholder: &[u8; 9], handle: HANDLE) -> Result<(), GhostError> {
    let filename = placeholder.map(|b| b as u16);
    let length = size_of::<[u16; 9]>();

    let mut file_rename_info: FILE_RENAME_INFO = FILE_RENAME_INFO {
        Anonymous: FILE_RENAME_INFO_0 {
            ReplaceIfExists: B(0),
        },
        RootDirectory: HANDLE::default(),
        FileNameLength: length as u32,
        FileName: [0],
    };
    unsafe {
        copy(
            filename.as_ptr(),
            file_rename_info,FileName.as_mut_ptr(),
            length,
        )
    };

    let buffer_size = size_of::<[u16; 9]>() + size_of::<FILE_RENAME_INFO>();

    match unsafe {
        SetFileInformationByHandle(
            handle,
            FileRenameInfo,
            &file_rename_info as *const _ as *const c_void,
            buffer_size as u32,
        )
        .ok()
    } {
        Ok() => Ok(()),
        Err(_) => Err(GhostError::CouldNotRenameToStream)
    }
}

fn dispose(handle: HANDLE) -> Result<(), GhostError> {
    let mut file_delete: FILE_DISPOSITION_INFO = FILE_DISPOSITION_INFO {
        DeleteFile: BOOLEAN(1),
    };

    match unsafe {
        SetFileInformationByHandle(
            handle,
            FileDispositionInfo,
            &file_delete as *const _ as *const c_void,
            size_of::<FILE_DISPOSITION_INFO>() as u32,
        )
        .ok()
    } {
        Ok(_) => Ok(()),
        Err(_) => Err(GhostError::CouldNotDisposeFile),
    }
}

fn close(handle: HANDLE) -> Result<(), GhostError> {
    match unsafe {CloseHandle(handle).ok() } {
        Ok(_) => Ok(()),
        Err(_) => Err(GhostError::CouldNotCloseHandle(handle)),
    }
}

fn get_filename() -> Result<String, GhostError> {
    let mut filename_buffer: &mut [u16] = &mut [0u16; MAX_PATH as usize];

    let filename_length = unsafe {
        GetModuleFileNameW(HINSTANCE(0), filename_buffer)
    };

    let len = filename_buffer.iter().take_while(|&&c| c!= 0).count();
    let filename = String::from_utf16_lossy(&filename_buffer[..len]);

    #[cfg(feature = "debug")]
    {
        println!("[*] FILENAME: {:?}", &filename);
        println!("[*] FILENAME LENGTH: {:?}", filename_length);
    }

    if filename.is_empty() {
        Err(GhostError::CouldNotFindFile)
    } else {
        Ok(filename)
    }
}