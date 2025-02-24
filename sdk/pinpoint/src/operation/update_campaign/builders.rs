// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_campaign::_update_campaign_output::UpdateCampaignOutputBuilder;

pub use crate::operation::update_campaign::_update_campaign_input::UpdateCampaignInputBuilder;

/// Fluent builder constructing a request to `UpdateCampaign`.
///
/// <p>Updates the configuration and other settings for a campaign.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateCampaignFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_campaign::builders::UpdateCampaignInputBuilder,
}
impl UpdateCampaignFluentBuilder {
    /// Creates a new `UpdateCampaign`.
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
            crate::operation::update_campaign::UpdateCampaign,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_campaign::UpdateCampaignError>,
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
        crate::operation::update_campaign::UpdateCampaignOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_campaign::UpdateCampaignError>,
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
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn set_application_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The unique identifier for the campaign.</p>
    pub fn campaign_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.campaign_id(input.into());
        self
    }
    /// <p>The unique identifier for the campaign.</p>
    pub fn set_campaign_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_campaign_id(input);
        self
    }
    /// <p>Specifies the configuration and other settings for a campaign.</p>
    pub fn write_campaign_request(mut self, input: crate::types::WriteCampaignRequest) -> Self {
        self.inner = self.inner.write_campaign_request(input);
        self
    }
    /// <p>Specifies the configuration and other settings for a campaign.</p>
    pub fn set_write_campaign_request(
        mut self,
        input: std::option::Option<crate::types::WriteCampaignRequest>,
    ) -> Self {
        self.inner = self.inner.set_write_campaign_request(input);
        self
    }
}
