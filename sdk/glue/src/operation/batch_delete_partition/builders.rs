// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_delete_partition::_batch_delete_partition_output::BatchDeletePartitionOutputBuilder;

pub use crate::operation::batch_delete_partition::_batch_delete_partition_input::BatchDeletePartitionInputBuilder;

/// Fluent builder constructing a request to `BatchDeletePartition`.
///
/// <p>Deletes one or more partitions in a batch operation.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct BatchDeletePartitionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_delete_partition::builders::BatchDeletePartitionInputBuilder,
}
impl BatchDeletePartitionFluentBuilder {
    /// Creates a new `BatchDeletePartition`.
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
            crate::operation::batch_delete_partition::BatchDeletePartition,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_delete_partition::BatchDeletePartitionError,
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
        crate::operation::batch_delete_partition::BatchDeletePartitionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_delete_partition::BatchDeletePartitionError,
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
    /// <p>The ID of the Data Catalog where the partition to be deleted resides. If none is provided, the Amazon Web Services account ID is used by default.</p>
    pub fn catalog_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.catalog_id(input.into());
        self
    }
    /// <p>The ID of the Data Catalog where the partition to be deleted resides. If none is provided, the Amazon Web Services account ID is used by default.</p>
    pub fn set_catalog_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_catalog_id(input);
        self
    }
    /// <p>The name of the catalog database in which the table in question resides.</p>
    pub fn database_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.database_name(input.into());
        self
    }
    /// <p>The name of the catalog database in which the table in question resides.</p>
    pub fn set_database_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_database_name(input);
        self
    }
    /// <p>The name of the table that contains the partitions to be deleted.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table that contains the partitions to be deleted.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// Appends an item to `PartitionsToDelete`.
    ///
    /// To override the contents of this collection use [`set_partitions_to_delete`](Self::set_partitions_to_delete).
    ///
    /// <p>A list of <code>PartitionInput</code> structures that define the partitions to be deleted.</p>
    pub fn partitions_to_delete(mut self, input: crate::types::PartitionValueList) -> Self {
        self.inner = self.inner.partitions_to_delete(input);
        self
    }
    /// <p>A list of <code>PartitionInput</code> structures that define the partitions to be deleted.</p>
    pub fn set_partitions_to_delete(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PartitionValueList>>,
    ) -> Self {
        self.inner = self.inner.set_partitions_to_delete(input);
        self
    }
}
