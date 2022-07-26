pub trait Abort<T> {
    fn abort(self) -> T;
}

impl<T> Abort<T> for Option<T> {
    #[inline]
    fn abort(self) -> T {
        unwrap_abort_option(self)
    }
}

impl<T, E> Abort<T> for Result<T, E> {
    #[inline]
    fn abort(self) -> T {
        unwrap_abort_result(self)
    }
}

#[inline]
pub fn unwrap_abort_option<T>(o: Option<T>) -> T {
    use std::process;
    match o {
        Some(t) => t,
        None => process::abort(),
    }
}

#[inline]
pub fn unwrap_abort_result<T, E>(o: Result<T, E>) -> T {
    use std::process;
    match o {
        Ok(t) => t,
        Err(_) => process::abort()
    }
}
