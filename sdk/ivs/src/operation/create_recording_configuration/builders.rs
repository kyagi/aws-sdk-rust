// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_recording_configuration::_create_recording_configuration_output::CreateRecordingConfigurationOutputBuilder;

pub use crate::operation::create_recording_configuration::_create_recording_configuration_input::CreateRecordingConfigurationInputBuilder;

/// Fluent builder constructing a request to `CreateRecordingConfiguration`.
///
/// <p>Creates a new recording configuration, used to enable recording to Amazon S3.</p>
/// <p> <b>Known issue:</b> In the us-east-1 region, if you use the Amazon Web Services CLI to create a recording configuration, it returns success even if the S3 bucket is in a different region. In this case, the <code>state</code> of the recording configuration is <code>CREATE_FAILED</code> (instead of <code>ACTIVE</code>). (In other regions, the CLI correctly returns failure if the bucket is in a different region.)</p>
/// <p> <b>Workaround:</b> Ensure that your S3 bucket is in the same region as the recording configuration. If you create a recording configuration in a different region as your S3 bucket, delete that recording configuration and create a new one with an S3 bucket from the correct region.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateRecordingConfigurationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_recording_configuration::builders::CreateRecordingConfigurationInputBuilder
            }
impl CreateRecordingConfigurationFluentBuilder {
    /// Creates a new `CreateRecordingConfiguration`.
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
            crate::operation::create_recording_configuration::CreateRecordingConfiguration,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_recording_configuration::CreateRecordingConfigurationError,
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
        crate::operation::create_recording_configuration::CreateRecordingConfigurationOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_recording_configuration::CreateRecordingConfigurationError,
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
    /// <p>Recording-configuration name. The value does not need to be unique.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>Recording-configuration name. The value does not need to be unique.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A complex type that contains a destination configuration for where recorded video will be stored.</p>
    pub fn destination_configuration(
        mut self,
        input: crate::types::DestinationConfiguration,
    ) -> Self {
        self.inner = self.inner.destination_configuration(input);
        self
    }
    /// <p>A complex type that contains a destination configuration for where recorded video will be stored.</p>
    pub fn set_destination_configuration(
        mut self,
        input: std::option::Option<crate::types::DestinationConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_destination_configuration(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Array of 1-50 maps, each of the form <code>string:string (key:value)</code>. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a> for more information, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS has no service-specific constraints beyond what is documented there.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Array of 1-50 maps, each of the form <code>string:string (key:value)</code>. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a> for more information, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS has no service-specific constraints beyond what is documented there.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A complex type that allows you to enable/disable the recording of thumbnails for a live session and modify the interval at which thumbnails are generated for the live session.</p>
    pub fn thumbnail_configuration(mut self, input: crate::types::ThumbnailConfiguration) -> Self {
        self.inner = self.inner.thumbnail_configuration(input);
        self
    }
    /// <p>A complex type that allows you to enable/disable the recording of thumbnails for a live session and modify the interval at which thumbnails are generated for the live session.</p>
    pub fn set_thumbnail_configuration(
        mut self,
        input: std::option::Option<crate::types::ThumbnailConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_thumbnail_configuration(input);
        self
    }
    /// <p>If a broadcast disconnects and then reconnects within the specified interval, the multiple streams will be considered a single broadcast and merged together. Default: 0.</p>
    pub fn recording_reconnect_window_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.recording_reconnect_window_seconds(input);
        self
    }
    /// <p>If a broadcast disconnects and then reconnects within the specified interval, the multiple streams will be considered a single broadcast and merged together. Default: 0.</p>
    pub fn set_recording_reconnect_window_seconds(
        mut self,
        input: std::option::Option<i32>,
    ) -> Self {
        self.inner = self.inner.set_recording_reconnect_window_seconds(input);
        self
    }
}
