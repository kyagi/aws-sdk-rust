// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have permission to perform an action.</p>
    AccessForbidden(crate::error::AccessForbidden),
    /// <p>An internal failure occurred. Try your request again. If the problem persists, contact AWS customer support.</p>
    InternalFailure(crate::error::InternalFailure),
    /// <p>A resource that is required to perform an action was not found.</p>
    ResourceNotFound(crate::error::ResourceNotFound),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(crate::error::ServiceUnavailable),
    /// <p>There was an error validating your request.</p>
    ValidationError(crate::error::ValidationError),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessForbidden(inner) => inner.fmt(f),
            Error::InternalFailure(inner) => inner.fmt(f),
            Error::ResourceNotFound(inner) => inner.fmt(f),
            Error::ServiceUnavailable(inner) => inner.fmt(f),
            Error::ValidationError(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BatchGetRecordError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::BatchGetRecordError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::BatchGetRecordError> for Error {
    fn from(err: crate::error::BatchGetRecordError) -> Self {
        match err.kind {
            crate::error::BatchGetRecordErrorKind::AccessForbidden(inner) => {
                Error::AccessForbidden(inner)
            }
            crate::error::BatchGetRecordErrorKind::InternalFailure(inner) => {
                Error::InternalFailure(inner)
            }
            crate::error::BatchGetRecordErrorKind::ServiceUnavailable(inner) => {
                Error::ServiceUnavailable(inner)
            }
            crate::error::BatchGetRecordErrorKind::ValidationError(inner) => {
                Error::ValidationError(inner)
            }
            crate::error::BatchGetRecordErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteRecordError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteRecordError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteRecordError> for Error {
    fn from(err: crate::error::DeleteRecordError) -> Self {
        match err.kind {
            crate::error::DeleteRecordErrorKind::AccessForbidden(inner) => {
                Error::AccessForbidden(inner)
            }
            crate::error::DeleteRecordErrorKind::InternalFailure(inner) => {
                Error::InternalFailure(inner)
            }
            crate::error::DeleteRecordErrorKind::ServiceUnavailable(inner) => {
                Error::ServiceUnavailable(inner)
            }
            crate::error::DeleteRecordErrorKind::ValidationError(inner) => {
                Error::ValidationError(inner)
            }
            crate::error::DeleteRecordErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetRecordError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetRecordError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetRecordError> for Error {
    fn from(err: crate::error::GetRecordError) -> Self {
        match err.kind {
            crate::error::GetRecordErrorKind::AccessForbidden(inner) => {
                Error::AccessForbidden(inner)
            }
            crate::error::GetRecordErrorKind::InternalFailure(inner) => {
                Error::InternalFailure(inner)
            }
            crate::error::GetRecordErrorKind::ResourceNotFound(inner) => {
                Error::ResourceNotFound(inner)
            }
            crate::error::GetRecordErrorKind::ServiceUnavailable(inner) => {
                Error::ServiceUnavailable(inner)
            }
            crate::error::GetRecordErrorKind::ValidationError(inner) => {
                Error::ValidationError(inner)
            }
            crate::error::GetRecordErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutRecordError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutRecordError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutRecordError> for Error {
    fn from(err: crate::error::PutRecordError) -> Self {
        match err.kind {
            crate::error::PutRecordErrorKind::AccessForbidden(inner) => {
                Error::AccessForbidden(inner)
            }
            crate::error::PutRecordErrorKind::InternalFailure(inner) => {
                Error::InternalFailure(inner)
            }
            crate::error::PutRecordErrorKind::ServiceUnavailable(inner) => {
                Error::ServiceUnavailable(inner)
            }
            crate::error::PutRecordErrorKind::ValidationError(inner) => {
                Error::ValidationError(inner)
            }
            crate::error::PutRecordErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
