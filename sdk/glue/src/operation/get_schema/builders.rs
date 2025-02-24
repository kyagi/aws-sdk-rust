// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_schema::_get_schema_output::GetSchemaOutputBuilder;

pub use crate::operation::get_schema::_get_schema_input::GetSchemaInputBuilder;

/// Fluent builder constructing a request to `GetSchema`.
///
/// <p>Describes the specified schema in detail.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetSchemaFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_schema::builders::GetSchemaInputBuilder,
}
impl GetSchemaFluentBuilder {
    /// Creates a new `GetSchema`.
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
            crate::operation::get_schema::GetSchema,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::get_schema::GetSchemaError>,
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
        crate::operation::get_schema::GetSchemaOutput,
        aws_smithy_http::result::SdkError<crate::operation::get_schema::GetSchemaError>,
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
    /// <p>This is a wrapper structure to contain schema identity fields. The structure contains:</p>
    /// <ul>
    /// <li> <p>SchemaId$SchemaArn: The Amazon Resource Name (ARN) of the schema. Either <code>SchemaArn</code> or <code>SchemaName</code> and <code>RegistryName</code> has to be provided.</p> </li>
    /// <li> <p>SchemaId$SchemaName: The name of the schema. Either <code>SchemaArn</code> or <code>SchemaName</code> and <code>RegistryName</code> has to be provided.</p> </li>
    /// </ul>
    pub fn schema_id(mut self, input: crate::types::SchemaId) -> Self {
        self.inner = self.inner.schema_id(input);
        self
    }
    /// <p>This is a wrapper structure to contain schema identity fields. The structure contains:</p>
    /// <ul>
    /// <li> <p>SchemaId$SchemaArn: The Amazon Resource Name (ARN) of the schema. Either <code>SchemaArn</code> or <code>SchemaName</code> and <code>RegistryName</code> has to be provided.</p> </li>
    /// <li> <p>SchemaId$SchemaName: The name of the schema. Either <code>SchemaArn</code> or <code>SchemaName</code> and <code>RegistryName</code> has to be provided.</p> </li>
    /// </ul>
    pub fn set_schema_id(mut self, input: std::option::Option<crate::types::SchemaId>) -> Self {
        self.inner = self.inner.set_schema_id(input);
        self
    }
}
