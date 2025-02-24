// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p>The request could not be processed because of conflict in the current state of the resource.</p>
    ConflictException(crate::types::error::ConflictException),
    /// <p>There was an internal server error.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>The input requested a resource that was not found.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::cancel_zonal_shift::CancelZonalShiftError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::cancel_zonal_shift::CancelZonalShiftError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::cancel_zonal_shift::CancelZonalShiftError> for Error {
    fn from(err: crate::operation::cancel_zonal_shift::CancelZonalShiftError) -> Self {
        match err {
            crate::operation::cancel_zonal_shift::CancelZonalShiftError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::cancel_zonal_shift::CancelZonalShiftError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::cancel_zonal_shift::CancelZonalShiftError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::cancel_zonal_shift::CancelZonalShiftError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::cancel_zonal_shift::CancelZonalShiftError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::cancel_zonal_shift::CancelZonalShiftError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::cancel_zonal_shift::CancelZonalShiftError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::get_managed_resource::GetManagedResourceError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::get_managed_resource::GetManagedResourceError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::get_managed_resource::GetManagedResourceError> for Error {
    fn from(err: crate::operation::get_managed_resource::GetManagedResourceError) -> Self {
        match err {
            crate::operation::get_managed_resource::GetManagedResourceError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::get_managed_resource::GetManagedResourceError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_managed_resource::GetManagedResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_managed_resource::GetManagedResourceError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::get_managed_resource::GetManagedResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_managed_resource::GetManagedResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::list_managed_resources::ListManagedResourcesError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::list_managed_resources::ListManagedResourcesError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_managed_resources::ListManagedResourcesError> for Error {
    fn from(err: crate::operation::list_managed_resources::ListManagedResourcesError) -> Self {
        match err {
            crate::operation::list_managed_resources::ListManagedResourcesError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::list_managed_resources::ListManagedResourcesError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_managed_resources::ListManagedResourcesError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_managed_resources::ListManagedResourcesError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_managed_resources::ListManagedResourcesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::list_zonal_shifts::ListZonalShiftsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::list_zonal_shifts::ListZonalShiftsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_zonal_shifts::ListZonalShiftsError> for Error {
    fn from(err: crate::operation::list_zonal_shifts::ListZonalShiftsError) -> Self {
        match err {
            crate::operation::list_zonal_shifts::ListZonalShiftsError::AccessDeniedException(
                inner,
            ) => Error::AccessDeniedException(inner),
            crate::operation::list_zonal_shifts::ListZonalShiftsError::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::operation::list_zonal_shifts::ListZonalShiftsError::ThrottlingException(
                inner,
            ) => Error::ThrottlingException(inner),
            crate::operation::list_zonal_shifts::ListZonalShiftsError::ValidationException(
                inner,
            ) => Error::ValidationException(inner),
            crate::operation::list_zonal_shifts::ListZonalShiftsError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::start_zonal_shift::StartZonalShiftError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::start_zonal_shift::StartZonalShiftError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::start_zonal_shift::StartZonalShiftError> for Error {
    fn from(err: crate::operation::start_zonal_shift::StartZonalShiftError) -> Self {
        match err {
            crate::operation::start_zonal_shift::StartZonalShiftError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::start_zonal_shift::StartZonalShiftError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::start_zonal_shift::StartZonalShiftError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::start_zonal_shift::StartZonalShiftError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::start_zonal_shift::StartZonalShiftError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::start_zonal_shift::StartZonalShiftError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::start_zonal_shift::StartZonalShiftError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::update_zonal_shift::UpdateZonalShiftError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::update_zonal_shift::UpdateZonalShiftError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::update_zonal_shift::UpdateZonalShiftError> for Error {
    fn from(err: crate::operation::update_zonal_shift::UpdateZonalShiftError) -> Self {
        match err {
            crate::operation::update_zonal_shift::UpdateZonalShiftError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::update_zonal_shift::UpdateZonalShiftError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::update_zonal_shift::UpdateZonalShiftError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::update_zonal_shift::UpdateZonalShiftError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::update_zonal_shift::UpdateZonalShiftError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::update_zonal_shift::UpdateZonalShiftError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::update_zonal_shift::UpdateZonalShiftError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl std::error::Error for Error {}
impl aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.request_id(),
            Self::ConflictException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
