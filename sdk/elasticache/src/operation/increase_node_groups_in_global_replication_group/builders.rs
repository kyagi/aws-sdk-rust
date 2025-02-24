// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::increase_node_groups_in_global_replication_group::_increase_node_groups_in_global_replication_group_output::IncreaseNodeGroupsInGlobalReplicationGroupOutputBuilder;

pub use crate::operation::increase_node_groups_in_global_replication_group::_increase_node_groups_in_global_replication_group_input::IncreaseNodeGroupsInGlobalReplicationGroupInputBuilder;

/// Fluent builder constructing a request to `IncreaseNodeGroupsInGlobalReplicationGroup`.
///
/// <p>Increase the number of node groups in the Global datastore</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct IncreaseNodeGroupsInGlobalReplicationGroupFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::increase_node_groups_in_global_replication_group::builders::IncreaseNodeGroupsInGlobalReplicationGroupInputBuilder
            }
impl IncreaseNodeGroupsInGlobalReplicationGroupFluentBuilder {
    /// Creates a new `IncreaseNodeGroupsInGlobalReplicationGroup`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroup, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroupError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroupOutput, aws_smithy_http::result::SdkError<crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroupError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The name of the Global datastore</p>
    pub fn global_replication_group_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.global_replication_group_id(input.into());
        self
    }
    /// <p>The name of the Global datastore</p>
    pub fn set_global_replication_group_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_global_replication_group_id(input);
        self
    }
    /// <p>The number of node groups you wish to add</p>
    pub fn node_group_count(mut self, input: i32) -> Self {
        self.inner = self.inner.node_group_count(input);
        self
    }
    /// <p>The number of node groups you wish to add</p>
    pub fn set_node_group_count(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_node_group_count(input);
        self
    }
    /// Appends an item to `RegionalConfigurations`.
    ///
    /// To override the contents of this collection use [`set_regional_configurations`](Self::set_regional_configurations).
    ///
    /// <p>Describes the replication group IDs, the Amazon regions where they are stored and the shard configuration for each that comprise the Global datastore</p>
    pub fn regional_configurations(mut self, input: crate::types::RegionalConfiguration) -> Self {
        self.inner = self.inner.regional_configurations(input);
        self
    }
    /// <p>Describes the replication group IDs, the Amazon regions where they are stored and the shard configuration for each that comprise the Global datastore</p>
    pub fn set_regional_configurations(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::RegionalConfiguration>>,
    ) -> Self {
        self.inner = self.inner.set_regional_configurations(input);
        self
    }
    /// <p>Indicates that the process begins immediately. At present, the only permitted value for this parameter is true.</p>
    pub fn apply_immediately(mut self, input: bool) -> Self {
        self.inner = self.inner.apply_immediately(input);
        self
    }
    /// <p>Indicates that the process begins immediately. At present, the only permitted value for this parameter is true.</p>
    pub fn set_apply_immediately(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_apply_immediately(input);
        self
    }
}
