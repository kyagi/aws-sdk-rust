// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_stream_key::_delete_stream_key_output::DeleteStreamKeyOutputBuilder;

pub use crate::operation::delete_stream_key::_delete_stream_key_input::DeleteStreamKeyInputBuilder;

/// Fluent builder constructing a request to `DeleteStreamKey`.
///
/// <p>Deletes the stream key for the specified ARN, so it can no longer be used to stream.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteStreamKeyFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_stream_key::builders::DeleteStreamKeyInputBuilder,
}
impl DeleteStreamKeyFluentBuilder {
    /// Creates a new `DeleteStreamKey`.
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
            crate::operation::delete_stream_key::DeleteStreamKey,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_stream_key::DeleteStreamKeyError,
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
        crate::operation::delete_stream_key::DeleteStreamKeyOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_stream_key::DeleteStreamKeyError,
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
    /// <p>ARN of the stream key to be deleted.</p>
    pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>ARN of the stream key to be deleted.</p>
    pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
}
