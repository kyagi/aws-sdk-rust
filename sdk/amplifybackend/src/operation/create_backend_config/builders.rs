// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_backend_config::_create_backend_config_output::CreateBackendConfigOutputBuilder;

pub use crate::operation::create_backend_config::_create_backend_config_input::CreateBackendConfigInputBuilder;

/// Fluent builder constructing a request to `CreateBackendConfig`.
///
/// <p>Creates a config object for a backend.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateBackendConfigFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_backend_config::builders::CreateBackendConfigInputBuilder,
}
impl CreateBackendConfigFluentBuilder {
    /// Creates a new `CreateBackendConfig`.
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
            crate::operation::create_backend_config::CreateBackendConfig,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_backend_config::CreateBackendConfigError,
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
        crate::operation::create_backend_config::CreateBackendConfigOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_backend_config::CreateBackendConfigError,
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
    /// <p>The app ID.</p>
    pub fn app_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.app_id(input.into());
        self
    }
    /// <p>The app ID.</p>
    pub fn set_app_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_app_id(input);
        self
    }
    /// <p>The app ID for the backend manager.</p>
    pub fn backend_manager_app_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.backend_manager_app_id(input.into());
        self
    }
    /// <p>The app ID for the backend manager.</p>
    pub fn set_backend_manager_app_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_backend_manager_app_id(input);
        self
    }
}
