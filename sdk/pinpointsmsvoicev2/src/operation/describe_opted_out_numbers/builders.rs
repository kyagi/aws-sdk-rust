// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_opted_out_numbers::_describe_opted_out_numbers_output::DescribeOptedOutNumbersOutputBuilder;

pub use crate::operation::describe_opted_out_numbers::_describe_opted_out_numbers_input::DescribeOptedOutNumbersInputBuilder;

/// Fluent builder constructing a request to `DescribeOptedOutNumbers`.
///
/// <p>Describes the specified opted out destination numbers or all opted out destination numbers in an opt-out list.</p>
/// <p>If you specify opted out numbers, the output includes information for only the specified opted out numbers. If you specify filters, the output includes information for only those opted out numbers that meet the filter criteria. If you don't specify opted out numbers or filters, the output includes information for all opted out destination numbers in your opt-out list.</p>
/// <p>If you specify an opted out number that isn't valid, an Error is returned.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeOptedOutNumbersFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::describe_opted_out_numbers::builders::DescribeOptedOutNumbersInputBuilder,
}
impl DescribeOptedOutNumbersFluentBuilder {
    /// Creates a new `DescribeOptedOutNumbers`.
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
            crate::operation::describe_opted_out_numbers::DescribeOptedOutNumbers,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_opted_out_numbers::DescribeOptedOutNumbersError,
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
        crate::operation::describe_opted_out_numbers::DescribeOptedOutNumbersOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_opted_out_numbers::DescribeOptedOutNumbersError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_opted_out_numbers::paginator::DescribeOptedOutNumbersPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_opted_out_numbers::paginator::DescribeOptedOutNumbersPaginator
    {
        crate::operation::describe_opted_out_numbers::paginator::DescribeOptedOutNumbersPaginator::new(self.handle, self.inner)
    }
    /// <p>The OptOutListName or OptOutListArn of the OptOutList. You can use <code>DescribeOptOutLists</code> to find the values for OptOutListName and OptOutListArn.</p>
    pub fn opt_out_list_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.opt_out_list_name(input.into());
        self
    }
    /// <p>The OptOutListName or OptOutListArn of the OptOutList. You can use <code>DescribeOptOutLists</code> to find the values for OptOutListName and OptOutListArn.</p>
    pub fn set_opt_out_list_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_opt_out_list_name(input);
        self
    }
    /// Appends an item to `OptedOutNumbers`.
    ///
    /// To override the contents of this collection use [`set_opted_out_numbers`](Self::set_opted_out_numbers).
    ///
    /// <p>An array of phone numbers to search for in the OptOutList.</p>
    pub fn opted_out_numbers(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.opted_out_numbers(input.into());
        self
    }
    /// <p>An array of phone numbers to search for in the OptOutList.</p>
    pub fn set_opted_out_numbers(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_opted_out_numbers(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>An array of OptedOutFilter objects to filter the results on.</p>
    pub fn filters(mut self, input: crate::types::OptedOutFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>An array of OptedOutFilter objects to filter the results on.</p>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::OptedOutFilter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
