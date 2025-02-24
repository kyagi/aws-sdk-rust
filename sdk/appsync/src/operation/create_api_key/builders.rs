// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_api_key::_create_api_key_output::CreateApiKeyOutputBuilder;

pub use crate::operation::create_api_key::_create_api_key_input::CreateApiKeyInputBuilder;

/// Fluent builder constructing a request to `CreateApiKey`.
///
/// <p>Creates a unique key that you can distribute to clients who invoke your API.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateApiKeyFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_api_key::builders::CreateApiKeyInputBuilder,
}
impl CreateApiKeyFluentBuilder {
    /// Creates a new `CreateApiKey`.
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
            crate::operation::create_api_key::CreateApiKey,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::create_api_key::CreateApiKeyError>,
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
        crate::operation::create_api_key::CreateApiKeyOutput,
        aws_smithy_http::result::SdkError<crate::operation::create_api_key::CreateApiKeyError>,
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
    /// <p>The ID for your GraphQL API.</p>
    pub fn api_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.api_id(input.into());
        self
    }
    /// <p>The ID for your GraphQL API.</p>
    pub fn set_api_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_api_id(input);
        self
    }
    /// <p>A description of the purpose of the API key.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the purpose of the API key.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>From the creation time, the time after which the API key expires. The date is represented as seconds since the epoch, rounded down to the nearest hour. The default value for this parameter is 7 days from creation time. For more information, see .</p>
    pub fn expires(mut self, input: i64) -> Self {
        self.inner = self.inner.expires(input);
        self
    }
    /// <p>From the creation time, the time after which the API key expires. The date is represented as seconds since the epoch, rounded down to the nearest hour. The default value for this parameter is 7 days from creation time. For more information, see .</p>
    pub fn set_expires(mut self, input: std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_expires(input);
        self
    }
}
