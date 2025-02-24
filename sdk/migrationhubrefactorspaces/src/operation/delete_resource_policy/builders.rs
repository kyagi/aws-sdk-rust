// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_resource_policy::_delete_resource_policy_output::DeleteResourcePolicyOutputBuilder;

pub use crate::operation::delete_resource_policy::_delete_resource_policy_input::DeleteResourcePolicyInputBuilder;

/// Fluent builder constructing a request to `DeleteResourcePolicy`.
///
/// <p>Deletes the resource policy set for the environment. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteResourcePolicyFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_resource_policy::builders::DeleteResourcePolicyInputBuilder,
}
impl DeleteResourcePolicyFluentBuilder {
    /// Creates a new `DeleteResourcePolicy`.
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
            crate::operation::delete_resource_policy::DeleteResourcePolicy,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_resource_policy::DeleteResourcePolicyError,
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
        crate::operation::delete_resource_policy::DeleteResourcePolicyOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_resource_policy::DeleteResourcePolicyError,
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
    /// <p>Amazon Resource Name (ARN) of the resource associated with the policy. </p>
    pub fn identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.identifier(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of the resource associated with the policy. </p>
    pub fn set_identifier(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_identifier(input);
        self
    }
}
