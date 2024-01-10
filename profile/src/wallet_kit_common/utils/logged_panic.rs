/// Panics and logs with error the `reason` (with file/line context.)
pub fn log_panic(prefix: &str, provided_reason: impl AsRef<str>) {
    let msg = format!(
        "{}: '{}' ({}:{}:{})",
        prefix,
        provided_reason.as_ref(),
        file!(),
        line!(),
        column!()
    );
    log::error!("{}", msg);
    panic!("{}", msg);
}

pub fn incorrect_impl(reason: impl AsRef<str>) {
    log_panic("Incorrect implementation", reason)
}

pub fn fatal_error(reason: impl AsRef<str>) {
    log_panic("Fatal error", reason)
}
