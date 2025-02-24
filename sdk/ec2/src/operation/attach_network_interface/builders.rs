// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::attach_network_interface::_attach_network_interface_output::AttachNetworkInterfaceOutputBuilder;

pub use crate::operation::attach_network_interface::_attach_network_interface_input::AttachNetworkInterfaceInputBuilder;

/// Fluent builder constructing a request to `AttachNetworkInterface`.
///
/// <p>Attaches a network interface to an instance.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AttachNetworkInterfaceFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::attach_network_interface::builders::AttachNetworkInterfaceInputBuilder,
}
impl AttachNetworkInterfaceFluentBuilder {
    /// Creates a new `AttachNetworkInterface`.
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
            crate::operation::attach_network_interface::AttachNetworkInterface,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::attach_network_interface::AttachNetworkInterfaceError,
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
        crate::operation::attach_network_interface::AttachNetworkInterfaceOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::attach_network_interface::AttachNetworkInterfaceError,
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
    /// <p>The index of the device for the network interface attachment.</p>
    pub fn device_index(mut self, input: i32) -> Self {
        self.inner = self.inner.device_index(input);
        self
    }
    /// <p>The index of the device for the network interface attachment.</p>
    pub fn set_device_index(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_device_index(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.network_interface_id(input.into());
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn set_network_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_network_interface_id(input);
        self
    }
    /// <p>The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.</p>
    pub fn network_card_index(mut self, input: i32) -> Self {
        self.inner = self.inner.network_card_index(input);
        self
    }
    /// <p>The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.</p>
    pub fn set_network_card_index(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_network_card_index(input);
        self
    }
    /// <p>Configures ENA Express for the network interface that this action attaches to the instance.</p>
    pub fn ena_srd_specification(mut self, input: crate::types::EnaSrdSpecification) -> Self {
        self.inner = self.inner.ena_srd_specification(input);
        self
    }
    /// <p>Configures ENA Express for the network interface that this action attaches to the instance.</p>
    pub fn set_ena_srd_specification(
        mut self,
        input: std::option::Option<crate::types::EnaSrdSpecification>,
    ) -> Self {
        self.inner = self.inner.set_ena_srd_specification(input);
        self
    }
}
