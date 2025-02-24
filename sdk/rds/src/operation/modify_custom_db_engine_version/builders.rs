// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_custom_db_engine_version::_modify_custom_db_engine_version_output::ModifyCustomDbEngineVersionOutputBuilder;

pub use crate::operation::modify_custom_db_engine_version::_modify_custom_db_engine_version_input::ModifyCustomDbEngineVersionInputBuilder;

/// Fluent builder constructing a request to `ModifyCustomDBEngineVersion`.
///
/// <p>Modifies the status of a custom engine version (CEV). You can find CEVs to modify by calling <code>DescribeDBEngineVersions</code>.</p> <note>
/// <p>The MediaImport service that imports files from Amazon S3 to create CEVs isn't integrated with Amazon Web Services CloudTrail. If you turn on data logging for Amazon RDS in CloudTrail, calls to the <code>ModifyCustomDbEngineVersion</code> event aren't logged. However, you might see calls from the API gateway that accesses your Amazon S3 bucket. These calls originate from the MediaImport service for the <code>ModifyCustomDbEngineVersion</code> event.</p>
/// </note>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/custom-cev.html#custom-cev.modify">Modifying CEV status</a> in the <i>Amazon RDS User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ModifyCustomDBEngineVersionFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::modify_custom_db_engine_version::builders::ModifyCustomDbEngineVersionInputBuilder
            }
impl ModifyCustomDBEngineVersionFluentBuilder {
    /// Creates a new `ModifyCustomDBEngineVersion`.
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
            crate::operation::modify_custom_db_engine_version::ModifyCustomDBEngineVersion,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_custom_db_engine_version::ModifyCustomDBEngineVersionError,
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
        crate::operation::modify_custom_db_engine_version::ModifyCustomDbEngineVersionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_custom_db_engine_version::ModifyCustomDBEngineVersionError,
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
    /// <p>The DB engine. The only supported values are <code>custom-oracle-ee</code> and <code>custom-oracle-ee-cdb</code>.</p>
    pub fn engine(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.engine(input.into());
        self
    }
    /// <p>The DB engine. The only supported values are <code>custom-oracle-ee</code> and <code>custom-oracle-ee-cdb</code>.</p>
    pub fn set_engine(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_engine(input);
        self
    }
    /// <p>The custom engine version (CEV) that you want to modify. This option is required for RDS Custom for Oracle, but optional for Amazon RDS. The combination of <code>Engine</code> and <code>EngineVersion</code> is unique per customer per Amazon Web Services Region.</p>
    pub fn engine_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.engine_version(input.into());
        self
    }
    /// <p>The custom engine version (CEV) that you want to modify. This option is required for RDS Custom for Oracle, but optional for Amazon RDS. The combination of <code>Engine</code> and <code>EngineVersion</code> is unique per customer per Amazon Web Services Region.</p>
    pub fn set_engine_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_engine_version(input);
        self
    }
    /// <p>An optional description of your CEV.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>An optional description of your CEV.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The availability status to be assigned to the CEV. Valid values are as follows:</p>
    /// <dl>
    /// <dt>
    /// available
    /// </dt>
    /// <dd>
    /// <p>You can use this CEV to create a new RDS Custom DB instance.</p>
    /// </dd>
    /// <dt>
    /// inactive
    /// </dt>
    /// <dd>
    /// <p>You can create a new RDS Custom instance by restoring a DB snapshot with this CEV. You can't patch or create new instances with this CEV.</p>
    /// </dd>
    /// </dl>
    /// <p>You can change any status to any status. A typical reason to change status is to prevent the accidental use of a CEV, or to make a deprecated CEV eligible for use again. For example, you might change the status of your CEV from <code>available</code> to <code>inactive</code>, and from <code>inactive</code> back to <code>available</code>. To change the availability status of the CEV, it must not currently be in use by an RDS Custom instance, snapshot, or automated backup.</p>
    pub fn status(mut self, input: crate::types::CustomEngineVersionStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>The availability status to be assigned to the CEV. Valid values are as follows:</p>
    /// <dl>
    /// <dt>
    /// available
    /// </dt>
    /// <dd>
    /// <p>You can use this CEV to create a new RDS Custom DB instance.</p>
    /// </dd>
    /// <dt>
    /// inactive
    /// </dt>
    /// <dd>
    /// <p>You can create a new RDS Custom instance by restoring a DB snapshot with this CEV. You can't patch or create new instances with this CEV.</p>
    /// </dd>
    /// </dl>
    /// <p>You can change any status to any status. A typical reason to change status is to prevent the accidental use of a CEV, or to make a deprecated CEV eligible for use again. For example, you might change the status of your CEV from <code>available</code> to <code>inactive</code>, and from <code>inactive</code> back to <code>available</code>. To change the availability status of the CEV, it must not currently be in use by an RDS Custom instance, snapshot, or automated backup.</p>
    pub fn set_status(
        mut self,
        input: std::option::Option<crate::types::CustomEngineVersionStatus>,
    ) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
}
