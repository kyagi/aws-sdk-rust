// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_crl::_import_crl_output::ImportCrlOutputBuilder;

pub use crate::operation::import_crl::_import_crl_input::ImportCrlInputBuilder;

/// Fluent builder constructing a request to `ImportCrl`.
///
/// <p>Imports the certificate revocation list (CRL). CRl is a list of certificates that have been revoked by the issuing certificate Authority (CA). IAM Roles Anywhere validates against the crl list before issuing credentials. </p>
/// <p> <b>Required permissions: </b> <code>rolesanywhere:ImportCrl</code>. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ImportCrlFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_crl::builders::ImportCrlInputBuilder,
}
impl ImportCrlFluentBuilder {
    /// Creates a new `ImportCrl`.
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
            crate::operation::import_crl::ImportCrl,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::import_crl::ImportCrlError>,
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
        crate::operation::import_crl::ImportCrlOutput,
        aws_smithy_http::result::SdkError<crate::operation::import_crl::ImportCrlError>,
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
    /// <p>The name of the certificate revocation list (CRL).</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the certificate revocation list (CRL).</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The x509 v3 specified certificate revocation list</p>
    pub fn crl_data(mut self, input: aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.crl_data(input);
        self
    }
    /// <p>The x509 v3 specified certificate revocation list</p>
    pub fn set_crl_data(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_crl_data(input);
        self
    }
    /// <p>Specifies whether the certificate revocation list (CRL) is enabled.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.enabled(input);
        self
    }
    /// <p>Specifies whether the certificate revocation list (CRL) is enabled.</p>
    pub fn set_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enabled(input);
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags to attach to the certificate revocation list (CRL).</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of tags to attach to the certificate revocation list (CRL).</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The ARN of the TrustAnchor the certificate revocation list (CRL) will provide revocation for.</p>
    pub fn trust_anchor_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.trust_anchor_arn(input.into());
        self
    }
    /// <p>The ARN of the TrustAnchor the certificate revocation list (CRL) will provide revocation for.</p>
    pub fn set_trust_anchor_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_trust_anchor_arn(input);
        self
    }
}
