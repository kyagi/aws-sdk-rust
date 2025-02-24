// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_publisher::_register_publisher_output::RegisterPublisherOutputBuilder;

pub use crate::operation::register_publisher::_register_publisher_input::RegisterPublisherInputBuilder;

/// Fluent builder constructing a request to `RegisterPublisher`.
///
/// <p>Registers your account as a publisher of public extensions in the CloudFormation registry. Public extensions are available for use by all CloudFormation users. This publisher ID applies to your account in all Amazon Web Services Regions.</p>
/// <p>For information about requirements for registering as a public extension publisher, see <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/publish-extension.html#publish-extension-prereqs">Registering your account to publish CloudFormation extensions</a> in the <i>CloudFormation CLI User Guide</i>.</p>
/// <p></p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RegisterPublisherFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::register_publisher::builders::RegisterPublisherInputBuilder,
}
impl RegisterPublisherFluentBuilder {
    /// Creates a new `RegisterPublisher`.
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
            crate::operation::register_publisher::RegisterPublisher,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::register_publisher::RegisterPublisherError,
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
        crate::operation::register_publisher::RegisterPublisherOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::register_publisher::RegisterPublisherError,
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
    /// <p>Whether you accept the <a href="https://cloudformation-registry-documents.s3.amazonaws.com/Terms_and_Conditions_for_AWS_CloudFormation_Registry_Publishers.pdf">Terms and Conditions</a> for publishing extensions in the CloudFormation registry. You must accept the terms and conditions in order to register to publish public extensions to the CloudFormation registry.</p>
    /// <p>The default is <code>false</code>.</p>
    pub fn accept_terms_and_conditions(mut self, input: bool) -> Self {
        self.inner = self.inner.accept_terms_and_conditions(input);
        self
    }
    /// <p>Whether you accept the <a href="https://cloudformation-registry-documents.s3.amazonaws.com/Terms_and_Conditions_for_AWS_CloudFormation_Registry_Publishers.pdf">Terms and Conditions</a> for publishing extensions in the CloudFormation registry. You must accept the terms and conditions in order to register to publish public extensions to the CloudFormation registry.</p>
    /// <p>The default is <code>false</code>.</p>
    pub fn set_accept_terms_and_conditions(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_accept_terms_and_conditions(input);
        self
    }
    /// <p>If you are using a Bitbucket or GitHub account for identity verification, the Amazon Resource Name (ARN) for your connection to that account.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/publish-extension.html#publish-extension-prereqs">Registering your account to publish CloudFormation extensions</a> in the <i>CloudFormation CLI User Guide</i>.</p>
    pub fn connection_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.connection_arn(input.into());
        self
    }
    /// <p>If you are using a Bitbucket or GitHub account for identity verification, the Amazon Resource Name (ARN) for your connection to that account.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/publish-extension.html#publish-extension-prereqs">Registering your account to publish CloudFormation extensions</a> in the <i>CloudFormation CLI User Guide</i>.</p>
    pub fn set_connection_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_connection_arn(input);
        self
    }
}
