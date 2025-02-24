// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_db_instance::_delete_db_instance_output::DeleteDbInstanceOutputBuilder;

pub use crate::operation::delete_db_instance::_delete_db_instance_input::DeleteDbInstanceInputBuilder;

/// Fluent builder constructing a request to `DeleteDBInstance`.
///
/// <p>Deletes a previously provisioned instance.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteDBInstanceFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_db_instance::builders::DeleteDbInstanceInputBuilder,
}
impl DeleteDBInstanceFluentBuilder {
    /// Creates a new `DeleteDBInstance`.
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
            crate::operation::delete_db_instance::DeleteDBInstance,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_db_instance::DeleteDBInstanceError,
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
        crate::operation::delete_db_instance::DeleteDbInstanceOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_db_instance::DeleteDBInstanceError,
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
    /// <p>The instance identifier for the instance to be deleted. This parameter isn't case sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the name of an existing instance.</p> </li>
    /// </ul>
    pub fn db_instance_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.db_instance_identifier(input.into());
        self
    }
    /// <p>The instance identifier for the instance to be deleted. This parameter isn't case sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the name of an existing instance.</p> </li>
    /// </ul>
    pub fn set_db_instance_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_db_instance_identifier(input);
        self
    }
}
