// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_regions::_describe_regions_output::DescribeRegionsOutputBuilder;

pub use crate::operation::describe_regions::_describe_regions_input::DescribeRegionsInputBuilder;

/// Fluent builder constructing a request to `DescribeRegions`.
///
/// <p>Describes the Regions that are enabled for your account, or all Regions.</p>
/// <p>For a list of the Regions supported by Amazon EC2, see <a href="https://docs.aws.amazon.com/general/latest/gr/ec2-service.html"> Amazon Elastic Compute Cloud endpoints and quotas</a>.</p>
/// <p>For information about enabling and disabling Regions for your account, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html">Managing Amazon Web Services Regions</a> in the <i>Amazon Web Services General Reference</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeRegionsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_regions::builders::DescribeRegionsInputBuilder,
}
impl DescribeRegionsFluentBuilder {
    /// Creates a new `DescribeRegions`.
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
            crate::operation::describe_regions::DescribeRegions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::describe_regions::DescribeRegionsError>,
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
        crate::operation::describe_regions::DescribeRegionsOutput,
        aws_smithy_http::result::SdkError<crate::operation::describe_regions::DescribeRegionsError>,
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
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>endpoint</code> - The endpoint of the Region (for example, <code>ec2.us-east-1.amazonaws.com</code>).</p> </li>
    /// <li> <p> <code>opt-in-status</code> - The opt-in status of the Region (<code>opt-in-not-required</code> | <code>opted-in</code> | <code>not-opted-in</code>).</p> </li>
    /// <li> <p> <code>region-name</code> - The name of the Region (for example, <code>us-east-1</code>).</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>endpoint</code> - The endpoint of the Region (for example, <code>ec2.us-east-1.amazonaws.com</code>).</p> </li>
    /// <li> <p> <code>opt-in-status</code> - The opt-in status of the Region (<code>opt-in-not-required</code> | <code>opted-in</code> | <code>not-opted-in</code>).</p> </li>
    /// <li> <p> <code>region-name</code> - The name of the Region (for example, <code>us-east-1</code>).</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// Appends an item to `RegionNames`.
    ///
    /// To override the contents of this collection use [`set_region_names`](Self::set_region_names).
    ///
    /// <p>The names of the Regions. You can specify any Regions, whether they are enabled and disabled for your account.</p>
    pub fn region_names(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.region_names(input.into());
        self
    }
    /// <p>The names of the Regions. You can specify any Regions, whether they are enabled and disabled for your account.</p>
    pub fn set_region_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_region_names(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Indicates whether to display all Regions, including Regions that are disabled for your account.</p>
    pub fn all_regions(mut self, input: bool) -> Self {
        self.inner = self.inner.all_regions(input);
        self
    }
    /// <p>Indicates whether to display all Regions, including Regions that are disabled for your account.</p>
    pub fn set_all_regions(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_all_regions(input);
        self
    }
}
