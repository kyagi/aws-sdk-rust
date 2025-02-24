// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_application_version::_create_application_version_output::CreateApplicationVersionOutputBuilder;

pub use crate::operation::create_application_version::_create_application_version_input::CreateApplicationVersionInputBuilder;

/// Fluent builder constructing a request to `CreateApplicationVersion`.
///
/// <p>Creates an application version.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateApplicationVersionFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_application_version::builders::CreateApplicationVersionInputBuilder
            }
impl CreateApplicationVersionFluentBuilder {
    /// Creates a new `CreateApplicationVersion`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_application_version::CreateApplicationVersion,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_application_version::CreateApplicationVersionError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::create_application_version::CreateApplicationVersionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_application_version::CreateApplicationVersionError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    pub fn set_application_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The semantic version of the new version.</p>
    pub fn semantic_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.semantic_version(input.into());
        self
    }
    /// <p>The semantic version of the new version.</p>
    pub fn set_semantic_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_semantic_version(input);
        self
    }
    /// <p>A link to the S3 object that contains the ZIP archive of the source code for this version of your application.</p>
    /// <p>Maximum size 50 MB</p>
    pub fn source_code_archive_url(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.source_code_archive_url(input.into());
        self
    }
    /// <p>A link to the S3 object that contains the ZIP archive of the source code for this version of your application.</p>
    /// <p>Maximum size 50 MB</p>
    pub fn set_source_code_archive_url(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_source_code_archive_url(input);
        self
    }
    /// <p>A link to a public repository for the source code of your application, for example the URL of a specific GitHub commit.</p>
    pub fn source_code_url(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.source_code_url(input.into());
        self
    }
    /// <p>A link to a public repository for the source code of your application, for example the URL of a specific GitHub commit.</p>
    pub fn set_source_code_url(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_source_code_url(input);
        self
    }
    /// <p>The raw packaged AWS SAM template of your application.</p>
    pub fn template_body(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.template_body(input.into());
        self
    }
    /// <p>The raw packaged AWS SAM template of your application.</p>
    pub fn set_template_body(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_template_body(input);
        self
    }
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    pub fn template_url(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.template_url(input.into());
        self
    }
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    pub fn set_template_url(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_template_url(input);
        self
    }
}
