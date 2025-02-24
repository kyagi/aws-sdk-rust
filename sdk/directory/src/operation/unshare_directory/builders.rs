// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::unshare_directory::_unshare_directory_output::UnshareDirectoryOutputBuilder;

pub use crate::operation::unshare_directory::_unshare_directory_input::UnshareDirectoryInputBuilder;

/// Fluent builder constructing a request to `UnshareDirectory`.
///
/// <p>Stops the directory sharing between the directory owner and consumer accounts. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UnshareDirectoryFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::unshare_directory::builders::UnshareDirectoryInputBuilder,
}
impl UnshareDirectoryFluentBuilder {
    /// Creates a new `UnshareDirectory`.
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
            crate::operation::unshare_directory::UnshareDirectory,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::unshare_directory::UnshareDirectoryError,
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
        crate::operation::unshare_directory::UnshareDirectoryOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::unshare_directory::UnshareDirectoryError,
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
    /// <p>The identifier of the Managed Microsoft AD directory that you want to stop sharing.</p>
    pub fn directory_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The identifier of the Managed Microsoft AD directory that you want to stop sharing.</p>
    pub fn set_directory_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>Identifier for the directory consumer account with whom the directory has to be unshared.</p>
    pub fn unshare_target(mut self, input: crate::types::UnshareTarget) -> Self {
        self.inner = self.inner.unshare_target(input);
        self
    }
    /// <p>Identifier for the directory consumer account with whom the directory has to be unshared.</p>
    pub fn set_unshare_target(
        mut self,
        input: std::option::Option<crate::types::UnshareTarget>,
    ) -> Self {
        self.inner = self.inner.set_unshare_target(input);
        self
    }
}
