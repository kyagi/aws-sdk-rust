// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_services::_describe_services_output::DescribeServicesOutputBuilder;

pub use crate::operation::describe_services::_describe_services_input::DescribeServicesInputBuilder;

/// Fluent builder constructing a request to `DescribeServices`.
///
/// <p>Returns the current list of Amazon Web Services services and a list of service categories for each service. You then use service names and categories in your <code>CreateCase</code> requests. Each Amazon Web Services service has its own set of categories.</p>
/// <p>The service codes and category codes correspond to the values that appear in the <b>Service</b> and <b>Category</b> lists on the Amazon Web Services Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page. The values in those fields don't necessarily match the service codes and categories returned by the <code>DescribeServices</code> operation. Always use the service codes and categories that the <code>DescribeServices</code> operation returns, so that you have the most recent set of service and category codes.</p> <note>
/// <ul>
/// <li> <p>You must have a Business, Enterprise On-Ramp, or Enterprise Support plan to use the Amazon Web Services Support API. </p> </li>
/// <li> <p>If you call the Amazon Web Services Support API from an account that doesn't have a Business, Enterprise On-Ramp, or Enterprise Support plan, the <code>SubscriptionRequiredException</code> error message appears. For information about changing your support plan, see <a href="http://aws.amazon.com/premiumsupport/">Amazon Web Services Support</a>.</p> </li>
/// </ul>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeServicesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_services::builders::DescribeServicesInputBuilder,
}
impl DescribeServicesFluentBuilder {
    /// Creates a new `DescribeServices`.
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
            crate::operation::describe_services::DescribeServices,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_services::DescribeServicesError,
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
        crate::operation::describe_services::DescribeServicesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_services::DescribeServicesError,
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
    /// Appends an item to `serviceCodeList`.
    ///
    /// To override the contents of this collection use [`set_service_code_list`](Self::set_service_code_list).
    ///
    /// <p>A JSON-formatted list of service codes available for Amazon Web Services services.</p>
    pub fn service_code_list(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.service_code_list(input.into());
        self
    }
    /// <p>A JSON-formatted list of service codes available for Amazon Web Services services.</p>
    pub fn set_service_code_list(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_service_code_list(input);
        self
    }
    /// <p>The language in which Amazon Web Services Support handles the case. Amazon Web Services Support currently supports English ("en") and Japanese ("ja"). You must specify the ISO 639-1 code for the <code>language</code> parameter if you want support in that language.</p>
    pub fn language(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.language(input.into());
        self
    }
    /// <p>The language in which Amazon Web Services Support handles the case. Amazon Web Services Support currently supports English ("en") and Japanese ("ja"). You must specify the ISO 639-1 code for the <code>language</code> parameter if you want support in that language.</p>
    pub fn set_language(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_language(input);
        self
    }
}
