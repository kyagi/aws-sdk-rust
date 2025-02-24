// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_fleet_location_capacity::_describe_fleet_location_capacity_output::DescribeFleetLocationCapacityOutputBuilder;

pub use crate::operation::describe_fleet_location_capacity::_describe_fleet_location_capacity_input::DescribeFleetLocationCapacityInputBuilder;

/// Fluent builder constructing a request to `DescribeFleetLocationCapacity`.
///
/// <p>Retrieves the resource capacity settings for a fleet location. The data returned includes the current capacity (number of EC2 instances) and some scaling settings for the requested fleet location. Use this operation to retrieve capacity information for a fleet's remote location or home Region (you can also retrieve home Region capacity by calling <code>DescribeFleetCapacity</code>).</p>
/// <p>To retrieve capacity data, identify a fleet and location. </p>
/// <p>If successful, a <code>FleetCapacity</code> object is returned for the requested fleet location. </p>
/// <p> <b>Learn more</b> </p>
/// <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift fleets</a> </p>
/// <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html#gamelift-metrics-fleet">GameLift metrics for fleets</a> </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeFleetLocationCapacityFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_fleet_location_capacity::builders::DescribeFleetLocationCapacityInputBuilder
            }
impl DescribeFleetLocationCapacityFluentBuilder {
    /// Creates a new `DescribeFleetLocationCapacity`.
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
            crate::operation::describe_fleet_location_capacity::DescribeFleetLocationCapacity,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_fleet_location_capacity::DescribeFleetLocationCapacityError,
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
        crate::operation::describe_fleet_location_capacity::DescribeFleetLocationCapacityOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_fleet_location_capacity::DescribeFleetLocationCapacityError,
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
    /// <p>A unique identifier for the fleet to request location capacity for. You can use either the fleet ID or ARN value.</p>
    pub fn fleet_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.fleet_id(input.into());
        self
    }
    /// <p>A unique identifier for the fleet to request location capacity for. You can use either the fleet ID or ARN value.</p>
    pub fn set_fleet_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_fleet_id(input);
        self
    }
    /// <p>The fleet location to retrieve capacity information for. Specify a location in the form of an Amazon Web Services Region code, such as <code>us-west-2</code>.</p>
    pub fn location(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.location(input.into());
        self
    }
    /// <p>The fleet location to retrieve capacity information for. Specify a location in the form of an Amazon Web Services Region code, such as <code>us-west-2</code>.</p>
    pub fn set_location(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_location(input);
        self
    }
}
