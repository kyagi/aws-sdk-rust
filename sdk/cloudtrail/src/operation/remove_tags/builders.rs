// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_tags::_remove_tags_output::RemoveTagsOutputBuilder;

pub use crate::operation::remove_tags::_remove_tags_input::RemoveTagsInputBuilder;

/// Fluent builder constructing a request to `RemoveTags`.
///
/// <p>Removes the specified tags from a trail, event data store, or channel.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RemoveTagsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::remove_tags::builders::RemoveTagsInputBuilder,
}
impl RemoveTagsFluentBuilder {
    /// Creates a new `RemoveTags`.
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
            crate::operation::remove_tags::RemoveTags,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::remove_tags::RemoveTagsError>,
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
        crate::operation::remove_tags::RemoveTagsOutput,
        aws_smithy_http::result::SdkError<crate::operation::remove_tags::RemoveTagsError>,
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
    /// <p>Specifies the ARN of the trail, event data store, or channel from which tags should be removed.</p>
    /// <p> Example trail ARN format: <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    /// <p>Example event data store ARN format: <code>arn:aws:cloudtrail:us-east-2:12345678910:eventdatastore/EXAMPLE-f852-4e8f-8bd1-bcf6cEXAMPLE</code> </p>
    /// <p>Example channel ARN format: <code>arn:aws:cloudtrail:us-east-2:123456789012:channel/01234567890</code> </p>
    pub fn resource_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>Specifies the ARN of the trail, event data store, or channel from which tags should be removed.</p>
    /// <p> Example trail ARN format: <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    /// <p>Example event data store ARN format: <code>arn:aws:cloudtrail:us-east-2:12345678910:eventdatastore/EXAMPLE-f852-4e8f-8bd1-bcf6cEXAMPLE</code> </p>
    /// <p>Example channel ARN format: <code>arn:aws:cloudtrail:us-east-2:123456789012:channel/01234567890</code> </p>
    pub fn set_resource_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// Appends an item to `TagsList`.
    ///
    /// To override the contents of this collection use [`set_tags_list`](Self::set_tags_list).
    ///
    /// <p>Specifies a list of tags to be removed.</p>
    pub fn tags_list(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags_list(input);
        self
    }
    /// <p>Specifies a list of tags to be removed.</p>
    pub fn set_tags_list(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags_list(input);
        self
    }
}
