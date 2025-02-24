// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_configuration_aggregators::_describe_configuration_aggregators_output::DescribeConfigurationAggregatorsOutputBuilder;

pub use crate::operation::describe_configuration_aggregators::_describe_configuration_aggregators_input::DescribeConfigurationAggregatorsInputBuilder;

/// Fluent builder constructing a request to `DescribeConfigurationAggregators`.
///
/// <p>Returns the details of one or more configuration aggregators. If the configuration aggregator is not specified, this action returns the details for all the configuration aggregators associated with the account. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeConfigurationAggregatorsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_configuration_aggregators::builders::DescribeConfigurationAggregatorsInputBuilder
            }
impl DescribeConfigurationAggregatorsFluentBuilder {
    /// Creates a new `DescribeConfigurationAggregators`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::describe_configuration_aggregators::DescribeConfigurationAggregators, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::describe_configuration_aggregators::DescribeConfigurationAggregatorsError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::describe_configuration_aggregators::DescribeConfigurationAggregatorsOutput, aws_smithy_http::result::SdkError<crate::operation::describe_configuration_aggregators::DescribeConfigurationAggregatorsError>>
                     {
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_configuration_aggregators::paginator::DescribeConfigurationAggregatorsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_configuration_aggregators::paginator::DescribeConfigurationAggregatorsPaginator{
        crate::operation::describe_configuration_aggregators::paginator::DescribeConfigurationAggregatorsPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `ConfigurationAggregatorNames`.
    ///
    /// To override the contents of this collection use [`set_configuration_aggregator_names`](Self::set_configuration_aggregator_names).
    ///
    /// <p>The name of the configuration aggregators.</p>
    pub fn configuration_aggregator_names(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.configuration_aggregator_names(input.into());
        self
    }
    /// <p>The name of the configuration aggregators.</p>
    pub fn set_configuration_aggregator_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_configuration_aggregator_names(input);
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of configuration aggregators returned on each page. The default is maximum. If you specify 0, Config uses the default.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of configuration aggregators returned on each page. The default is maximum. If you specify 0, Config uses the default.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
}
