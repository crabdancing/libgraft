use std::{
    error::Error,
    path::{Path, PathBuf},
};

pub trait GraftPath {
    fn ensure_exists(&self) -> Result<&Self, std::io::Error>;
}

pub trait GraftPathBuf
where
    Self: Sized,
{
    fn ensure_exists(self) -> Result<Self, std::io::Error>;
}

impl GraftPathBuf for PathBuf {
    fn ensure_exists(self) -> Result<Self, std::io::Error> {
        if self.is_file() {
            Err(std::io::Error::new(
                std::io::ErrorKind::IsADirectory,
                format!(
                    "Tried to ensure directory path exists: `{}`\n But this is a file!",
                    self.display()
                ),
            ))
        } else {
            std::fs::create_dir_all(&self)?;
            Ok(self)
        }
    }
}

impl GraftPath for Path {
    fn ensure_exists(&self) -> Result<&Self, std::io::Error> {
        if self.is_file() {
            Err(std::io::Error::new(
                std::io::ErrorKind::IsADirectory,
                format!(
                    "Tried to ensure directory path exists: `{}`\n But this is a file!",
                    self.display()
                ),
            ))
        } else {
            std::fs::create_dir_all(self)?;
            Ok(self)
        }
    }
}
