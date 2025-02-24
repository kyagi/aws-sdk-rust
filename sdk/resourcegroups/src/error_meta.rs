// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequestException(crate::types::error::BadRequestException),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    ForbiddenException(crate::types::error::ForbiddenException),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerErrorException(crate::types::error::InternalServerErrorException),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowedException(crate::types::error::MethodNotAllowedException),
    /// <p>One or more of the specified resources don't exist.</p>
    NotFoundException(crate::types::error::NotFoundException),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
    TooManyRequestsException(crate::types::error::TooManyRequestsException),
    /// <p>The request was rejected because it doesn't have valid credentials for the target resource.</p>
    UnauthorizedException(crate::types::error::UnauthorizedException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::ForbiddenException(inner) => inner.fmt(f),
            Error::InternalServerErrorException(inner) => inner.fmt(f),
            Error::MethodNotAllowedException(inner) => inner.fmt(f),
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::TooManyRequestsException(inner) => inner.fmt(f),
            Error::UnauthorizedException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::create_group::CreateGroupError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::create_group::CreateGroupError, R>,
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
impl From<crate::operation::create_group::CreateGroupError> for Error {
    fn from(err: crate::operation::create_group::CreateGroupError) -> Self {
        match err {
            crate::operation::create_group::CreateGroupError::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::operation::create_group::CreateGroupError::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::operation::create_group::CreateGroupError::InternalServerErrorException(
                inner,
            ) => Error::InternalServerErrorException(inner),
            crate::operation::create_group::CreateGroupError::MethodNotAllowedException(inner) => {
                Error::MethodNotAllowedException(inner)
            }
            crate::operation::create_group::CreateGroupError::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::operation::create_group::CreateGroupError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::delete_group::DeleteGroupError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::delete_group::DeleteGroupError, R>,
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
impl From<crate::operation::delete_group::DeleteGroupError> for Error {
    fn from(err: crate::operation::delete_group::DeleteGroupError) -> Self {
        match err {
            crate::operation::delete_group::DeleteGroupError::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::operation::delete_group::DeleteGroupError::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::operation::delete_group::DeleteGroupError::InternalServerErrorException(
                inner,
            ) => Error::InternalServerErrorException(inner),
            crate::operation::delete_group::DeleteGroupError::MethodNotAllowedException(inner) => {
                Error::MethodNotAllowedException(inner)
            }
            crate::operation::delete_group::DeleteGroupError::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::operation::delete_group::DeleteGroupError::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::operation::delete_group::DeleteGroupError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::get_account_settings::GetAccountSettingsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::get_account_settings::GetAccountSettingsError,
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
impl From<crate::operation::get_account_settings::GetAccountSettingsError> for Error {
    fn from(err: crate::operation::get_account_settings::GetAccountSettingsError) -> Self {
        match err {
            crate::operation::get_account_settings::GetAccountSettingsError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::get_account_settings::GetAccountSettingsError::ForbiddenException(inner) => Error::ForbiddenException(inner),
            crate::operation::get_account_settings::GetAccountSettingsError::InternalServerErrorException(inner) => Error::InternalServerErrorException(inner),
            crate::operation::get_account_settings::GetAccountSettingsError::MethodNotAllowedException(inner) => Error::MethodNotAllowedException(inner),
            crate::operation::get_account_settings::GetAccountSettingsError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::operation::get_account_settings::GetAccountSettingsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::get_group::GetGroupError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::get_group::GetGroupError, R>,
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
impl From<crate::operation::get_group::GetGroupError> for Error {
    fn from(err: crate::operation::get_group::GetGroupError) -> Self {
        match err {
            crate::operation::get_group::GetGroupError::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::operation::get_group::GetGroupError::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::operation::get_group::GetGroupError::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::operation::get_group::GetGroupError::MethodNotAllowedException(inner) => {
                Error::MethodNotAllowedException(inner)
            }
            crate::operation::get_group::GetGroupError::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::operation::get_group::GetGroupError::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::operation::get_group::GetGroupError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::get_group_configuration::GetGroupConfigurationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::get_group_configuration::GetGroupConfigurationError,
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
impl From<crate::operation::get_group_configuration::GetGroupConfigurationError> for Error {
    fn from(err: crate::operation::get_group_configuration::GetGroupConfigurationError) -> Self {
        match err {
            crate::operation::get_group_configuration::GetGroupConfigurationError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::get_group_configuration::GetGroupConfigurationError::ForbiddenException(inner) => Error::ForbiddenException(inner),
            crate::operation::get_group_configuration::GetGroupConfigurationError::InternalServerErrorException(inner) => Error::InternalServerErrorException(inner),
            crate::operation::get_group_configuration::GetGroupConfigurationError::MethodNotAllowedException(inner) => Error::MethodNotAllowedException(inner),
            crate::operation::get_group_configuration::GetGroupConfigurationError::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::operation::get_group_configuration::GetGroupConfigurationError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::operation::get_group_configuration::GetGroupConfigurationError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<crate::operation::get_group_query::GetGroupQueryError, R>,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::get_group_query::GetGroupQueryError,
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
impl From<crate::operation::get_group_query::GetGroupQueryError> for Error {
    fn from(err: crate::operation::get_group_query::GetGroupQueryError) -> Self {
        match err {
            crate::operation::get_group_query::GetGroupQueryError::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::operation::get_group_query::GetGroupQueryError::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::operation::get_group_query::GetGroupQueryError::InternalServerErrorException(
                inner,
            ) => Error::InternalServerErrorException(inner),
            crate::operation::get_group_query::GetGroupQueryError::MethodNotAllowedException(
                inner,
            ) => Error::MethodNotAllowedException(inner),
            crate::operation::get_group_query::GetGroupQueryError::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::operation::get_group_query::GetGroupQueryError::TooManyRequestsException(
                inner,
            ) => Error::TooManyRequestsException(inner),
            crate::operation::get_group_query::GetGroupQueryError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::get_tags::GetTagsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::get_tags::GetTagsError, R>,
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
impl From<crate::operation::get_tags::GetTagsError> for Error {
    fn from(err: crate::operation::get_tags::GetTagsError) -> Self {
        match err {
            crate::operation::get_tags::GetTagsError::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::operation::get_tags::GetTagsError::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::operation::get_tags::GetTagsError::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::operation::get_tags::GetTagsError::MethodNotAllowedException(inner) => {
                Error::MethodNotAllowedException(inner)
            }
            crate::operation::get_tags::GetTagsError::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::operation::get_tags::GetTagsError::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::operation::get_tags::GetTagsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::group_resources::GroupResourcesError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::group_resources::GroupResourcesError,
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
impl From<crate::operation::group_resources::GroupResourcesError> for Error {
    fn from(err: crate::operation::group_resources::GroupResourcesError) -> Self {
        match err {
            crate::operation::group_resources::GroupResourcesError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::group_resources::GroupResourcesError::ForbiddenException(inner) => Error::ForbiddenException(inner),
            crate::operation::group_resources::GroupResourcesError::InternalServerErrorException(inner) => Error::InternalServerErrorException(inner),
            crate::operation::group_resources::GroupResourcesError::MethodNotAllowedException(inner) => Error::MethodNotAllowedException(inner),
            crate::operation::group_resources::GroupResourcesError::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::operation::group_resources::GroupResourcesError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::operation::group_resources::GroupResourcesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::list_group_resources::ListGroupResourcesError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::list_group_resources::ListGroupResourcesError,
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
impl From<crate::operation::list_group_resources::ListGroupResourcesError> for Error {
    fn from(err: crate::operation::list_group_resources::ListGroupResourcesError) -> Self {
        match err {
            crate::operation::list_group_resources::ListGroupResourcesError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::list_group_resources::ListGroupResourcesError::ForbiddenException(inner) => Error::ForbiddenException(inner),
            crate::operation::list_group_resources::ListGroupResourcesError::InternalServerErrorException(inner) => Error::InternalServerErrorException(inner),
            crate::operation::list_group_resources::ListGroupResourcesError::MethodNotAllowedException(inner) => Error::MethodNotAllowedException(inner),
            crate::operation::list_group_resources::ListGroupResourcesError::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::operation::list_group_resources::ListGroupResourcesError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::operation::list_group_resources::ListGroupResourcesError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
            crate::operation::list_group_resources::ListGroupResourcesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::list_groups::ListGroupsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::list_groups::ListGroupsError, R>,
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
impl From<crate::operation::list_groups::ListGroupsError> for Error {
    fn from(err: crate::operation::list_groups::ListGroupsError) -> Self {
        match err {
            crate::operation::list_groups::ListGroupsError::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::operation::list_groups::ListGroupsError::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::operation::list_groups::ListGroupsError::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::operation::list_groups::ListGroupsError::MethodNotAllowedException(inner) => {
                Error::MethodNotAllowedException(inner)
            }
            crate::operation::list_groups::ListGroupsError::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::operation::list_groups::ListGroupsError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::put_group_configuration::PutGroupConfigurationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::put_group_configuration::PutGroupConfigurationError,
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
impl From<crate::operation::put_group_configuration::PutGroupConfigurationError> for Error {
    fn from(err: crate::operation::put_group_configuration::PutGroupConfigurationError) -> Self {
        match err {
            crate::operation::put_group_configuration::PutGroupConfigurationError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::put_group_configuration::PutGroupConfigurationError::ForbiddenException(inner) => Error::ForbiddenException(inner),
            crate::operation::put_group_configuration::PutGroupConfigurationError::InternalServerErrorException(inner) => Error::InternalServerErrorException(inner),
            crate::operation::put_group_configuration::PutGroupConfigurationError::MethodNotAllowedException(inner) => Error::MethodNotAllowedException(inner),
            crate::operation::put_group_configuration::PutGroupConfigurationError::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::operation::put_group_configuration::PutGroupConfigurationError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::operation::put_group_configuration::PutGroupConfigurationError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::search_resources::SearchResourcesError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::search_resources::SearchResourcesError,
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
impl From<crate::operation::search_resources::SearchResourcesError> for Error {
    fn from(err: crate::operation::search_resources::SearchResourcesError) -> Self {
        match err {
            crate::operation::search_resources::SearchResourcesError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::search_resources::SearchResourcesError::ForbiddenException(inner) => Error::ForbiddenException(inner),
            crate::operation::search_resources::SearchResourcesError::InternalServerErrorException(inner) => Error::InternalServerErrorException(inner),
            crate::operation::search_resources::SearchResourcesError::MethodNotAllowedException(inner) => Error::MethodNotAllowedException(inner),
            crate::operation::search_resources::SearchResourcesError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::operation::search_resources::SearchResourcesError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
            crate::operation::search_resources::SearchResourcesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::tag::TagError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::operation::tag::TagError, R>) -> Self {
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
impl From<crate::operation::tag::TagError> for Error {
    fn from(err: crate::operation::tag::TagError) -> Self {
        match err {
            crate::operation::tag::TagError::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::operation::tag::TagError::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::operation::tag::TagError::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::operation::tag::TagError::MethodNotAllowedException(inner) => {
                Error::MethodNotAllowedException(inner)
            }
            crate::operation::tag::TagError::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::operation::tag::TagError::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::operation::tag::TagError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::ungroup_resources::UngroupResourcesError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::ungroup_resources::UngroupResourcesError,
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
impl From<crate::operation::ungroup_resources::UngroupResourcesError> for Error {
    fn from(err: crate::operation::ungroup_resources::UngroupResourcesError) -> Self {
        match err {
            crate::operation::ungroup_resources::UngroupResourcesError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::ungroup_resources::UngroupResourcesError::ForbiddenException(inner) => Error::ForbiddenException(inner),
            crate::operation::ungroup_resources::UngroupResourcesError::InternalServerErrorException(inner) => Error::InternalServerErrorException(inner),
            crate::operation::ungroup_resources::UngroupResourcesError::MethodNotAllowedException(inner) => Error::MethodNotAllowedException(inner),
            crate::operation::ungroup_resources::UngroupResourcesError::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::operation::ungroup_resources::UngroupResourcesError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::operation::ungroup_resources::UngroupResourcesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::untag::UntagError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::untag::UntagError, R>,
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
impl From<crate::operation::untag::UntagError> for Error {
    fn from(err: crate::operation::untag::UntagError) -> Self {
        match err {
            crate::operation::untag::UntagError::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::operation::untag::UntagError::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::operation::untag::UntagError::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::operation::untag::UntagError::MethodNotAllowedException(inner) => {
                Error::MethodNotAllowedException(inner)
            }
            crate::operation::untag::UntagError::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::operation::untag::UntagError::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::operation::untag::UntagError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::update_account_settings::UpdateAccountSettingsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::update_account_settings::UpdateAccountSettingsError,
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
impl From<crate::operation::update_account_settings::UpdateAccountSettingsError> for Error {
    fn from(err: crate::operation::update_account_settings::UpdateAccountSettingsError) -> Self {
        match err {
            crate::operation::update_account_settings::UpdateAccountSettingsError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::update_account_settings::UpdateAccountSettingsError::ForbiddenException(inner) => Error::ForbiddenException(inner),
            crate::operation::update_account_settings::UpdateAccountSettingsError::InternalServerErrorException(inner) => Error::InternalServerErrorException(inner),
            crate::operation::update_account_settings::UpdateAccountSettingsError::MethodNotAllowedException(inner) => Error::MethodNotAllowedException(inner),
            crate::operation::update_account_settings::UpdateAccountSettingsError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::operation::update_account_settings::UpdateAccountSettingsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::update_group::UpdateGroupError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::update_group::UpdateGroupError, R>,
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
impl From<crate::operation::update_group::UpdateGroupError> for Error {
    fn from(err: crate::operation::update_group::UpdateGroupError) -> Self {
        match err {
            crate::operation::update_group::UpdateGroupError::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::operation::update_group::UpdateGroupError::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::operation::update_group::UpdateGroupError::InternalServerErrorException(
                inner,
            ) => Error::InternalServerErrorException(inner),
            crate::operation::update_group::UpdateGroupError::MethodNotAllowedException(inner) => {
                Error::MethodNotAllowedException(inner)
            }
            crate::operation::update_group::UpdateGroupError::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::operation::update_group::UpdateGroupError::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::operation::update_group::UpdateGroupError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::update_group_query::UpdateGroupQueryError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::update_group_query::UpdateGroupQueryError,
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
impl From<crate::operation::update_group_query::UpdateGroupQueryError> for Error {
    fn from(err: crate::operation::update_group_query::UpdateGroupQueryError) -> Self {
        match err {
            crate::operation::update_group_query::UpdateGroupQueryError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::update_group_query::UpdateGroupQueryError::ForbiddenException(inner) => Error::ForbiddenException(inner),
            crate::operation::update_group_query::UpdateGroupQueryError::InternalServerErrorException(inner) => Error::InternalServerErrorException(inner),
            crate::operation::update_group_query::UpdateGroupQueryError::MethodNotAllowedException(inner) => Error::MethodNotAllowedException(inner),
            crate::operation::update_group_query::UpdateGroupQueryError::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::operation::update_group_query::UpdateGroupQueryError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::operation::update_group_query::UpdateGroupQueryError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl std::error::Error for Error {}
impl aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::BadRequestException(e) => e.request_id(),
            Self::ForbiddenException(e) => e.request_id(),
            Self::InternalServerErrorException(e) => e.request_id(),
            Self::MethodNotAllowedException(e) => e.request_id(),
            Self::NotFoundException(e) => e.request_id(),
            Self::TooManyRequestsException(e) => e.request_id(),
            Self::UnauthorizedException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
