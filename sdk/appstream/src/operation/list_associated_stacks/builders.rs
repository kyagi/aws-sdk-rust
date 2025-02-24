// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_associated_stacks::_list_associated_stacks_output::ListAssociatedStacksOutputBuilder;

pub use crate::operation::list_associated_stacks::_list_associated_stacks_input::ListAssociatedStacksInputBuilder;

/// Fluent builder constructing a request to `ListAssociatedStacks`.
///
/// <p>Retrieves the name of the stack with which the specified fleet is associated.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListAssociatedStacksFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_associated_stacks::builders::ListAssociatedStacksInputBuilder,
}
impl ListAssociatedStacksFluentBuilder {
    /// Creates a new `ListAssociatedStacks`.
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
            crate::operation::list_associated_stacks::ListAssociatedStacks,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_associated_stacks::ListAssociatedStacksError,
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
        crate::operation::list_associated_stacks::ListAssociatedStacksOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_associated_stacks::ListAssociatedStacksError,
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
    /// <p>The name of the fleet.</p>
    pub fn fleet_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.fleet_name(input.into());
        self
    }
    /// <p>The name of the fleet.</p>
    pub fn set_fleet_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_fleet_name(input);
        self
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
