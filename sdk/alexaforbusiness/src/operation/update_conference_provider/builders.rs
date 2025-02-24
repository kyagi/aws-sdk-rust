// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_conference_provider::_update_conference_provider_output::UpdateConferenceProviderOutputBuilder;

pub use crate::operation::update_conference_provider::_update_conference_provider_input::UpdateConferenceProviderInputBuilder;

/// Fluent builder constructing a request to `UpdateConferenceProvider`.
///
/// <p>Updates an existing conference provider's settings.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateConferenceProviderFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::update_conference_provider::builders::UpdateConferenceProviderInputBuilder
            }
impl UpdateConferenceProviderFluentBuilder {
    /// Creates a new `UpdateConferenceProvider`.
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
            crate::operation::update_conference_provider::UpdateConferenceProvider,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_conference_provider::UpdateConferenceProviderError,
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
        crate::operation::update_conference_provider::UpdateConferenceProviderOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_conference_provider::UpdateConferenceProviderError,
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
    /// <p>The ARN of the conference provider.</p>
    pub fn conference_provider_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.conference_provider_arn(input.into());
        self
    }
    /// <p>The ARN of the conference provider.</p>
    pub fn set_conference_provider_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_conference_provider_arn(input);
        self
    }
    /// <p>The type of the conference provider.</p>
    pub fn conference_provider_type(mut self, input: crate::types::ConferenceProviderType) -> Self {
        self.inner = self.inner.conference_provider_type(input);
        self
    }
    /// <p>The type of the conference provider.</p>
    pub fn set_conference_provider_type(
        mut self,
        input: std::option::Option<crate::types::ConferenceProviderType>,
    ) -> Self {
        self.inner = self.inner.set_conference_provider_type(input);
        self
    }
    /// <p>The IP endpoint and protocol for calling.</p>
    pub fn ip_dial_in(mut self, input: crate::types::IpDialIn) -> Self {
        self.inner = self.inner.ip_dial_in(input);
        self
    }
    /// <p>The IP endpoint and protocol for calling.</p>
    pub fn set_ip_dial_in(mut self, input: std::option::Option<crate::types::IpDialIn>) -> Self {
        self.inner = self.inner.set_ip_dial_in(input);
        self
    }
    /// <p>The information for PSTN conferencing.</p>
    pub fn pstn_dial_in(mut self, input: crate::types::PstnDialIn) -> Self {
        self.inner = self.inner.pstn_dial_in(input);
        self
    }
    /// <p>The information for PSTN conferencing.</p>
    pub fn set_pstn_dial_in(
        mut self,
        input: std::option::Option<crate::types::PstnDialIn>,
    ) -> Self {
        self.inner = self.inner.set_pstn_dial_in(input);
        self
    }
    /// <p>The meeting settings for the conference provider.</p>
    pub fn meeting_setting(mut self, input: crate::types::MeetingSetting) -> Self {
        self.inner = self.inner.meeting_setting(input);
        self
    }
    /// <p>The meeting settings for the conference provider.</p>
    pub fn set_meeting_setting(
        mut self,
        input: std::option::Option<crate::types::MeetingSetting>,
    ) -> Self {
        self.inner = self.inner.set_meeting_setting(input);
        self
    }
}
