// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::configure_logs_for_channel::_configure_logs_for_channel_output::ConfigureLogsForChannelOutputBuilder;

pub use crate::operation::configure_logs_for_channel::_configure_logs_for_channel_input::ConfigureLogsForChannelInputBuilder;

/// Fluent builder constructing a request to `ConfigureLogsForChannel`.
///
/// <p>Configures Amazon CloudWatch log settings for a channel.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ConfigureLogsForChannelFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::configure_logs_for_channel::builders::ConfigureLogsForChannelInputBuilder,
}
impl ConfigureLogsForChannelFluentBuilder {
    /// Creates a new `ConfigureLogsForChannel`.
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
            crate::operation::configure_logs_for_channel::ConfigureLogsForChannel,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::configure_logs_for_channel::ConfigureLogsForChannelError,
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
        crate::operation::configure_logs_for_channel::ConfigureLogsForChannelOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::configure_logs_for_channel::ConfigureLogsForChannelError,
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
    /// <p>The name of the channel.</p>
    pub fn channel_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.channel_name(input.into());
        self
    }
    /// <p>The name of the channel.</p>
    pub fn set_channel_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_channel_name(input);
        self
    }
    /// Appends an item to `LogTypes`.
    ///
    /// To override the contents of this collection use [`set_log_types`](Self::set_log_types).
    ///
    /// <p>The types of logs to collect.</p>
    pub fn log_types(mut self, input: crate::types::LogType) -> Self {
        self.inner = self.inner.log_types(input);
        self
    }
    /// <p>The types of logs to collect.</p>
    pub fn set_log_types(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::LogType>>,
    ) -> Self {
        self.inner = self.inner.set_log_types(input);
        self
    }
}
