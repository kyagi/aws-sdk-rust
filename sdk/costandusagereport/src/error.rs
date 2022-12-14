// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ValidationException {
    /// <p>A message to show the detail of the exception.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ValidationException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ValidationException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidationException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ValidationException {}
/// See [`ValidationException`](crate::error::ValidationException).
pub mod validation_exception {

    /// A builder for [`ValidationException`](crate::error::ValidationException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>A message to show the detail of the exception.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// <p>A message to show the detail of the exception.</p>
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ValidationException`](crate::error::ValidationException).
        pub fn build(self) -> crate::error::ValidationException {
            crate::error::ValidationException {
                message: self.message,
            }
        }
    }
}
impl ValidationException {
    /// Creates a new builder-style object to manufacture [`ValidationException`](crate::error::ValidationException).
    pub fn builder() -> crate::error::validation_exception::Builder {
        crate::error::validation_exception::Builder::default()
    }
}

/// <p>This account already has five reports defined. To define a new report, you must delete an existing report.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ReportLimitReachedException {
    /// <p>A message to show the detail of the exception.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ReportLimitReachedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ReportLimitReachedException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ReportLimitReachedException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ReportLimitReachedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReportLimitReachedException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for ReportLimitReachedException {}
/// See [`ReportLimitReachedException`](crate::error::ReportLimitReachedException).
pub mod report_limit_reached_exception {

    /// A builder for [`ReportLimitReachedException`](crate::error::ReportLimitReachedException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>A message to show the detail of the exception.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// <p>A message to show the detail of the exception.</p>
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ReportLimitReachedException`](crate::error::ReportLimitReachedException).
        pub fn build(self) -> crate::error::ReportLimitReachedException {
            crate::error::ReportLimitReachedException {
                message: self.message,
            }
        }
    }
}
impl ReportLimitReachedException {
    /// Creates a new builder-style object to manufacture [`ReportLimitReachedException`](crate::error::ReportLimitReachedException).
    pub fn builder() -> crate::error::report_limit_reached_exception::Builder {
        crate::error::report_limit_reached_exception::Builder::default()
    }
}

/// <p>An error on the server occurred during the processing of your request. Try again later.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalErrorException {
    /// <p>A message to show the detail of the exception.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalErrorException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalErrorException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalErrorException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalErrorException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalErrorException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalErrorException {}
/// See [`InternalErrorException`](crate::error::InternalErrorException).
pub mod internal_error_exception {

