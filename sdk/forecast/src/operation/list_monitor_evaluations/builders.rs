// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_monitor_evaluations::_list_monitor_evaluations_output::ListMonitorEvaluationsOutputBuilder;

pub use crate::operation::list_monitor_evaluations::_list_monitor_evaluations_input::ListMonitorEvaluationsInputBuilder;

/// Fluent builder constructing a request to `ListMonitorEvaluations`.
///
/// <p>Returns a list of the monitoring evaluation results and predictor events collected by the monitor resource during different windows of time.</p>
/// <p>For information about monitoring see <code>predictor-monitoring</code>. For more information about retrieving monitoring results see <a href="https://docs.aws.amazon.com/forecast/latest/dg/predictor-monitoring-results.html">Viewing Monitoring Results</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListMonitorEvaluationsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_monitor_evaluations::builders::ListMonitorEvaluationsInputBuilder,
}
impl ListMonitorEvaluationsFluentBuilder {
    /// Creates a new `ListMonitorEvaluations`.
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
            crate::operation::list_monitor_evaluations::ListMonitorEvaluations,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_monitor_evaluations::ListMonitorEvaluationsError,
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
        crate::operation::list_monitor_evaluations::ListMonitorEvaluationsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_monitor_evaluations::ListMonitorEvaluationsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_monitor_evaluations::paginator::ListMonitorEvaluationsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_monitor_evaluations::paginator::ListMonitorEvaluationsPaginator
    {
        crate::operation::list_monitor_evaluations::paginator::ListMonitorEvaluationsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of monitoring results to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of monitoring results to return.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the monitor resource to get results from.</p>
    pub fn monitor_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.monitor_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the monitor resource to get results from.</p>
    pub fn set_monitor_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_monitor_arn(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>An array of filters. For each filter, provide a condition and a match statement. The condition is either <code>IS</code> or <code>IS_NOT</code>, which specifies whether to include or exclude the resources that match the statement from the list. The match statement consists of a key and a value.</p>
    /// <p> <b>Filter properties</b> </p>
    /// <ul>
    /// <li> <p> <code>Condition</code> - The condition to apply. Valid values are <code>IS</code> and <code>IS_NOT</code>.</p> </li>
    /// <li> <p> <code>Key</code> - The name of the parameter to filter on. The only valid value is <code>EvaluationState</code>.</p> </li>
    /// <li> <p> <code>Value</code> - The value to match. Valid values are only <code>SUCCESS</code> or <code>FAILURE</code>.</p> </li>
    /// </ul>
    /// <p>For example, to list only successful monitor evaluations, you would specify:</p>
    /// <p> <code>"Filters": [ { "Condition": "IS", "Key": "EvaluationState", "Value": "SUCCESS" } ]</code> </p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>An array of filters. For each filter, provide a condition and a match statement. The condition is either <code>IS</code> or <code>IS_NOT</code>, which specifies whether to include or exclude the resources that match the statement from the list. The match statement consists of a key and a value.</p>
    /// <p> <b>Filter properties</b> </p>
    /// <ul>
    /// <li> <p> <code>Condition</code> - The condition to apply. Valid values are <code>IS</code> and <code>IS_NOT</code>.</p> </li>
    /// <li> <p> <code>Key</code> - The name of the parameter to filter on. The only valid value is <code>EvaluationState</code>.</p> </li>
    /// <li> <p> <code>Value</code> - The value to match. Valid values are only <code>SUCCESS</code> or <code>FAILURE</code>.</p> </li>
    /// </ul>
    /// <p>For example, to list only successful monitor evaluations, you would specify:</p>
    /// <p> <code>"Filters": [ { "Condition": "IS", "Key": "EvaluationState", "Value": "SUCCESS" } ]</code> </p>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
}
