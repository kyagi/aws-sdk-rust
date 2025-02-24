// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_access_policy::_update_access_policy_output::UpdateAccessPolicyOutputBuilder;

pub use crate::operation::update_access_policy::_update_access_policy_input::UpdateAccessPolicyInputBuilder;

/// Fluent builder constructing a request to `UpdateAccessPolicy`.
///
/// <p>Updates an OpenSearch Serverless access policy. For more information, see <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless-data-access.html">Data access control for Amazon OpenSearch Serverless</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateAccessPolicyFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_access_policy::builders::UpdateAccessPolicyInputBuilder,
}
impl UpdateAccessPolicyFluentBuilder {
    /// Creates a new `UpdateAccessPolicy`.
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
            crate::operation::update_access_policy::UpdateAccessPolicy,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_access_policy::UpdateAccessPolicyError,
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
        crate::operation::update_access_policy::UpdateAccessPolicyOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_access_policy::UpdateAccessPolicyError,
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
    /// <p>The type of policy.</p>
    pub fn r#type(mut self, input: crate::types::AccessPolicyType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of policy.</p>
    pub fn set_type(mut self, input: std::option::Option<crate::types::AccessPolicyType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The name of the policy.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the policy.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The version of the policy being updated.</p>
    pub fn policy_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.policy_version(input.into());
        self
    }
    /// <p>The version of the policy being updated.</p>
    pub fn set_policy_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_policy_version(input);
        self
    }
    /// <p>A description of the policy. Typically used to store information about the permissions defined in the policy.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the policy. Typically used to store information about the permissions defined in the policy.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The JSON policy document to use as the content for the policy.</p>
    pub fn policy(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.policy(input.into());
        self
    }
    /// <p>The JSON policy document to use as the content for the policy.</p>
    pub fn set_policy(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_policy(input);
        self
    }
    /// <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
