// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_configs::_list_configs_output::ListConfigsOutputBuilder;

pub use crate::operation::list_configs::_list_configs_input::ListConfigsInputBuilder;

/// Fluent builder constructing a request to `ListConfigs`.
///
/// <p>Returns a list of <code>Config</code> objects.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListConfigsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_configs::builders::ListConfigsInputBuilder,
}
impl ListConfigsFluentBuilder {
    /// Creates a new `ListConfigs`.
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
            crate::operation::list_configs::ListConfigs,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::list_configs::ListConfigsError>,
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
        crate::operation::list_configs::ListConfigsOutput,
        aws_smithy_http::result::SdkError<crate::operation::list_configs::ListConfigsError>,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_configs::paginator::ListConfigsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_configs::paginator::ListConfigsPaginator {
        crate::operation::list_configs::paginator::ListConfigsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>Maximum number of <code>Configs</code> returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Maximum number of <code>Configs</code> returned.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Next token returned in the request of a previous <code>ListConfigs</code> call. Used to get the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Next token returned in the request of a previous <code>ListConfigs</code> call. Used to get the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
