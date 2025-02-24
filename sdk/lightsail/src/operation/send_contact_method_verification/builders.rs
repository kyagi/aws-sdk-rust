// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::send_contact_method_verification::_send_contact_method_verification_output::SendContactMethodVerificationOutputBuilder;

pub use crate::operation::send_contact_method_verification::_send_contact_method_verification_input::SendContactMethodVerificationInputBuilder;

/// Fluent builder constructing a request to `SendContactMethodVerification`.
///
/// <p>Sends a verification request to an email contact method to ensure it's owned by the requester. SMS contact methods don't need to be verified.</p>
/// <p>A contact method is used to send you notifications about your Amazon Lightsail resources. You can add one email address and one mobile phone number contact method in each Amazon Web Services Region. However, SMS text messaging is not supported in some Amazon Web Services Regions, and SMS text messages cannot be sent to some countries/regions. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-notifications">Notifications in Amazon Lightsail</a>.</p>
/// <p>A verification request is sent to the contact method when you initially create it. Use this action to send another verification request if a previous verification request was deleted, or has expired.</p> <important>
/// <p>Notifications are not sent to an email contact method until after it is verified, and confirmed as valid.</p>
/// </important>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct SendContactMethodVerificationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::send_contact_method_verification::builders::SendContactMethodVerificationInputBuilder
            }
impl SendContactMethodVerificationFluentBuilder {
    /// Creates a new `SendContactMethodVerification`.
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
            crate::operation::send_contact_method_verification::SendContactMethodVerification,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::send_contact_method_verification::SendContactMethodVerificationError,
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
        crate::operation::send_contact_method_verification::SendContactMethodVerificationOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::send_contact_method_verification::SendContactMethodVerificationError,
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
    /// <p>The protocol to verify, such as <code>Email</code> or <code>SMS</code> (text messaging).</p>
    pub fn protocol(mut self, input: crate::types::ContactMethodVerificationProtocol) -> Self {
        self.inner = self.inner.protocol(input);
        self
    }
    /// <p>The protocol to verify, such as <code>Email</code> or <code>SMS</code> (text messaging).</p>
    pub fn set_protocol(
        mut self,
        input: std::option::Option<crate::types::ContactMethodVerificationProtocol>,
    ) -> Self {
        self.inner = self.inner.set_protocol(input);
        self
    }
}
