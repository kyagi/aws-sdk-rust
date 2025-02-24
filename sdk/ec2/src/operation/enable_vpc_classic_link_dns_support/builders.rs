// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_vpc_classic_link_dns_support::_enable_vpc_classic_link_dns_support_output::EnableVpcClassicLinkDnsSupportOutputBuilder;

pub use crate::operation::enable_vpc_classic_link_dns_support::_enable_vpc_classic_link_dns_support_input::EnableVpcClassicLinkDnsSupportInputBuilder;

/// Fluent builder constructing a request to `EnableVpcClassicLinkDnsSupport`.
///
/// <note>
/// <p>We are retiring EC2-Classic. We recommend that you migrate from EC2-Classic to a VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-migrate.html">Migrate from EC2-Classic to a VPC</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
/// </note>
/// <p>Enables a VPC to support DNS hostname resolution for ClassicLink. If enabled, the DNS hostname of a linked EC2-Classic instance resolves to its private IP address when addressed from an instance in the VPC to which it's linked. Similarly, the DNS hostname of an instance in a VPC resolves to its private IP address when addressed from a linked EC2-Classic instance. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
/// <p>You must specify a VPC ID in the request.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct EnableVpcClassicLinkDnsSupportFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::enable_vpc_classic_link_dns_support::builders::EnableVpcClassicLinkDnsSupportInputBuilder
            }
impl EnableVpcClassicLinkDnsSupportFluentBuilder {
    /// Creates a new `EnableVpcClassicLinkDnsSupport`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::enable_vpc_classic_link_dns_support::EnableVpcClassicLinkDnsSupport, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::enable_vpc_classic_link_dns_support::EnableVpcClassicLinkDnsSupportError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::enable_vpc_classic_link_dns_support::EnableVpcClassicLinkDnsSupportOutput, aws_smithy_http::result::SdkError<crate::operation::enable_vpc_classic_link_dns_support::EnableVpcClassicLinkDnsSupportError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.vpc_id(input.into());
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn set_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_id(input);
        self
    }
}
