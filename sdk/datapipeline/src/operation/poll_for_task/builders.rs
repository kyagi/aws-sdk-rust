// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::poll_for_task::_poll_for_task_output::PollForTaskOutputBuilder;

pub use crate::operation::poll_for_task::_poll_for_task_input::PollForTaskInputBuilder;

/// Fluent builder constructing a request to `PollForTask`.
///
/// <p>Task runners call <code>PollForTask</code> to receive a task to perform from AWS Data Pipeline. The task runner specifies which tasks it can perform by setting a value for the <code>workerGroup</code> parameter. The task returned can come from any of the pipelines that match the <code>workerGroup</code> value passed in by the task runner and that was launched using the IAM user credentials specified by the task runner.</p>
/// <p>If tasks are ready in the work queue, <code>PollForTask</code> returns a response immediately. If no tasks are available in the queue, <code>PollForTask</code> uses long-polling and holds on to a poll connection for up to a 90 seconds, during which time the first newly scheduled task is handed to the task runner. To accomodate this, set the socket timeout in your task runner to 90 seconds. The task runner should not call <code>PollForTask</code> again on the same <code>workerGroup</code> until it receives a response, and this can take up to 90 seconds. </p> <examples>
/// <request>
/// POST / HTTP/1.1 Content-Type: application/x-amz-json-1.1 X-Amz-Target: DataPipeline.PollForTask Content-Length: 59 Host: datapipeline.us-east-1.amazonaws.com X-Amz-Date: Mon, 12 Nov 2012 17:49:52 GMT Authorization: AuthParams {"workerGroup": "MyworkerGroup", "hostname": "example.com"}
/// </request>
/// <response>
/// x-amzn-RequestId: 41c713d2-0775-11e2-af6f-6bc7a6be60d9 Content-Type: application/x-amz-json-1.1 Content-Length: 39 Date: Mon, 12 Nov 2012 17:50:53 GMT {"taskObject": {"attemptId": "@SayHello_2012-12-12T00:00:00_Attempt=1", "objects": {"@SayHello_2012-12-12T00:00:00_Attempt=1": {"fields": [ {"key": "@componentParent", "refValue": "SayHello"}, {"key": "@scheduledStartTime", "stringValue": "2012-12-12T00:00:00"}, {"key": "parent", "refValue": "SayHello"}, {"key": "@sphere", "stringValue": "ATTEMPT"}, {"key": "workerGroup", "stringValue": "workerGroup"}, {"key": "@instanceParent", "refValue": "@SayHello_2012-12-12T00:00:00"}, {"key": "type", "stringValue": "ShellCommandActivity"}, {"key": "@status", "stringValue": "WAITING_FOR_RUNNER"}, {"key": "@version", "stringValue": "1"}, {"key": "schedule", "refValue": "Schedule"}, {"key": "@actualStartTime", "stringValue": "2012-12-13T01:40:50"}, {"key": "command", "stringValue": "echo hello"}, {"key": "@scheduledEndTime", "stringValue": "2012-12-12T01:00:00"}, {"key": "@activeInstances", "refValue": "@SayHello_2012-12-12T00:00:00"}, {"key": "@pipelineId", "stringValue": "df-0937003356ZJEXAMPLE"} ], "id": "@SayHello_2012-12-12T00:00:00_Attempt=1", "name": "@SayHello_2012-12-12T00:00:00_Attempt=1"} }, "pipelineId": "df-0937003356ZJEXAMPLE", "taskId": "2xaM4wRs5zOsIH+g9U3oVHfAgAlbSqU6XduncB0HhZ3xMnmvfePZPn4dIbYXHyWyRK+cU15MqDHwdrvftx/4wv+sNS4w34vJfv7QA9aOoOazW28l1GYSb2ZRR0N0paiQp+d1MhSKo10hOTWOsVK5S5Lnx9Qm6omFgXHyIvZRIvTlrQMpr1xuUrflyGOfbFOGpOLpvPE172MYdqpZKnbSS4TcuqgQKSWV2833fEubI57DPOP7ghWa2TcYeSIv4pdLYG53fTuwfbnbdc98g2LNUQzSVhSnt7BoqyNwht2aQ6b/UHg9A80+KVpuXuqmz3m1MXwHFgxjdmuesXNOrrlGpeLCcRWD+aGo0RN1NqhQRzNAig8V4GlaPTQzMsRCljKqvrIyAoP3Tt2XEGsHkkQo12rEX8Z90957XX2qKRwhruwYzqGkSLWjINoLdAxUJdpRXRc5DJTrBd3D5mdzn7kY1l7NEh4kFHJDt3Cx4Z3Mk8MYCACyCk/CEyy9DwuPi66cLz0NBcgbCM5LKjTBOwo1m+am+pvM1kSposE9FPP1+RFGb8k6jQBTJx3TRz1yKilnGXQTZ5xvdOFpJrklIT0OXP1MG3+auM9FlJA+1dX90QoNJE5z7axmK//MOGXUdkqFe2kiDkorqjxwDvc0Js9pVKfKvAmW8YqUbmI9l0ERpWCXXnLVHNmPWz3jaPY+OBAmuJWDmxB/Z8p94aEDg4BVXQ7LvsKQ3DLYhaB7yJ390CJT+i0mm+EBqY60V6YikPSWDFrYQ/NPi2b1DgE19mX8zHqw8qprIl4yh1Ckx2Iige4En/N5ktOoIxnASxAw/TzcE2skxdw5KlHDF+UTj71m16CR/dIaKlXijlfNlNzUBo/bNSadCQn3G5NoO501wPKI:XO50TgDNyo8EXAMPLE/g==:1"} }
/// </response>
/// </examples>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PollForTaskFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::poll_for_task::builders::PollForTaskInputBuilder,
}
impl PollForTaskFluentBuilder {
    /// Creates a new `PollForTask`.
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
            crate::operation::poll_for_task::PollForTask,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::poll_for_task::PollForTaskError>,
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
        crate::operation::poll_for_task::PollForTaskOutput,
        aws_smithy_http::result::SdkError<crate::operation::poll_for_task::PollForTaskError>,
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
    /// <p>The type of task the task runner is configured to accept and process. The worker group is set as a field on objects in the pipeline when they are created. You can only specify a single value for <code>workerGroup</code> in the call to <code>PollForTask</code>. There are no wildcard values permitted in <code>workerGroup</code>; the string must be an exact, case-sensitive, match.</p>
    pub fn worker_group(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.worker_group(input.into());
        self
    }
    /// <p>The type of task the task runner is configured to accept and process. The worker group is set as a field on objects in the pipeline when they are created. You can only specify a single value for <code>workerGroup</code> in the call to <code>PollForTask</code>. There are no wildcard values permitted in <code>workerGroup</code>; the string must be an exact, case-sensitive, match.</p>
    pub fn set_worker_group(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_worker_group(input);
        self
    }
    /// <p>The public DNS name of the calling task runner.</p>
    pub fn hostname(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.hostname(input.into());
        self
    }
    /// <p>The public DNS name of the calling task runner.</p>
    pub fn set_hostname(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_hostname(input);
        self
    }
    /// <p>Identity information for the EC2 instance that is hosting the task runner. You can get this value from the instance using <code>http://169.254.169.254/latest/meta-data/instance-id</code>. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/AESDG-chapter-instancedata.html">Instance Metadata</a> in the <i>Amazon Elastic Compute Cloud User Guide.</i> Passing in this value proves that your task runner is running on an EC2 instance, and ensures the proper AWS Data Pipeline service charges are applied to your pipeline.</p>
    pub fn instance_identity(mut self, input: crate::types::InstanceIdentity) -> Self {
        self.inner = self.inner.instance_identity(input);
        self
    }
    /// <p>Identity information for the EC2 instance that is hosting the task runner. You can get this value from the instance using <code>http://169.254.169.254/latest/meta-data/instance-id</code>. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/AESDG-chapter-instancedata.html">Instance Metadata</a> in the <i>Amazon Elastic Compute Cloud User Guide.</i> Passing in this value proves that your task runner is running on an EC2 instance, and ensures the proper AWS Data Pipeline service charges are applied to your pipeline.</p>
    pub fn set_instance_identity(
        mut self,
        input: std::option::Option<crate::types::InstanceIdentity>,
    ) -> Self {
        self.inner = self.inner.set_instance_identity(input);
        self
    }
}
