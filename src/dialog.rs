use nfd2::error::NfdError;
use nfd2::Response;
use std::path::PathBuf;
use std::result::Result;

#[derive(Debug)]
pub enum OpenDialogError {
    NoFiles,
    SystemError(NfdError),
}

impl From<NfdError> for OpenDialogError {
    fn from(error: NfdError) -> Self {
        OpenDialogError::SystemError(error)
    }
}

impl std::fmt::Display for OpenDialogError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpenDialogError::NoFiles => write!(f, "got no files to open"),
            OpenDialogError::SystemError(error) => error.fmt(f),
        }
    }
}

pub fn open_file() -> Result<PathBuf, OpenDialogError> {
    match nfd2::open_file_dialog(Option::Some("png"), None)? {
        Response::Okay(file_path) => Result::Ok(file_path),
        Response::OkayMultiple(mut files) => {
            println!("Got {:?}, using first one.", files);
            match files.pop() {
                Option::None => Result::Err(OpenDialogError::NoFiles),
                Option::Some(file) => Result::Ok(file),
            }
        }
        Response::Cancel => Result::Err(OpenDialogError::NoFiles),
    }
}
