// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl CreatePullRequestApprovalRuleInput {
    /// Consumes the builder and constructs an Operation<[`CreatePullRequestApprovalRule`](crate::operation::create_pull_request_approval_rule::CreatePullRequestApprovalRule)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::create_pull_request_approval_rule::CreatePullRequestApprovalRule,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(_config.use_dual_stack)
            .set_use_fips(_config.use_fips)
            .set_endpoint(_config.endpoint_url.clone())
            .build()
            .map_err(|err| {
                aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                Some(params),
            ),
            Err(e) => (Err(e), None),
        };
        let mut request = {
            fn uri_base(
                _input: &crate::operation::create_pull_request_approval_rule::CreatePullRequestApprovalRuleInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_pull_request_approval_rule::CreatePullRequestApprovalRuleInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "CodeCommit_20150413.CreatePullRequestApprovalRule",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_create_pull_request_approval_rule::ser_create_pull_request_approval_rule_input(&self)?
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::meta::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_credentials_cache(
            &mut request.properties_mut(),
            _config.credentials_cache.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::create_pull_request_approval_rule::CreatePullRequestApprovalRule::new(
            ),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "CreatePullRequestApprovalRule",
            "codecommit",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `CreatePullRequestApprovalRule`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct CreatePullRequestApprovalRule;
impl CreatePullRequestApprovalRule {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreatePullRequestApprovalRule {
    type Output = std::result::Result<
        crate::operation::create_pull_request_approval_rule::CreatePullRequestApprovalRuleOutput,
        crate::operation::create_pull_request_approval_rule::CreatePullRequestApprovalRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_create_pull_request_approval_rule::de_create_pull_request_approval_rule_http_error(response)
        } else {
            crate::protocol_serde::shape_create_pull_request_approval_rule::de_create_pull_request_approval_rule_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type CreatePullRequestApprovalRuleErrorKind = CreatePullRequestApprovalRuleError;
/// Error type for the `CreatePullRequestApprovalRuleError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum CreatePullRequestApprovalRuleError {
    /// <p>The content for the approval rule is empty. You must provide some content for an approval rule. The content cannot be null.</p>
    ApprovalRuleContentRequiredException(crate::types::error::ApprovalRuleContentRequiredException),
    /// <p>An approval rule with that name already exists. Approval rule names must be unique within the scope of a pull request.</p>
    ApprovalRuleNameAlreadyExistsException(
        crate::types::error::ApprovalRuleNameAlreadyExistsException,
    ),
    /// <p>An approval rule name is required, but was not specified.</p>
    ApprovalRuleNameRequiredException(crate::types::error::ApprovalRuleNameRequiredException),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailedException(
        crate::types::error::EncryptionIntegrityChecksFailedException,
    ),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDeniedException(crate::types::error::EncryptionKeyAccessDeniedException),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabledException(crate::types::error::EncryptionKeyDisabledException),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFoundException(crate::types::error::EncryptionKeyNotFoundException),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailableException(crate::types::error::EncryptionKeyUnavailableException),
    /// <p>The content for the approval rule is not valid.</p>
    InvalidApprovalRuleContentException(crate::types::error::InvalidApprovalRuleContentException),
    /// <p>The name for the approval rule is not valid.</p>
    InvalidApprovalRuleNameException(crate::types::error::InvalidApprovalRuleNameException),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestIdException(crate::types::error::InvalidPullRequestIdException),
    /// <p>The approval rule cannot be added. The pull request has the maximum number of approval rules associated with it.</p>
    NumberOfRulesExceededException(crate::types::error::NumberOfRulesExceededException),
    /// <p>The pull request status cannot be updated because it is already closed.</p>
    PullRequestAlreadyClosedException(crate::types::error::PullRequestAlreadyClosedException),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExistException(crate::types::error::PullRequestDoesNotExistException),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequiredException(crate::types::error::PullRequestIdRequiredException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for CreatePullRequestApprovalRuleError {
    fn create_unhandled_error(
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled({
            let mut builder = aws_smithy_types::error::Unhandled::builder().source(source);
            builder.set_meta(meta);
            builder.build()
        })
    }
}
impl std::fmt::Display for CreatePullRequestApprovalRuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApprovalRuleContentRequiredException(_inner) => _inner.fmt(f),
            Self::ApprovalRuleNameAlreadyExistsException(_inner) => _inner.fmt(f),
            Self::ApprovalRuleNameRequiredException(_inner) => _inner.fmt(f),
            Self::EncryptionIntegrityChecksFailedException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyAccessDeniedException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyDisabledException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyNotFoundException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyUnavailableException(_inner) => _inner.fmt(f),
            Self::InvalidApprovalRuleContentException(_inner) => _inner.fmt(f),
            Self::InvalidApprovalRuleNameException(_inner) => _inner.fmt(f),
            Self::InvalidPullRequestIdException(_inner) => _inner.fmt(f),
            Self::NumberOfRulesExceededException(_inner) => _inner.fmt(f),
            Self::PullRequestAlreadyClosedException(_inner) => _inner.fmt(f),
            Self::PullRequestDoesNotExistException(_inner) => _inner.fmt(f),
            Self::PullRequestIdRequiredException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata
    for CreatePullRequestApprovalRuleError
{
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ApprovalRuleContentRequiredException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ApprovalRuleNameAlreadyExistsException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ApprovalRuleNameRequiredException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionIntegrityChecksFailedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyAccessDeniedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyDisabledException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyNotFoundException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyUnavailableException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidApprovalRuleContentException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidApprovalRuleNameException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidPullRequestIdException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NumberOfRulesExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::PullRequestAlreadyClosedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::PullRequestDoesNotExistException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::PullRequestIdRequiredException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::create_pull_request_approval_rule::CreatePullRequestApprovalRuleError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for CreatePullRequestApprovalRuleError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl CreatePullRequestApprovalRuleError {
    /// Creates the `CreatePullRequestApprovalRuleError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `CreatePullRequestApprovalRuleError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
    pub fn generic(err: aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err.clone())
                .meta(err)
                .build(),
        )
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::ApprovalRuleContentRequiredException(e) => e.meta(),
            Self::ApprovalRuleNameAlreadyExistsException(e) => e.meta(),
            Self::ApprovalRuleNameRequiredException(e) => e.meta(),
            Self::EncryptionIntegrityChecksFailedException(e) => e.meta(),
            Self::EncryptionKeyAccessDeniedException(e) => e.meta(),
            Self::EncryptionKeyDisabledException(e) => e.meta(),
            Self::EncryptionKeyNotFoundException(e) => e.meta(),
            Self::EncryptionKeyUnavailableException(e) => e.meta(),
            Self::InvalidApprovalRuleContentException(e) => e.meta(),
            Self::InvalidApprovalRuleNameException(e) => e.meta(),
            Self::InvalidPullRequestIdException(e) => e.meta(),
            Self::NumberOfRulesExceededException(e) => e.meta(),
            Self::PullRequestAlreadyClosedException(e) => e.meta(),
            Self::PullRequestDoesNotExistException(e) => e.meta(),
            Self::PullRequestIdRequiredException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::ApprovalRuleContentRequiredException`.
    pub fn is_approval_rule_content_required_exception(&self) -> bool {
        matches!(self, Self::ApprovalRuleContentRequiredException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::ApprovalRuleNameAlreadyExistsException`.
    pub fn is_approval_rule_name_already_exists_exception(&self) -> bool {
        matches!(self, Self::ApprovalRuleNameAlreadyExistsException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::ApprovalRuleNameRequiredException`.
    pub fn is_approval_rule_name_required_exception(&self) -> bool {
        matches!(self, Self::ApprovalRuleNameRequiredException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::EncryptionIntegrityChecksFailedException`.
    pub fn is_encryption_integrity_checks_failed_exception(&self) -> bool {
        matches!(self, Self::EncryptionIntegrityChecksFailedException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::EncryptionKeyAccessDeniedException`.
    pub fn is_encryption_key_access_denied_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyAccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::EncryptionKeyDisabledException`.
    pub fn is_encryption_key_disabled_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyDisabledException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::EncryptionKeyNotFoundException`.
    pub fn is_encryption_key_not_found_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyNotFoundException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::EncryptionKeyUnavailableException`.
    pub fn is_encryption_key_unavailable_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyUnavailableException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::InvalidApprovalRuleContentException`.
    pub fn is_invalid_approval_rule_content_exception(&self) -> bool {
        matches!(self, Self::InvalidApprovalRuleContentException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::InvalidApprovalRuleNameException`.
    pub fn is_invalid_approval_rule_name_exception(&self) -> bool {
        matches!(self, Self::InvalidApprovalRuleNameException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::InvalidPullRequestIdException`.
    pub fn is_invalid_pull_request_id_exception(&self) -> bool {
        matches!(self, Self::InvalidPullRequestIdException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::NumberOfRulesExceededException`.
    pub fn is_number_of_rules_exceeded_exception(&self) -> bool {
        matches!(self, Self::NumberOfRulesExceededException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::PullRequestAlreadyClosedException`.
    pub fn is_pull_request_already_closed_exception(&self) -> bool {
        matches!(self, Self::PullRequestAlreadyClosedException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::PullRequestDoesNotExistException`.
    pub fn is_pull_request_does_not_exist_exception(&self) -> bool {
        matches!(self, Self::PullRequestDoesNotExistException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestApprovalRuleError::PullRequestIdRequiredException`.
    pub fn is_pull_request_id_required_exception(&self) -> bool {
        matches!(self, Self::PullRequestIdRequiredException(_))
    }
}
impl std::error::Error for CreatePullRequestApprovalRuleError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ApprovalRuleContentRequiredException(_inner) => Some(_inner),
            Self::ApprovalRuleNameAlreadyExistsException(_inner) => Some(_inner),
            Self::ApprovalRuleNameRequiredException(_inner) => Some(_inner),
            Self::EncryptionIntegrityChecksFailedException(_inner) => Some(_inner),
            Self::EncryptionKeyAccessDeniedException(_inner) => Some(_inner),
            Self::EncryptionKeyDisabledException(_inner) => Some(_inner),
            Self::EncryptionKeyNotFoundException(_inner) => Some(_inner),
            Self::EncryptionKeyUnavailableException(_inner) => Some(_inner),
            Self::InvalidApprovalRuleContentException(_inner) => Some(_inner),
            Self::InvalidApprovalRuleNameException(_inner) => Some(_inner),
            Self::InvalidPullRequestIdException(_inner) => Some(_inner),
            Self::NumberOfRulesExceededException(_inner) => Some(_inner),
            Self::PullRequestAlreadyClosedException(_inner) => Some(_inner),
            Self::PullRequestDoesNotExistException(_inner) => Some(_inner),
            Self::PullRequestIdRequiredException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::create_pull_request_approval_rule::_create_pull_request_approval_rule_output::CreatePullRequestApprovalRuleOutput;

pub use crate::operation::create_pull_request_approval_rule::_create_pull_request_approval_rule_input::CreatePullRequestApprovalRuleInput;

mod _create_pull_request_approval_rule_input;

mod _create_pull_request_approval_rule_output;

/// Builders
pub mod builders;
