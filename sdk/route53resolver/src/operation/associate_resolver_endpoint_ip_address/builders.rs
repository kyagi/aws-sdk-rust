// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_resolver_endpoint_ip_address::_associate_resolver_endpoint_ip_address_output::AssociateResolverEndpointIpAddressOutputBuilder;

pub use crate::operation::associate_resolver_endpoint_ip_address::_associate_resolver_endpoint_ip_address_input::AssociateResolverEndpointIpAddressInputBuilder;

/// Fluent builder constructing a request to `AssociateResolverEndpointIpAddress`.
///
/// <p>Adds IP addresses to an inbound or an outbound Resolver endpoint. If you want to add more than one IP address, submit one <code>AssociateResolverEndpointIpAddress</code> request for each IP address.</p>
/// <p>To remove an IP address from an endpoint, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_DisassociateResolverEndpointIpAddress.html">DisassociateResolverEndpointIpAddress</a>. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AssociateResolverEndpointIpAddressFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::associate_resolver_endpoint_ip_address::builders::AssociateResolverEndpointIpAddressInputBuilder
            }
impl AssociateResolverEndpointIpAddressFluentBuilder {
    /// Creates a new `AssociateResolverEndpointIpAddress`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::associate_resolver_endpoint_ip_address::AssociateResolverEndpointIpAddress, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::associate_resolver_endpoint_ip_address::AssociateResolverEndpointIpAddressError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::associate_resolver_endpoint_ip_address::AssociateResolverEndpointIpAddressOutput, aws_smithy_http::result::SdkError<crate::operation::associate_resolver_endpoint_ip_address::AssociateResolverEndpointIpAddressError>>
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
    /// <p>The ID of the Resolver endpoint that you want to associate IP addresses with.</p>
    pub fn resolver_endpoint_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resolver_endpoint_id(input.into());
        self
    }
    /// <p>The ID of the Resolver endpoint that you want to associate IP addresses with.</p>
    pub fn set_resolver_endpoint_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_resolver_endpoint_id(input);
        self
    }
    /// <p>Either the IPv4 address that you want to add to a Resolver endpoint or a subnet ID. If you specify a subnet ID, Resolver chooses an IP address for you from the available IPs in the specified subnet.</p>
    pub fn ip_address(mut self, input: crate::types::IpAddressUpdate) -> Self {
        self.inner = self.inner.ip_address(input);
        self
    }
    /// <p>Either the IPv4 address that you want to add to a Resolver endpoint or a subnet ID. If you specify a subnet ID, Resolver chooses an IP address for you from the available IPs in the specified subnet.</p>
    pub fn set_ip_address(
        mut self,
        input: std::option::Option<crate::types::IpAddressUpdate>,
    ) -> Self {
        self.inner = self.inner.set_ip_address(input);
        self
    }
}
