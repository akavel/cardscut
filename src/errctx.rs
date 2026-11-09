use easy_error::ResultExt;
use std::string::ToString;

// A small helper type to shorten the way context is added to errors.
pub trait Ctx<T> {
    #[allow(dead_code)]
    #[track_caller]
    fn ctx<S: ToString>(self, ctx: S) -> Result<T, easy_error::Error>;

    #[allow(dead_code)]
    #[track_caller]
    fn ctxf<S, F>(self, ctxf: F) -> Result<T, easy_error::Error>
    where
        S: ToString,
        F: FnOnce() -> S;
}

impl<T, E> Ctx<T> for Result<T, E>
where
    E: std::error::Error + Send + 'static,
{
    fn ctx<S: ToString>(self, ctx: S) -> Result<T, easy_error::Error> {
        self.context(ctx)
    }

    fn ctxf<S, F>(self, ctxf: F) -> Result<T, easy_error::Error>
    where
        S: ToString,
        F: FnOnce() -> S,
    {
        self.with_context(ctxf)
    }
}
