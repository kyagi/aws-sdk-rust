// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_realtime_log_config::_get_realtime_log_config_output::GetRealtimeLogConfigOutputBuilder;

pub use crate::operation::get_realtime_log_config::_get_realtime_log_config_input::GetRealtimeLogConfigInputBuilder;

/// Fluent builder constructing a request to `GetRealtimeLogConfig`.
///
/// <p>Gets a real-time log configuration.</p>
/// <p>To get a real-time log configuration, you can provide the configuration's name or its Amazon Resource Name (ARN). You must provide at least one. If you provide both, CloudFront uses the name to identify the real-time log configuration to get.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetRealtimeLogConfigFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_realtime_log_config::builders::GetRealtimeLogConfigInputBuilder,
}
impl GetRealtimeLogConfigFluentBuilder {
    /// Creates a new `GetRealtimeLogConfig`.
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
            crate::operation::get_realtime_log_config::GetRealtimeLogConfig,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_realtime_log_config::GetRealtimeLogConfigError,
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
        crate::operation::get_realtime_log_config::GetRealtimeLogConfigOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_realtime_log_config::GetRealtimeLogConfigError,
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
    /// <p>The name of the real-time log configuration to get.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the real-time log configuration to get.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the real-time log configuration to get.</p>
    pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the real-time log configuration to get.</p>
    pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
}
