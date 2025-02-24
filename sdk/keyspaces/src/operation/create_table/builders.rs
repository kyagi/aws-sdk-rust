// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_table::_create_table_output::CreateTableOutputBuilder;

pub use crate::operation::create_table::_create_table_input::CreateTableInputBuilder;

/// Fluent builder constructing a request to `CreateTable`.
///
/// <p>The <code>CreateTable</code> operation adds a new table to the specified keyspace. Within a keyspace, table names must be unique.</p>
/// <p> <code>CreateTable</code> is an asynchronous operation. When the request is received, the status of the table is set to <code>CREATING</code>. You can monitor the creation status of the new table by using the <code>GetTable</code> operation, which returns the current <code>status</code> of the table. You can start using a table when the status is <code>ACTIVE</code>.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/working-with-tables.html#tables-create">Creating tables</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateTableFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_table::builders::CreateTableInputBuilder,
}
impl CreateTableFluentBuilder {
    /// Creates a new `CreateTable`.
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
            crate::operation::create_table::CreateTable,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::create_table::CreateTableError>,
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
        crate::operation::create_table::CreateTableOutput,
        aws_smithy_http::result::SdkError<crate::operation::create_table::CreateTableError>,
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
    /// <p>The name of the keyspace that the table is going to be created in.</p>
    pub fn keyspace_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.keyspace_name(input.into());
        self
    }
    /// <p>The name of the keyspace that the table is going to be created in.</p>
    pub fn set_keyspace_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_keyspace_name(input);
        self
    }
    /// <p>The name of the table.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>The <code>schemaDefinition</code> consists of the following parameters.</p>
    /// <p>For each column to be created:</p>
    /// <ul>
    /// <li> <p> <code>name</code> - The name of the column.</p> </li>
    /// <li> <p> <code>type</code> - An Amazon Keyspaces data type. For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/cql.elements.html#cql.data-types">Data types</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> </li>
    /// </ul>
    /// <p>The primary key of the table consists of the following columns:</p>
    /// <ul>
    /// <li> <p> <code>partitionKeys</code> - The partition key can be a single column, or it can be a compound value composed of two or more columns. The partition key portion of the primary key is required and determines how Amazon Keyspaces stores your data.</p> </li>
    /// <li> <p> <code>name</code> - The name of each partition key column.</p> </li>
    /// <li> <p> <code>clusteringKeys</code> - The optional clustering column portion of your primary key determines how the data is clustered and sorted within each partition.</p> </li>
    /// <li> <p> <code>name</code> - The name of the clustering column. </p> </li>
    /// <li> <p> <code>orderBy</code> - Sets the ascendant (<code>ASC</code>) or descendant (<code>DESC</code>) order modifier.</p> <p>To define a column as static use <code>staticColumns</code> - Static columns store values that are shared by all rows in the same partition:</p> </li>
    /// <li> <p> <code>name</code> - The name of the column.</p> </li>
    /// <li> <p> <code>type</code> - An Amazon Keyspaces data type.</p> </li>
    /// </ul>
    pub fn schema_definition(mut self, input: crate::types::SchemaDefinition) -> Self {
        self.inner = self.inner.schema_definition(input);
        self
    }
    /// <p>The <code>schemaDefinition</code> consists of the following parameters.</p>
    /// <p>For each column to be created:</p>
    /// <ul>
    /// <li> <p> <code>name</code> - The name of the column.</p> </li>
    /// <li> <p> <code>type</code> - An Amazon Keyspaces data type. For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/cql.elements.html#cql.data-types">Data types</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> </li>
    /// </ul>
    /// <p>The primary key of the table consists of the following columns:</p>
    /// <ul>
    /// <li> <p> <code>partitionKeys</code> - The partition key can be a single column, or it can be a compound value composed of two or more columns. The partition key portion of the primary key is required and determines how Amazon Keyspaces stores your data.</p> </li>
    /// <li> <p> <code>name</code> - The name of each partition key column.</p> </li>
    /// <li> <p> <code>clusteringKeys</code> - The optional clustering column portion of your primary key determines how the data is clustered and sorted within each partition.</p> </li>
    /// <li> <p> <code>name</code> - The name of the clustering column. </p> </li>
    /// <li> <p> <code>orderBy</code> - Sets the ascendant (<code>ASC</code>) or descendant (<code>DESC</code>) order modifier.</p> <p>To define a column as static use <code>staticColumns</code> - Static columns store values that are shared by all rows in the same partition:</p> </li>
    /// <li> <p> <code>name</code> - The name of the column.</p> </li>
    /// <li> <p> <code>type</code> - An Amazon Keyspaces data type.</p> </li>
    /// </ul>
    pub fn set_schema_definition(
        mut self,
        input: std::option::Option<crate::types::SchemaDefinition>,
    ) -> Self {
        self.inner = self.inner.set_schema_definition(input);
        self
    }
    /// <p>This parameter allows to enter a description of the table.</p>
    pub fn comment(mut self, input: crate::types::Comment) -> Self {
        self.inner = self.inner.comment(input);
        self
    }
    /// <p>This parameter allows to enter a description of the table.</p>
    pub fn set_comment(mut self, input: std::option::Option<crate::types::Comment>) -> Self {
        self.inner = self.inner.set_comment(input);
        self
    }
    /// <p>Specifies the read/write throughput capacity mode for the table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>throughputMode:PAY_PER_REQUEST</code> and </p> </li>
    /// <li> <p> <code>throughputMode:PROVISIONED</code> - Provisioned capacity mode requires <code>readCapacityUnits</code> and <code>writeCapacityUnits</code> as input.</p> </li>
    /// </ul>
    /// <p>The default is <code>throughput_mode:PAY_PER_REQUEST</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/ReadWriteCapacityMode.html">Read/write capacity modes</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn capacity_specification(mut self, input: crate::types::CapacitySpecification) -> Self {
        self.inner = self.inner.capacity_specification(input);
        self
    }
    /// <p>Specifies the read/write throughput capacity mode for the table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>throughputMode:PAY_PER_REQUEST</code> and </p> </li>
    /// <li> <p> <code>throughputMode:PROVISIONED</code> - Provisioned capacity mode requires <code>readCapacityUnits</code> and <code>writeCapacityUnits</code> as input.</p> </li>
    /// </ul>
    /// <p>The default is <code>throughput_mode:PAY_PER_REQUEST</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/ReadWriteCapacityMode.html">Read/write capacity modes</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_capacity_specification(
        mut self,
        input: std::option::Option<crate::types::CapacitySpecification>,
    ) -> Self {
        self.inner = self.inner.set_capacity_specification(input);
        self
    }
    /// <p>Specifies how the encryption key for encryption at rest is managed for the table. You can choose one of the following KMS key (KMS key):</p>
    /// <ul>
    /// <li> <p> <code>type:AWS_OWNED_KMS_KEY</code> - This key is owned by Amazon Keyspaces. </p> </li>
    /// <li> <p> <code>type:CUSTOMER_MANAGED_KMS_KEY</code> - This key is stored in your account and is created, owned, and managed by you. This option requires the <code>kms_key_identifier</code> of the KMS key in Amazon Resource Name (ARN) format as input.</p> </li>
    /// </ul>
    /// <p>The default is <code>type:AWS_OWNED_KMS_KEY</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/EncryptionAtRest.html">Encryption at rest</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn encryption_specification(
        mut self,
        input: crate::types::EncryptionSpecification,
    ) -> Self {
        self.inner = self.inner.encryption_specification(input);
        self
    }
    /// <p>Specifies how the encryption key for encryption at rest is managed for the table. You can choose one of the following KMS key (KMS key):</p>
    /// <ul>
    /// <li> <p> <code>type:AWS_OWNED_KMS_KEY</code> - This key is owned by Amazon Keyspaces. </p> </li>
    /// <li> <p> <code>type:CUSTOMER_MANAGED_KMS_KEY</code> - This key is stored in your account and is created, owned, and managed by you. This option requires the <code>kms_key_identifier</code> of the KMS key in Amazon Resource Name (ARN) format as input.</p> </li>
    /// </ul>
    /// <p>The default is <code>type:AWS_OWNED_KMS_KEY</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/EncryptionAtRest.html">Encryption at rest</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_encryption_specification(
        mut self,
        input: std::option::Option<crate::types::EncryptionSpecification>,
    ) -> Self {
        self.inner = self.inner.set_encryption_specification(input);
        self
    }
    /// <p>Specifies if <code>pointInTimeRecovery</code> is enabled or disabled for the table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>status=ENABLED</code> </p> </li>
    /// <li> <p> <code>status=DISABLED</code> </p> </li>
    /// </ul>
    /// <p>If it's not specified, the default is <code>status=DISABLED</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery.html">Point-in-time recovery</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn point_in_time_recovery(mut self, input: crate::types::PointInTimeRecovery) -> Self {
        self.inner = self.inner.point_in_time_recovery(input);
        self
    }
    /// <p>Specifies if <code>pointInTimeRecovery</code> is enabled or disabled for the table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>status=ENABLED</code> </p> </li>
    /// <li> <p> <code>status=DISABLED</code> </p> </li>
    /// </ul>
    /// <p>If it's not specified, the default is <code>status=DISABLED</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery.html">Point-in-time recovery</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_point_in_time_recovery(
        mut self,
        input: std::option::Option<crate::types::PointInTimeRecovery>,
    ) -> Self {
        self.inner = self.inner.set_point_in_time_recovery(input);
        self
    }
    /// <p>Enables Time to Live custom settings for the table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>status:enabled</code> </p> </li>
    /// <li> <p> <code>status:disabled</code> </p> </li>
    /// </ul>
    /// <p>The default is <code>status:disabled</code>. After <code>ttl</code> is enabled, you can't disable it for the table.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL.html">Expiring data by using Amazon Keyspaces Time to Live (TTL)</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn ttl(mut self, input: crate::types::TimeToLive) -> Self {
        self.inner = self.inner.ttl(input);
        self
    }
    /// <p>Enables Time to Live custom settings for the table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>status:enabled</code> </p> </li>
    /// <li> <p> <code>status:disabled</code> </p> </li>
    /// </ul>
    /// <p>The default is <code>status:disabled</code>. After <code>ttl</code> is enabled, you can't disable it for the table.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL.html">Expiring data by using Amazon Keyspaces Time to Live (TTL)</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_ttl(mut self, input: std::option::Option<crate::types::TimeToLive>) -> Self {
        self.inner = self.inner.set_ttl(input);
        self
    }
    /// <p>The default Time to Live setting in seconds for the table.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL-how-it-works.html#ttl-howitworks_default_ttl">Setting the default TTL value for a table</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn default_time_to_live(mut self, input: i32) -> Self {
        self.inner = self.inner.default_time_to_live(input);
        self
    }
    /// <p>The default Time to Live setting in seconds for the table.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL-how-it-works.html#ttl-howitworks_default_ttl">Setting the default TTL value for a table</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_default_time_to_live(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_default_time_to_live(input);
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of key-value pair tags to be attached to the resource. </p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/tagging-keyspaces.html">Adding tags and labels to Amazon Keyspaces resources</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of key-value pair tags to be attached to the resource. </p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/tagging-keyspaces.html">Adding tags and labels to Amazon Keyspaces resources</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p> Enables client-side timestamps for the table. By default, the setting is disabled. You can enable client-side timestamps with the following option:</p>
    /// <ul>
    /// <li> <p> <code>status: "enabled"</code> </p> </li>
    /// </ul>
    /// <p>Once client-side timestamps are enabled for a table, this setting cannot be disabled.</p>
    pub fn client_side_timestamps(mut self, input: crate::types::ClientSideTimestamps) -> Self {
        self.inner = self.inner.client_side_timestamps(input);
        self
    }
    /// <p> Enables client-side timestamps for the table. By default, the setting is disabled. You can enable client-side timestamps with the following option:</p>
    /// <ul>
    /// <li> <p> <code>status: "enabled"</code> </p> </li>
    /// </ul>
    /// <p>Once client-side timestamps are enabled for a table, this setting cannot be disabled.</p>
    pub fn set_client_side_timestamps(
        mut self,
        input: std::option::Option<crate::types::ClientSideTimestamps>,
    ) -> Self {
        self.inner = self.inner.set_client_side_timestamps(input);
        self
    }
}
