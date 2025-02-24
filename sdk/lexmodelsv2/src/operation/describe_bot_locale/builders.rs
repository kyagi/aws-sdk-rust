// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_bot_locale::_describe_bot_locale_output::DescribeBotLocaleOutputBuilder;

pub use crate::operation::describe_bot_locale::_describe_bot_locale_input::DescribeBotLocaleInputBuilder;

/// Fluent builder constructing a request to `DescribeBotLocale`.
///
/// <p>Describes the settings that a bot has for a specific locale. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBotLocaleFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_bot_locale::builders::DescribeBotLocaleInputBuilder,
}
impl DescribeBotLocaleFluentBuilder {
    /// Creates a new `DescribeBotLocale`.
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
            crate::operation::describe_bot_locale::DescribeBotLocale,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_bot_locale::DescribeBotLocaleError,
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
        crate::operation::describe_bot_locale::DescribeBotLocaleOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_bot_locale::DescribeBotLocaleError,
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
    /// <p>The identifier of the bot associated with the locale.</p>
    pub fn bot_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The identifier of the bot associated with the locale.</p>
    pub fn set_bot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The identifier of the version of the bot associated with the locale.</p>
    pub fn bot_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bot_version(input.into());
        self
    }
    /// <p>The identifier of the version of the bot associated with the locale.</p>
    pub fn set_bot_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bot_version(input);
        self
    }
    /// <p>The unique identifier of the locale to describe. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>. </p>
    pub fn locale_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.locale_id(input.into());
        self
    }
    /// <p>The unique identifier of the locale to describe. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>. </p>
    pub fn set_locale_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_locale_id(input);
        self
    }
}
