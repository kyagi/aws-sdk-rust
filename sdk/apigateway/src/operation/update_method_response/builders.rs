// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_method_response::_update_method_response_output::UpdateMethodResponseOutputBuilder;

pub use crate::operation::update_method_response::_update_method_response_input::UpdateMethodResponseInputBuilder;

/// Fluent builder constructing a request to `UpdateMethodResponse`.
///
/// <p>Updates an existing MethodResponse resource.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateMethodResponseFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_method_response::builders::UpdateMethodResponseInputBuilder,
}
impl UpdateMethodResponseFluentBuilder {
    /// Creates a new `UpdateMethodResponse`.
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
            crate::operation::update_method_response::UpdateMethodResponse,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_method_response::UpdateMethodResponseError,
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
        crate::operation::update_method_response::UpdateMethodResponseOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_method_response::UpdateMethodResponseError,
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
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn rest_api_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.rest_api_id(input.into());
        self
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn set_rest_api_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_rest_api_id(input);
        self
    }
    /// <p>The Resource identifier for the MethodResponse resource.</p>
    pub fn resource_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>The Resource identifier for the MethodResponse resource.</p>
    pub fn set_resource_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>The HTTP verb of the Method resource.</p>
    pub fn http_method(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.http_method(input.into());
        self
    }
    /// <p>The HTTP verb of the Method resource.</p>
    pub fn set_http_method(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_http_method(input);
        self
    }
    /// <p>The status code for the MethodResponse resource.</p>
    pub fn status_code(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.status_code(input.into());
        self
    }
    /// <p>The status code for the MethodResponse resource.</p>
    pub fn set_status_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_status_code(input);
        self
    }
    /// Appends an item to `patchOperations`.
    ///
    /// To override the contents of this collection use [`set_patch_operations`](Self::set_patch_operations).
    ///
    /// <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p>
    pub fn patch_operations(mut self, input: crate::types::PatchOperation) -> Self {
        self.inner = self.inner.patch_operations(input);
        self
    }
    /// <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p>
    pub fn set_patch_operations(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PatchOperation>>,
    ) -> Self {
        self.inner = self.inner.set_patch_operations(input);
        self
    }
}
