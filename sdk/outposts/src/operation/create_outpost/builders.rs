// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_outpost::_create_outpost_output::CreateOutpostOutputBuilder;

pub use crate::operation::create_outpost::_create_outpost_input::CreateOutpostInputBuilder;

/// Fluent builder constructing a request to `CreateOutpost`.
///
/// <p>Creates an Outpost.</p>
/// <p>You can specify either an Availability one or an AZ ID.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateOutpostFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_outpost::builders::CreateOutpostInputBuilder,
}
impl CreateOutpostFluentBuilder {
    /// Creates a new `CreateOutpost`.
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
            crate::operation::create_outpost::CreateOutpost,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::create_outpost::CreateOutpostError>,
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
        crate::operation::create_outpost::CreateOutpostOutput,
        aws_smithy_http::result::SdkError<crate::operation::create_outpost::CreateOutpostError>,
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
    /// <p>The name of the Outpost.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the Outpost.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The description of the Outpost.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the Outpost.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p> The ID or the Amazon Resource Name (ARN) of the site. </p>
    pub fn site_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.site_id(input.into());
        self
    }
    /// <p> The ID or the Amazon Resource Name (ARN) of the site. </p>
    pub fn set_site_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_site_id(input);
        self
    }
    /// <p>The Availability Zone.</p>
    pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p>The Availability Zone.</p>
    pub fn set_availability_zone(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>The ID of the Availability Zone.</p>
    pub fn availability_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.availability_zone_id(input.into());
        self
    }
    /// <p>The ID of the Availability Zone.</p>
    pub fn set_availability_zone_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_availability_zone_id(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to apply to the Outpost.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags to apply to the Outpost.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p> The type of hardware for this Outpost. </p>
    pub fn supported_hardware_type(mut self, input: crate::types::SupportedHardwareType) -> Self {
        self.inner = self.inner.supported_hardware_type(input);
        self
    }
    /// <p> The type of hardware for this Outpost. </p>
    pub fn set_supported_hardware_type(
        mut self,
        input: std::option::Option<crate::types::SupportedHardwareType>,
    ) -> Self {
        self.inner = self.inner.set_supported_hardware_type(input);
        self
    }
}
