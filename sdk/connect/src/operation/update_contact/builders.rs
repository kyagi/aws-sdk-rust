// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_contact::_update_contact_output::UpdateContactOutputBuilder;

pub use crate::operation::update_contact::_update_contact_input::UpdateContactInputBuilder;

/// Fluent builder constructing a request to `UpdateContact`.
///
/// <p>This API is in preview release for Amazon Connect and is subject to change.</p>
/// <p>Adds or updates user-defined contact information associated with the specified contact. At least one field to be updated must be present in the request.</p> <important>
/// <p>You can add or update user-defined contact information for both ongoing and completed contacts.</p>
/// </important>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateContactFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_contact::builders::UpdateContactInputBuilder,
}
impl UpdateContactFluentBuilder {
    /// Creates a new `UpdateContact`.
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
            crate::operation::update_contact::UpdateContact,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_contact::UpdateContactError>,
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
        crate::operation::update_contact::UpdateContactOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_contact::UpdateContactError>,
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
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with your contact center.</p>
    pub fn contact_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.contact_id(input.into());
        self
    }
    /// <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with your contact center.</p>
    pub fn set_contact_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_contact_id(input);
        self
    }
    /// <p>The name of the contact.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the contact.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The description of the contact.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the contact.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Adds a key-value pair to `References`.
    ///
    /// To override the contents of this collection use [`set_references`](Self::set_references).
    ///
    /// <p>Well-formed data on contact, shown to agents on Contact Control Panel (CCP).</p>
    pub fn references(
        mut self,
        k: impl Into<std::string::String>,
        v: crate::types::Reference,
    ) -> Self {
        self.inner = self.inner.references(k.into(), v);
        self
    }
    /// <p>Well-formed data on contact, shown to agents on Contact Control Panel (CCP).</p>
    pub fn set_references(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, crate::types::Reference>,
        >,
    ) -> Self {
        self.inner = self.inner.set_references(input);
        self
    }
}
