// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_security::_update_security_output::UpdateSecurityOutputBuilder;

pub use crate::operation::update_security::_update_security_input::UpdateSecurityInputBuilder;

/// Fluent builder constructing a request to `UpdateSecurity`.
///
/// <p>Updates the security settings for the cluster. You can use this operation to specify encryption and authentication on existing clusters.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateSecurityFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_security::builders::UpdateSecurityInputBuilder,
}
impl UpdateSecurityFluentBuilder {
    /// Creates a new `UpdateSecurity`.
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
            crate::operation::update_security::UpdateSecurity,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_security::UpdateSecurityError>,
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
        crate::operation::update_security::UpdateSecurityOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_security::UpdateSecurityError>,
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
    /// <p>Includes all client authentication related information.</p>
    pub fn client_authentication(mut self, input: crate::types::ClientAuthentication) -> Self {
        self.inner = self.inner.client_authentication(input);
        self
    }
    /// <p>Includes all client authentication related information.</p>
    pub fn set_client_authentication(
        mut self,
        input: std::option::Option<crate::types::ClientAuthentication>,
    ) -> Self {
        self.inner = self.inner.set_client_authentication(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p>
    pub fn cluster_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cluster_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p>
    pub fn set_cluster_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_arn(input);
        self
    }
    /// <p>The version of the MSK cluster to update. Cluster versions aren't simple numbers. You can describe an MSK cluster to find its version. When this update operation is successful, it generates a new cluster version.</p>
    pub fn current_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.current_version(input.into());
        self
    }
    /// <p>The version of the MSK cluster to update. Cluster versions aren't simple numbers. You can describe an MSK cluster to find its version. When this update operation is successful, it generates a new cluster version.</p>
    pub fn set_current_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_current_version(input);
        self
    }
    /// <p>Includes all encryption-related information.</p>
    pub fn encryption_info(mut self, input: crate::types::EncryptionInfo) -> Self {
        self.inner = self.inner.encryption_info(input);
        self
    }
    /// <p>Includes all encryption-related information.</p>
    pub fn set_encryption_info(
        mut self,
        input: std::option::Option<crate::types::EncryptionInfo>,
    ) -> Self {
        self.inner = self.inner.set_encryption_info(input);
        self
    }
}
