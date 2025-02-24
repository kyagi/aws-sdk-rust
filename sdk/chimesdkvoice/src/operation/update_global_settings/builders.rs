// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_global_settings::_update_global_settings_output::UpdateGlobalSettingsOutputBuilder;

pub use crate::operation::update_global_settings::_update_global_settings_input::UpdateGlobalSettingsInputBuilder;

/// Fluent builder constructing a request to `UpdateGlobalSettings`.
///
/// <p>Updates global settings for the Amazon Chime SDK Voice Connectors in an AWS account.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateGlobalSettingsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_global_settings::builders::UpdateGlobalSettingsInputBuilder,
}
impl UpdateGlobalSettingsFluentBuilder {
    /// Creates a new `UpdateGlobalSettings`.
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
            crate::operation::update_global_settings::UpdateGlobalSettings,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_global_settings::UpdateGlobalSettingsError,
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
        crate::operation::update_global_settings::UpdateGlobalSettingsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_global_settings::UpdateGlobalSettingsError,
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
    /// <p>The Voice Connector settings.</p>
    pub fn voice_connector(mut self, input: crate::types::VoiceConnectorSettings) -> Self {
        self.inner = self.inner.voice_connector(input);
        self
    }
    /// <p>The Voice Connector settings.</p>
    pub fn set_voice_connector(
        mut self,
        input: std::option::Option<crate::types::VoiceConnectorSettings>,
    ) -> Self {
        self.inner = self.inner.set_voice_connector(input);
        self
    }
}
