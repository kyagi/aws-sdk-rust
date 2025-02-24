// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_model_version::_delete_model_version_output::DeleteModelVersionOutputBuilder;

pub use crate::operation::delete_model_version::_delete_model_version_input::DeleteModelVersionInputBuilder;

/// Fluent builder constructing a request to `DeleteModelVersion`.
///
/// <p>Deletes a model version.</p>
/// <p>You can delete models and model versions in Amazon Fraud Detector, provided that they are not associated with a detector version.</p>
/// <p> When you delete a model version, Amazon Fraud Detector permanently deletes that model version and the data is no longer stored in Amazon Fraud Detector.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteModelVersionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_model_version::builders::DeleteModelVersionInputBuilder,
}
impl DeleteModelVersionFluentBuilder {
    /// Creates a new `DeleteModelVersion`.
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
            crate::operation::delete_model_version::DeleteModelVersion,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_model_version::DeleteModelVersionError,
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
        crate::operation::delete_model_version::DeleteModelVersionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_model_version::DeleteModelVersionError,
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
    /// <p>The model ID of the model version to delete.</p>
    pub fn model_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.model_id(input.into());
        self
    }
    /// <p>The model ID of the model version to delete.</p>
    pub fn set_model_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_model_id(input);
        self
    }
    /// <p>The model type of the model version to delete.</p>
    pub fn model_type(mut self, input: crate::types::ModelTypeEnum) -> Self {
        self.inner = self.inner.model_type(input);
        self
    }
    /// <p>The model type of the model version to delete.</p>
    pub fn set_model_type(
        mut self,
        input: std::option::Option<crate::types::ModelTypeEnum>,
    ) -> Self {
        self.inner = self.inner.set_model_type(input);
        self
    }
    /// <p>The model version number of the model version to delete.</p>
    pub fn model_version_number(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.model_version_number(input.into());
        self
    }
    /// <p>The model version number of the model version to delete.</p>
    pub fn set_model_version_number(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_model_version_number(input);
        self
    }
}
