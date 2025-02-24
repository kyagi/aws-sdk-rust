// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_logging_configuration::_put_logging_configuration_output::PutLoggingConfigurationOutputBuilder;

pub use crate::operation::put_logging_configuration::_put_logging_configuration_input::PutLoggingConfigurationInputBuilder;

/// Fluent builder constructing a request to `PutLoggingConfiguration`.
///
/// <p>Enables the specified <code>LoggingConfiguration</code>, to start logging from a web ACL, according to the configuration provided. </p> <note>
/// <p>This operation completely replaces any mutable specifications that you already have for a logging configuration with the ones that you provide to this call. </p>
/// <p>To modify an existing logging configuration, do the following: </p>
/// <ol>
/// <li> <p>Retrieve it by calling <code>GetLoggingConfiguration</code> </p> </li>
/// <li> <p>Update its settings as needed</p> </li>
/// <li> <p>Provide the complete logging configuration specification to this call</p> </li>
/// </ol>
/// </note> <note>
/// <p>You can define one logging destination per web ACL.</p>
/// </note>
/// <p>You can access information about the traffic that WAF inspects using the following steps:</p>
/// <ol>
/// <li> <p>Create your logging destination. You can use an Amazon CloudWatch Logs log group, an Amazon Simple Storage Service (Amazon S3) bucket, or an Amazon Kinesis Data Firehose. </p> <p>The name that you give the destination must start with <code>aws-waf-logs-</code>. Depending on the type of destination, you might need to configure additional settings or permissions. </p> <p>For configuration requirements and pricing information for each destination type, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/logging.html">Logging web ACL traffic</a> in the <i>WAF Developer Guide</i>.</p> </li>
/// <li> <p>Associate your logging destination to your web ACL using a <code>PutLoggingConfiguration</code> request.</p> </li>
/// </ol>
/// <p>When you successfully enable logging using a <code>PutLoggingConfiguration</code> request, WAF creates an additional role or policy that is required to write logs to the logging destination. For an Amazon CloudWatch Logs log group, WAF creates a resource policy on the log group. For an Amazon S3 bucket, WAF creates a bucket policy. For an Amazon Kinesis Data Firehose, WAF creates a service-linked role.</p>
/// <p>For additional information about web ACL logging, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/logging.html">Logging web ACL traffic information</a> in the <i>WAF Developer Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutLoggingConfigurationFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::put_logging_configuration::builders::PutLoggingConfigurationInputBuilder,
}
impl PutLoggingConfigurationFluentBuilder {
    /// Creates a new `PutLoggingConfiguration`.
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
            crate::operation::put_logging_configuration::PutLoggingConfiguration,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_logging_configuration::PutLoggingConfigurationError,
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
        crate::operation::put_logging_configuration::PutLoggingConfigurationOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_logging_configuration::PutLoggingConfigurationError,
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
    /// <p></p>
    pub fn logging_configuration(mut self, input: crate::types::LoggingConfiguration) -> Self {
        self.inner = self.inner.logging_configuration(input);
        self
    }
    /// <p></p>
    pub fn set_logging_configuration(
        mut self,
        input: std::option::Option<crate::types::LoggingConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_logging_configuration(input);
        self
    }
}
