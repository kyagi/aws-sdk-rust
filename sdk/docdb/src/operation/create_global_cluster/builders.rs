// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_global_cluster::_create_global_cluster_output::CreateGlobalClusterOutputBuilder;

pub use crate::operation::create_global_cluster::_create_global_cluster_input::CreateGlobalClusterInputBuilder;

/// Fluent builder constructing a request to `CreateGlobalCluster`.
///
/// <p>Creates an Amazon DocumentDB global cluster that can span multiple multiple Amazon Web Services Regions. The global cluster contains one primary cluster with read-write capability, and up-to give read-only secondary clusters. Global clusters uses storage-based fast replication across regions with latencies less than one second, using dedicated infrastructure with no impact to your workload’s performance.</p>
/// <p></p>
/// <p>You can create a global cluster that is initially empty, and then add a primary and a secondary to it. Or you can specify an existing cluster during the create operation, and this cluster becomes the primary of the global cluster. </p> <note>
/// <p>This action only applies to Amazon DocumentDB clusters.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateGlobalClusterFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_global_cluster::builders::CreateGlobalClusterInputBuilder,
}
impl CreateGlobalClusterFluentBuilder {
    /// Creates a new `CreateGlobalCluster`.
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
            crate::operation::create_global_cluster::CreateGlobalCluster,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_global_cluster::CreateGlobalClusterError,
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
        crate::operation::create_global_cluster::CreateGlobalClusterOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_global_cluster::CreateGlobalClusterError,
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
    /// <p>The cluster identifier of the new global cluster.</p>
    pub fn global_cluster_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.global_cluster_identifier(input.into());
        self
    }
    /// <p>The cluster identifier of the new global cluster.</p>
    pub fn set_global_cluster_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_global_cluster_identifier(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) to use as the primary cluster of the global cluster. This parameter is optional.</p>
    pub fn source_db_cluster_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.source_db_cluster_identifier(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) to use as the primary cluster of the global cluster. This parameter is optional.</p>
    pub fn set_source_db_cluster_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_source_db_cluster_identifier(input);
        self
    }
    /// <p>The name of the database engine to be used for this cluster.</p>
    pub fn engine(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.engine(input.into());
        self
    }
    /// <p>The name of the database engine to be used for this cluster.</p>
    pub fn set_engine(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_engine(input);
        self
    }
    /// <p>The engine version of the global cluster.</p>
    pub fn engine_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.engine_version(input.into());
        self
    }
    /// <p>The engine version of the global cluster.</p>
    pub fn set_engine_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_engine_version(input);
        self
    }
    /// <p>The deletion protection setting for the new global cluster. The global cluster can't be deleted when deletion protection is enabled. </p>
    pub fn deletion_protection(mut self, input: bool) -> Self {
        self.inner = self.inner.deletion_protection(input);
        self
    }
    /// <p>The deletion protection setting for the new global cluster. The global cluster can't be deleted when deletion protection is enabled. </p>
    pub fn set_deletion_protection(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_deletion_protection(input);
        self
    }
    /// <p>The name for your database of up to 64 alpha-numeric characters. If you do not provide a name, Amazon DocumentDB will not create a database in the global cluster you are creating.</p>
    pub fn database_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.database_name(input.into());
        self
    }
    /// <p>The name for your database of up to 64 alpha-numeric characters. If you do not provide a name, Amazon DocumentDB will not create a database in the global cluster you are creating.</p>
    pub fn set_database_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_database_name(input);
        self
    }
    /// <p>The storage encryption setting for the new global cluster. </p>
    pub fn storage_encrypted(mut self, input: bool) -> Self {
        self.inner = self.inner.storage_encrypted(input);
        self
    }
    /// <p>The storage encryption setting for the new global cluster. </p>
    pub fn set_storage_encrypted(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_storage_encrypted(input);
        self
    }
}
