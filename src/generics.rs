//! Internal useage only.
//! Generics

use super::*;

pub(crate) fn cvt_path(path: &Path) -> Result<PCSTR, WinError> {
        let path = path.as_os_str();
        if !path.is_ascii() {
                return Err(WinError::new(
                        ERROR_INVALID_NAME,
                        ::windows::w!("invalid named pipe path, must be valid ascii").to_owned(),
                ));
        }
        if path.len() >= MAX_PATH as _ {
                return Err(WinError::new(
                        ERROR_INVALID_NAME,
                        ::windows::w!("invalid named pipe path, must be less than 260 characters").to_owned(),
                ));
        }
        let mut path = path.to_str().unwrap().to_string();
        path.push('\0');
        Ok(PCSTR(path.as_ptr()))
}