    /// A builder for [`InternalErrorException`](crate::error::InternalErrorException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>A message to show the detail of the exception.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// <p>A message to show the detail of the exception.</p>
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InternalErrorException`](crate::error::InternalErrorException).
        pub fn build(self) -> crate::error::InternalErrorException {
            crate::error::InternalErrorException {
                message: self.message,
            }
        }
    }
}
impl InternalErrorException {
    /// Creates a new builder-style object to manufacture [`InternalErrorException`](crate::error::InternalErrorException).
    pub fn builder() -> crate::error::internal_error_exception::Builder {
        crate::error::internal_error_exception::Builder::default()
    }
}

/// <p>A report with the specified name already exists in the account. Specify a different report name.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DuplicateReportNameException {
    /// <p>A message to show the detail of the exception.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DuplicateReportNameException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DuplicateReportNameException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl DuplicateReportNameException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for DuplicateReportNameException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DuplicateReportNameException")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for DuplicateReportNameException {}
/// See [`DuplicateReportNameException`](crate::error::DuplicateReportNameException).
pub mod duplicate_report_name_exception {

    /// A builder for [`DuplicateReportNameException`](crate::error::DuplicateReportNameException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>A message to show the detail of the exception.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// <p>A message to show the detail of the exception.</p>
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`DuplicateReportNameException`](crate::error::DuplicateReportNameException).
        pub fn build(self) -> crate::error::DuplicateReportNameException {
            crate::error::DuplicateReportNameException {
                message: self.message,
            }
        }
    }
}
impl DuplicateReportNameException {
    /// Creates a new builder-style object to manufacture [`DuplicateReportNameException`](crate::error::DuplicateReportNameException).
    pub fn builder() -> crate::error::duplicate_report_name_exception::Builder {
        crate::error::duplicate_report_name_exception::Builder::default()
    }
}

/// Error type for the `DeleteReportDefinition` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DeleteReportDefinitionError {
    /// Kind of error that occurred.
    pub kind: DeleteReportDefinitionErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for DeleteReportDefinitionError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: DeleteReportDefinitionErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `DeleteReportDefinition` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DeleteReportDefinitionErrorKind {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalErrorException(crate::error::InternalErrorException),
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
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
impl std::fmt::Display for DeleteReportDefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DeleteReportDefinitionErrorKind::InternalErrorException(_inner) => _inner.fmt(f),
            DeleteReportDefinitionErrorKind::ValidationException(_inner) => _inner.fmt(f),
            DeleteReportDefinitionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DeleteReportDefinitionError {
    fn code(&self) -> Option<&str> {
        DeleteReportDefinitionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DeleteReportDefinitionError {
    /// Creates a new `DeleteReportDefinitionError`.
    pub fn new(kind: DeleteReportDefinitionErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `DeleteReportDefinitionError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DeleteReportDefinitionErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
            meta: Default::default(),
        }
    }

    /// Creates the `DeleteReportDefinitionError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DeleteReportDefinitionErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `DeleteReportDefinitionErrorKind::InternalErrorException`.
    pub fn is_internal_error_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteReportDefinitionErrorKind::InternalErrorException(_)
        )
    }
    /// Returns `true` if the error kind is `DeleteReportDefinitionErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteReportDefinitionErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for DeleteReportDefinitionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DeleteReportDefinitionErrorKind::InternalErrorException(_inner) => Some(_inner),
            DeleteReportDefinitionErrorKind::ValidationException(_inner) => Some(_inner),
            DeleteReportDefinitionErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `DescribeReportDefinitions` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DescribeReportDefinitionsError {
    /// Kind of error that occurred.
    pub kind: DescribeReportDefinitionsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for DescribeReportDefinitionsError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: DescribeReportDefinitionsErrorKind::Unhandled(crate::error::Unhandled::new(
                source,
            )),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `DescribeReportDefinitions` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DescribeReportDefinitionsErrorKind {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalErrorException(crate::error::InternalErrorException),
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
impl std::fmt::Display for DescribeReportDefinitionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DescribeReportDefinitionsErrorKind::InternalErrorException(_inner) => _inner.fmt(f),
            DescribeReportDefinitionsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DescribeReportDefinitionsError {
    fn code(&self) -> Option<&str> {
        DescribeReportDefinitionsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DescribeReportDefinitionsError {
    /// Creates a new `DescribeReportDefinitionsError`.
    pub fn new(kind: DescribeReportDefinitionsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `DescribeReportDefinitionsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DescribeReportDefinitionsErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
            meta: Default::default(),
        }
    }

    /// Creates the `DescribeReportDefinitionsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DescribeReportDefinitionsErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `DescribeReportDefinitionsErrorKind::InternalErrorException`.
    pub fn is_internal_error_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeReportDefinitionsErrorKind::InternalErrorException(_)
        )
    }
}
impl std::error::Error for DescribeReportDefinitionsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DescribeReportDefinitionsErrorKind::InternalErrorException(_inner) => Some(_inner),
            DescribeReportDefinitionsErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `ModifyReportDefinition` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ModifyReportDefinitionError {
    /// Kind of error that occurred.
    pub kind: ModifyReportDefinitionErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for ModifyReportDefinitionError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: ModifyReportDefinitionErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `ModifyReportDefinition` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ModifyReportDefinitionErrorKind {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalErrorException(crate::error::InternalErrorException),
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
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
impl std::fmt::Display for ModifyReportDefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ModifyReportDefinitionErrorKind::InternalErrorException(_inner) => _inner.fmt(f),
            ModifyReportDefinitionErrorKind::ValidationException(_inner) => _inner.fmt(f),
            ModifyReportDefinitionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ModifyReportDefinitionError {
    fn code(&self) -> Option<&str> {
        ModifyReportDefinitionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ModifyReportDefinitionError {
    /// Creates a new `ModifyReportDefinitionError`.
    pub fn new(kind: ModifyReportDefinitionErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `ModifyReportDefinitionError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: ModifyReportDefinitionErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
            meta: Default::default(),
        }
    }

    /// Creates the `ModifyReportDefinitionError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: ModifyReportDefinitionErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `ModifyReportDefinitionErrorKind::InternalErrorException`.
    pub fn is_internal_error_exception(&self) -> bool {
        matches!(
            &self.kind,
            ModifyReportDefinitionErrorKind::InternalErrorException(_)
        )
    }
    /// Returns `true` if the error kind is `ModifyReportDefinitionErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            ModifyReportDefinitionErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for ModifyReportDefinitionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ModifyReportDefinitionErrorKind::InternalErrorException(_inner) => Some(_inner),
            ModifyReportDefinitionErrorKind::ValidationException(_inner) => Some(_inner),
            ModifyReportDefinitionErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `PutReportDefinition` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct PutReportDefinitionError {
    /// Kind of error that occurred.
    pub kind: PutReportDefinitionErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for PutReportDefinitionError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: PutReportDefinitionErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `PutReportDefinition` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PutReportDefinitionErrorKind {
    /// <p>A report with the specified name already exists in the account. Specify a different report name.</p>
    DuplicateReportNameException(crate::error::DuplicateReportNameException),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalErrorException(crate::error::InternalErrorException),
    /// <p>This account already has five reports defined. To define a new report, you must delete an existing report.</p>
    ReportLimitReachedException(crate::error::ReportLimitReachedException),
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
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
impl std::fmt::Display for PutReportDefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PutReportDefinitionErrorKind::DuplicateReportNameException(_inner) => _inner.fmt(f),
            PutReportDefinitionErrorKind::InternalErrorException(_inner) => _inner.fmt(f),
            PutReportDefinitionErrorKind::ReportLimitReachedException(_inner) => _inner.fmt(f),
            PutReportDefinitionErrorKind::ValidationException(_inner) => _inner.fmt(f),
            PutReportDefinitionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for PutReportDefinitionError {
    fn code(&self) -> Option<&str> {
        PutReportDefinitionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl PutReportDefinitionError {
    /// Creates a new `PutReportDefinitionError`.
    pub fn new(kind: PutReportDefinitionErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `PutReportDefinitionError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: PutReportDefinitionErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `PutReportDefinitionError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: PutReportDefinitionErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `PutReportDefinitionErrorKind::DuplicateReportNameException`.
    pub fn is_duplicate_report_name_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutReportDefinitionErrorKind::DuplicateReportNameException(_)
        )
    }
    /// Returns `true` if the error kind is `PutReportDefinitionErrorKind::InternalErrorException`.
    pub fn is_internal_error_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutReportDefinitionErrorKind::InternalErrorException(_)
        )
    }
    /// Returns `true` if the error kind is `PutReportDefinitionErrorKind::ReportLimitReachedException`.
    pub fn is_report_limit_reached_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutReportDefinitionErrorKind::ReportLimitReachedException(_)
        )
    }
    /// Returns `true` if the error kind is `PutReportDefinitionErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutReportDefinitionErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for PutReportDefinitionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            PutReportDefinitionErrorKind::DuplicateReportNameException(_inner) => Some(_inner),
            PutReportDefinitionErrorKind::InternalErrorException(_inner) => Some(_inner),
            PutReportDefinitionErrorKind::ReportLimitReachedException(_inner) => Some(_inner),
            PutReportDefinitionErrorKind::ValidationException(_inner) => Some(_inner),
            PutReportDefinitionErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

///
/// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
///
/// When logging an error from the SDK, it is recommended that you either wrap the error in
/// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
/// error reporter library that visits the error's cause/source chain, or call
/// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
///
#[derive(Debug)]
pub struct Unhandled {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
}
impl Unhandled {
    pub(crate) fn new(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self { source }
    }
}
impl std::fmt::Display for Unhandled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "unhandled error")
    }
}
impl std::error::Error for Unhandled {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref() as _)
    }
}
