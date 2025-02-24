// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_namespace::_describe_namespace_output::DescribeNamespaceOutputBuilder;

pub use crate::operation::describe_namespace::_describe_namespace_input::DescribeNamespaceInputBuilder;

/// Fluent builder constructing a request to `DescribeNamespace`.
///
/// <p>Gets the latest version of the user's namespace and the public version that it is tracking.</p>
#[deprecated(note = "since: 2022-08-30")]
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeNamespaceFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_namespace::builders::DescribeNamespaceInputBuilder,
}
impl DescribeNamespaceFluentBuilder {
    /// Creates a new `DescribeNamespace`.
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
            crate::operation::describe_namespace::DescribeNamespace,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_namespace::DescribeNamespaceError,
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
        crate::operation::describe_namespace::DescribeNamespaceOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_namespace::DescribeNamespaceError,
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
    /// <p>The name of the user's namespace. Set this to <code>aws</code> to get the public namespace.</p>
    pub fn namespace_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.namespace_name(input.into());
        self
    }
    /// <p>The name of the user's namespace. Set this to <code>aws</code> to get the public namespace.</p>
    pub fn set_namespace_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_namespace_name(input);
        self
    }
}
