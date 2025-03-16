use log::warn;

pub trait WarnError {
    fn warn(self) -> Self;
    fn warn_msg(self, warning: impl AsRef<str>) -> Self;
    fn warn_drop(self);
    fn warn_is_err(self) -> bool;
}

impl<T, E> WarnError for std::result::Result<T, E>
where
    E: std::fmt::Display,
{
    fn warn(self) -> Self {
        if let Err(e) = self.as_ref() {
            warn!("{}", &e);
        }
        self
    }

    fn warn_drop(self) {
        if let Err(e) = self.as_ref() {
            warn!("{}", e);
        }
    }

    fn warn_is_err(self) -> bool {
        if let Err(e) = self {
            warn!("{}", &e);
            true
        } else {
            false
        }
    }

    fn warn_msg(self, warning: impl AsRef<str>) -> Self {
        let warning: &str = warning.as_ref();
        if let Err(e) = self.as_ref() {
            warn!("{} ({})", warning, &e);
        }
        self
    }
}
