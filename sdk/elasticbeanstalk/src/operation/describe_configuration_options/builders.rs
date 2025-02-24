// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_configuration_options::_describe_configuration_options_output::DescribeConfigurationOptionsOutputBuilder;

pub use crate::operation::describe_configuration_options::_describe_configuration_options_input::DescribeConfigurationOptionsInputBuilder;

/// Fluent builder constructing a request to `DescribeConfigurationOptions`.
///
/// <p>Describes the configuration options that are used in a particular configuration template or environment, or that a specified solution stack defines. The description includes the values the options, their default values, and an indication of the required action on a running environment if an option value is changed.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeConfigurationOptionsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_configuration_options::builders::DescribeConfigurationOptionsInputBuilder
            }
impl DescribeConfigurationOptionsFluentBuilder {
    /// Creates a new `DescribeConfigurationOptions`.
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
            crate::operation::describe_configuration_options::DescribeConfigurationOptions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_configuration_options::DescribeConfigurationOptionsError,
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
        crate::operation::describe_configuration_options::DescribeConfigurationOptionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_configuration_options::DescribeConfigurationOptionsError,
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
    /// <p>The name of the application associated with the configuration template or environment. Only needed if you want to describe the configuration options associated with either the configuration template or environment.</p>
    pub fn application_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The name of the application associated with the configuration template or environment. Only needed if you want to describe the configuration options associated with either the configuration template or environment.</p>
    pub fn set_application_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>The name of the configuration template whose configuration options you want to describe.</p>
    pub fn template_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.template_name(input.into());
        self
    }
    /// <p>The name of the configuration template whose configuration options you want to describe.</p>
    pub fn set_template_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_template_name(input);
        self
    }
    /// <p>The name of the environment whose configuration options you want to describe.</p>
    pub fn environment_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.environment_name(input.into());
        self
    }
    /// <p>The name of the environment whose configuration options you want to describe.</p>
    pub fn set_environment_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_environment_name(input);
        self
    }
    /// <p>The name of the solution stack whose configuration options you want to describe.</p>
    pub fn solution_stack_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.solution_stack_name(input.into());
        self
    }
    /// <p>The name of the solution stack whose configuration options you want to describe.</p>
    pub fn set_solution_stack_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_solution_stack_name(input);
        self
    }
    /// <p>The ARN of the custom platform.</p>
    pub fn platform_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.platform_arn(input.into());
        self
    }
    /// <p>The ARN of the custom platform.</p>
    pub fn set_platform_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_platform_arn(input);
        self
    }
    /// Appends an item to `Options`.
    ///
    /// To override the contents of this collection use [`set_options`](Self::set_options).
    ///
    /// <p>If specified, restricts the descriptions to only the specified options.</p>
    pub fn options(mut self, input: crate::types::OptionSpecification) -> Self {
        self.inner = self.inner.options(input);
        self
    }
    /// <p>If specified, restricts the descriptions to only the specified options.</p>
    pub fn set_options(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::OptionSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_options(input);
        self
    }
}
