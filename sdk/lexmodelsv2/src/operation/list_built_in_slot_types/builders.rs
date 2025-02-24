// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_built_in_slot_types::_list_built_in_slot_types_output::ListBuiltInSlotTypesOutputBuilder;

pub use crate::operation::list_built_in_slot_types::_list_built_in_slot_types_input::ListBuiltInSlotTypesInputBuilder;

/// Fluent builder constructing a request to `ListBuiltInSlotTypes`.
///
/// <p>Gets a list of built-in slot types that meet the specified criteria.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListBuiltInSlotTypesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_built_in_slot_types::builders::ListBuiltInSlotTypesInputBuilder,
}
impl ListBuiltInSlotTypesFluentBuilder {
    /// Creates a new `ListBuiltInSlotTypes`.
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
            crate::operation::list_built_in_slot_types::ListBuiltInSlotTypes,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_built_in_slot_types::ListBuiltInSlotTypesError,
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
        crate::operation::list_built_in_slot_types::ListBuiltInSlotTypesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_built_in_slot_types::ListBuiltInSlotTypesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_built_in_slot_types::paginator::ListBuiltInSlotTypesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_built_in_slot_types::paginator::ListBuiltInSlotTypesPaginator {
        crate::operation::list_built_in_slot_types::paginator::ListBuiltInSlotTypesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The identifier of the language and locale of the slot types to list. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    pub fn locale_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.locale_id(input.into());
        self
    }
    /// <p>The identifier of the language and locale of the slot types to list. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    pub fn set_locale_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_locale_id(input);
        self
    }
    /// <p>Determines the sort order for the response from the <code>ListBuiltInSlotTypes</code> operation. You can choose to sort by the slot type signature in either ascending or descending order.</p>
    pub fn sort_by(mut self, input: crate::types::BuiltInSlotTypeSortBy) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>Determines the sort order for the response from the <code>ListBuiltInSlotTypes</code> operation. You can choose to sort by the slot type signature in either ascending or descending order.</p>
    pub fn set_sort_by(
        mut self,
        input: std::option::Option<crate::types::BuiltInSlotTypeSortBy>,
    ) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>The maximum number of built-in slot types to return in each page of results. If there are fewer results than the max page size, only the actual number of results are returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of built-in slot types to return in each page of results. If there are fewer results than the max page size, only the actual number of results are returned.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>If the response from the <code>ListBuiltInSlotTypes</code> operation contains more results than specified in the <code>maxResults</code> parameter, a token is returned in the response. Use that token in the <code>nextToken</code> parameter to return the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the response from the <code>ListBuiltInSlotTypes</code> operation contains more results than specified in the <code>maxResults</code> parameter, a token is returned in the response. Use that token in the <code>nextToken</code> parameter to return the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
