use crate::prelude::*;

pub trait LoggedResult<T: SafeToLog>: Sized {
    fn log_lvl_formatted<F>(self, level: log::Level, format: F) -> Self
    where
        F: FnOnce(&T) -> String;

    fn log_lvl(self, level: log::Level, prefix: impl AsRef<str>) -> Self {
        self.log_lvl_formatted(level, |f| {
            format!("{} - {:?}", prefix.as_ref(), f.non_sensitive())
        })
    }
    fn log_info(self, prefix: impl AsRef<str>) -> Self {
        self.log_lvl(log::Level::Info, prefix)
    }
}
impl<T: SafeToLog> LoggedResult<T> for Result<T> {
    fn log_lvl_formatted<F>(self, level: log::Level, format: F) -> Self
    where
        F: FnOnce(&T) -> String,
    {
        self.inspect(|x| log::log!(level, "{}", format(&x)))
    }
}
