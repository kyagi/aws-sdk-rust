// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_sampling_targets::_get_sampling_targets_output::GetSamplingTargetsOutputBuilder;

pub use crate::operation::get_sampling_targets::_get_sampling_targets_input::GetSamplingTargetsInputBuilder;

/// Fluent builder constructing a request to `GetSamplingTargets`.
///
/// <p>Requests a sampling quota for rules that the service is using to sample requests. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetSamplingTargetsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_sampling_targets::builders::GetSamplingTargetsInputBuilder,
}
impl GetSamplingTargetsFluentBuilder {
    /// Creates a new `GetSamplingTargets`.
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
            crate::operation::get_sampling_targets::GetSamplingTargets,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_sampling_targets::GetSamplingTargetsError,
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
        crate::operation::get_sampling_targets::GetSamplingTargetsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_sampling_targets::GetSamplingTargetsError,
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
    /// Appends an item to `SamplingStatisticsDocuments`.
    ///
    /// To override the contents of this collection use [`set_sampling_statistics_documents`](Self::set_sampling_statistics_documents).
    ///
    /// <p>Information about rules that the service is using to sample requests.</p>
    pub fn sampling_statistics_documents(
        mut self,
        input: crate::types::SamplingStatisticsDocument,
    ) -> Self {
        self.inner = self.inner.sampling_statistics_documents(input);
        self
    }
    /// <p>Information about rules that the service is using to sample requests.</p>
    pub fn set_sampling_statistics_documents(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::SamplingStatisticsDocument>>,
    ) -> Self {
        self.inner = self.inner.set_sampling_statistics_documents(input);
        self
    }
}
