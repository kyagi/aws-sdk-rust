// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_contact_list::_update_contact_list_output::UpdateContactListOutputBuilder;

pub use crate::operation::update_contact_list::_update_contact_list_input::UpdateContactListInputBuilder;

/// Fluent builder constructing a request to `UpdateContactList`.
///
/// <p>Updates contact list metadata. This operation does a complete replacement.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateContactListFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_contact_list::builders::UpdateContactListInputBuilder,
}
impl UpdateContactListFluentBuilder {
    /// Creates a new `UpdateContactList`.
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
            crate::operation::update_contact_list::UpdateContactList,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_contact_list::UpdateContactListError,
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
        crate::operation::update_contact_list::UpdateContactListOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_contact_list::UpdateContactListError,
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
    /// <p>The name of the contact list.</p>
    pub fn contact_list_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.contact_list_name(input.into());
        self
    }
    /// <p>The name of the contact list.</p>
    pub fn set_contact_list_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_contact_list_name(input);
        self
    }
    /// Appends an item to `Topics`.
    ///
    /// To override the contents of this collection use [`set_topics`](Self::set_topics).
    ///
    /// <p>An interest group, theme, or label within a list. A contact list can have multiple topics.</p>
    pub fn topics(mut self, input: crate::types::Topic) -> Self {
        self.inner = self.inner.topics(input);
        self
    }
    /// <p>An interest group, theme, or label within a list. A contact list can have multiple topics.</p>
    pub fn set_topics(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Topic>>,
    ) -> Self {
        self.inner = self.inner.set_topics(input);
        self
    }
    /// <p>A description of what the contact list is about.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of what the contact list is about.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
}
