// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_configuration_sets::_list_configuration_sets_output::ListConfigurationSetsOutputBuilder;

pub use crate::operation::list_configuration_sets::_list_configuration_sets_input::ListConfigurationSetsInputBuilder;

/// Fluent builder constructing a request to `ListConfigurationSets`.
///
/// List all of the configuration sets associated with your Amazon Pinpoint account in the current region.
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListConfigurationSetsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_configuration_sets::builders::ListConfigurationSetsInputBuilder,
}
impl ListConfigurationSetsFluentBuilder {
    /// Creates a new `ListConfigurationSets`.
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
            crate::operation::list_configuration_sets::ListConfigurationSets,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_configuration_sets::ListConfigurationSetsError,
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
        crate::operation::list_configuration_sets::ListConfigurationSetsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_configuration_sets::ListConfigurationSetsError,
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
    /// A token returned from a previous call to the API that indicates the position in the list of results.
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// A token returned from a previous call to the API that indicates the position in the list of results.
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// Used to specify the number of items that should be returned in the response.
    pub fn page_size(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.page_size(input.into());
        self
    }
    /// Used to specify the number of items that should be returned in the response.
    pub fn set_page_size(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_page_size(input);
        self
    }
}
