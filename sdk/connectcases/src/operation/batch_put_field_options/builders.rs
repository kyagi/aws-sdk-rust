// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_put_field_options::_batch_put_field_options_output::BatchPutFieldOptionsOutputBuilder;

pub use crate::operation::batch_put_field_options::_batch_put_field_options_input::BatchPutFieldOptionsInputBuilder;

/// Fluent builder constructing a request to `BatchPutFieldOptions`.
///
/// <p>Creates and updates a set of field options for a single select field in a Cases domain.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct BatchPutFieldOptionsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_put_field_options::builders::BatchPutFieldOptionsInputBuilder,
}
impl BatchPutFieldOptionsFluentBuilder {
    /// Creates a new `BatchPutFieldOptions`.
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
            crate::operation::batch_put_field_options::BatchPutFieldOptions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_put_field_options::BatchPutFieldOptionsError,
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
        crate::operation::batch_put_field_options::BatchPutFieldOptionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_put_field_options::BatchPutFieldOptionsError,
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
    /// <p>The unique identifier of the Cases domain. </p>
    pub fn domain_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.domain_id(input.into());
        self
    }
    /// <p>The unique identifier of the Cases domain. </p>
    pub fn set_domain_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_domain_id(input);
        self
    }
    /// <p>The unique identifier of a field.</p>
    pub fn field_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.field_id(input.into());
        self
    }
    /// <p>The unique identifier of a field.</p>
    pub fn set_field_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_field_id(input);
        self
    }
    /// Appends an item to `options`.
    ///
    /// To override the contents of this collection use [`set_options`](Self::set_options).
    ///
    /// <p>A list of <code>FieldOption</code> objects.</p>
    pub fn options(mut self, input: crate::types::FieldOption) -> Self {
        self.inner = self.inner.options(input);
        self
    }
    /// <p>A list of <code>FieldOption</code> objects.</p>
    pub fn set_options(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::FieldOption>>,
    ) -> Self {
        self.inner = self.inner.set_options(input);
        self
    }
}
