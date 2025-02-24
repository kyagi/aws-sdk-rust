// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::search_channels::_search_channels_output::SearchChannelsOutputBuilder;

pub use crate::operation::search_channels::_search_channels_input::SearchChannelsInputBuilder;

/// Fluent builder constructing a request to `SearchChannels`.
///
/// <p>Allows the <code>ChimeBearer</code> to search channels by channel members. Users or bots can search across the channels that they belong to. Users in the <code>AppInstanceAdmin</code> role can search across all channels.</p>
/// <p>The <code>x-amz-chime-bearer</code> request header is mandatory. Use the ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call as the value in the header.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct SearchChannelsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::search_channels::builders::SearchChannelsInputBuilder,
}
impl SearchChannelsFluentBuilder {
    /// Creates a new `SearchChannels`.
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
            crate::operation::search_channels::SearchChannels,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::search_channels::SearchChannelsError>,
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
        crate::operation::search_channels::SearchChannelsOutput,
        aws_smithy_http::result::SdkError<crate::operation::search_channels::SearchChannelsError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::search_channels::paginator::SearchChannelsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::search_channels::paginator::SearchChannelsPaginator {
        crate::operation::search_channels::paginator::SearchChannelsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user making the API call.</p>
    pub fn chime_bearer(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.chime_bearer(input.into());
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user making the API call.</p>
    pub fn set_chime_bearer(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_chime_bearer(input);
        self
    }
    /// Appends an item to `Fields`.
    ///
    /// To override the contents of this collection use [`set_fields`](Self::set_fields).
    ///
    /// <p>A list of the <code>Field</code> objects in the channel being searched.</p>
    pub fn fields(mut self, input: crate::types::SearchField) -> Self {
        self.inner = self.inner.fields(input);
        self
    }
    /// <p>A list of the <code>Field</code> objects in the channel being searched.</p>
    pub fn set_fields(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::SearchField>>,
    ) -> Self {
        self.inner = self.inner.set_fields(input);
        self
    }
    /// <p>The maximum number of channels that you want returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of channels that you want returned.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
