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
        self.inspect(|x| log::log!(level, "{}", format(x)))
            .inspect_err(|e| log::error!("Err: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn error() {
        #[derive(PartialEq, Debug)]
        struct Foo {
            value: u8,
        }
        impl SafeToLog for Foo {
            fn non_sensitive(&self) -> impl std::fmt::Debug {
                "bar"
            }
        }
        // should not change error
        assert_eq!(
            Err::<Foo, CommonError>(CommonError::Unknown).log_info("test Err"),
            Err(CommonError::Unknown)
        );
        // should not change value
        assert_eq!(
            Ok::<Foo, CommonError>(Foo { value: 42 }).log_info("test Ok"),
            Ok(Foo { value: 42 })
        );
    }
}
