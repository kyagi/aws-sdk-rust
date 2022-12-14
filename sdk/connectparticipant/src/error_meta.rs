// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>An attachment with that identifier is already being uploaded.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>This exception occurs when there is an internal failure in the Amazon Connect service.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The number of attachments per contact exceeds the quota.</p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The input fails to satisfy the constraints specified by Amazon Connect.</p>
    ValidationException(crate::error::ValidationException),
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
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CompleteAttachmentUploadError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CompleteAttachmentUploadError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CompleteAttachmentUploadError> for Error {
    fn from(err: crate::error::CompleteAttachmentUploadError) -> Self {
        match err.kind {
            crate::error::CompleteAttachmentUploadErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::CompleteAttachmentUploadErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::CompleteAttachmentUploadErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::CompleteAttachmentUploadErrorKind::ServiceQuotaExceededException(
                inner,
            ) => Error::ServiceQuotaExceededException(inner),
            crate::error::CompleteAttachmentUploadErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::CompleteAttachmentUploadErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::CompleteAttachmentUploadErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateParticipantConnectionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateParticipantConnectionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateParticipantConnectionError> for Error {
    fn from(err: crate::error::CreateParticipantConnectionError) -> Self {
        match err.kind {
            crate::error::CreateParticipantConnectionErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::CreateParticipantConnectionErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::CreateParticipantConnectionErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::CreateParticipantConnectionErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::CreateParticipantConnectionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisconnectParticipantError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DisconnectParticipantError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DisconnectParticipantError> for Error {
    fn from(err: crate::error::DisconnectParticipantError) -> Self {
        match err.kind {
            crate::error::DisconnectParticipantErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DisconnectParticipantErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DisconnectParticipantErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DisconnectParticipantErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DisconnectParticipantErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetAttachmentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetAttachmentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetAttachmentError> for Error {
    fn from(err: crate::error::GetAttachmentError) -> Self {
        match err.kind {
            crate::error::GetAttachmentErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::GetAttachmentErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::GetAttachmentErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetAttachmentErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::GetAttachmentErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetTranscriptError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetTranscriptError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetTranscriptError> for Error {
    fn from(err: crate::error::GetTranscriptError) -> Self {
        match err.kind {
            crate::error::GetTranscriptErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::GetTranscriptErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::GetTranscriptErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetTranscriptErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::GetTranscriptErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SendEventError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::SendEventError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::SendEventError> for Error {
    fn from(err: crate::error::SendEventError) -> Self {
        match err.kind {
            crate::error::SendEventErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::SendEventErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::SendEventErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::SendEventErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::SendEventErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SendMessageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::SendMessageError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::SendMessageError> for Error {
    fn from(err: crate::error::SendMessageError) -> Self {
        match err.kind {
            crate::error::SendMessageErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::SendMessageErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::SendMessageErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::SendMessageErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::SendMessageErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartAttachmentUploadError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartAttachmentUploadError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StartAttachmentUploadError> for Error {
    fn from(err: crate::error::StartAttachmentUploadError) -> Self {
        match err.kind {
            crate::error::StartAttachmentUploadErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::StartAttachmentUploadErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::StartAttachmentUploadErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::StartAttachmentUploadErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::StartAttachmentUploadErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::StartAttachmentUploadErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
