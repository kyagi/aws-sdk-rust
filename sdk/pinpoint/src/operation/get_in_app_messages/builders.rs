// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_in_app_messages::_get_in_app_messages_output::GetInAppMessagesOutputBuilder;

pub use crate::operation::get_in_app_messages::_get_in_app_messages_input::GetInAppMessagesInputBuilder;

/// Fluent builder constructing a request to `GetInAppMessages`.
///
/// <p>Retrieves the in-app messages targeted for the provided endpoint ID.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetInAppMessagesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_in_app_messages::builders::GetInAppMessagesInputBuilder,
}
impl GetInAppMessagesFluentBuilder {
    /// Creates a new `GetInAppMessages`.
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
            crate::operation::get_in_app_messages::GetInAppMessages,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_in_app_messages::GetInAppMessagesError,
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
        crate::operation::get_in_app_messages::GetInAppMessagesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_in_app_messages::GetInAppMessagesError,
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
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn set_application_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The unique identifier for the endpoint.</p>
    pub fn endpoint_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.endpoint_id(input.into());
        self
    }
    /// <p>The unique identifier for the endpoint.</p>
    pub fn set_endpoint_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_endpoint_id(input);
        self
    }
}
