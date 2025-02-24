// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_cachedi_scsi_volume::_create_cachedi_scsi_volume_output::CreateCachediScsiVolumeOutputBuilder;

pub use crate::operation::create_cachedi_scsi_volume::_create_cachedi_scsi_volume_input::CreateCachediScsiVolumeInputBuilder;

/// Fluent builder constructing a request to `CreateCachediSCSIVolume`.
///
/// <p>Creates a cached volume on a specified cached volume gateway. This operation is only supported in the cached volume gateway type.</p> <note>
/// <p>Cache storage must be allocated to the gateway before you can create a cached volume. Use the <code>AddCache</code> operation to add cache storage to a gateway.</p>
/// </note>
/// <p>In the request, you must specify the gateway, size of the volume in bytes, the iSCSI target name, an IP address on which to expose the target, and a unique client token. In response, the gateway creates the volume and returns information about it. This information includes the volume Amazon Resource Name (ARN), its size, and the iSCSI target ARN that initiators can use to connect to the volume target.</p>
/// <p>Optionally, you can provide the ARN for an existing volume as the <code>SourceVolumeARN</code> for this cached volume, which creates an exact copy of the existing volume’s latest recovery point. The <code>VolumeSizeInBytes</code> value must be equal to or larger than the size of the copied volume, in bytes.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateCachediSCSIVolumeFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::create_cachedi_scsi_volume::builders::CreateCachediScsiVolumeInputBuilder,
}
impl CreateCachediSCSIVolumeFluentBuilder {
    /// Creates a new `CreateCachediSCSIVolume`.
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
            crate::operation::create_cachedi_scsi_volume::CreateCachediSCSIVolume,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_cachedi_scsi_volume::CreateCachediSCSIVolumeError,
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
        crate::operation::create_cachedi_scsi_volume::CreateCachediScsiVolumeOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_cachedi_scsi_volume::CreateCachediSCSIVolumeError,
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
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn gateway_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.gateway_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn set_gateway_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_gateway_arn(input);
        self
    }
    /// <p>The size of the volume in bytes.</p>
    pub fn volume_size_in_bytes(mut self, input: i64) -> Self {
        self.inner = self.inner.volume_size_in_bytes(input);
        self
    }
    /// <p>The size of the volume in bytes.</p>
    pub fn set_volume_size_in_bytes(mut self, input: std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_volume_size_in_bytes(input);
        self
    }
    /// <p>The snapshot ID (e.g. "snap-1122aabb") of the snapshot to restore as the new cached volume. Specify this field if you want to create the iSCSI storage volume from a snapshot; otherwise, do not include this field. To list snapshots for your account use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p>
    pub fn snapshot_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.snapshot_id(input.into());
        self
    }
    /// <p>The snapshot ID (e.g. "snap-1122aabb") of the snapshot to restore as the new cached volume. Specify this field if you want to create the iSCSI storage volume from a snapshot; otherwise, do not include this field. To list snapshots for your account use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p>
    pub fn set_snapshot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_id(input);
        self
    }
    /// <p>The name of the iSCSI target used by an initiator to connect to a volume and used as a suffix for the target ARN. For example, specifying <code>TargetName</code> as <i>myvolume</i> results in the target ARN of <code>arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/target/iqn.1997-05.com.amazon:myvolume</code>. The target name must be unique across all volumes on a gateway.</p>
    /// <p>If you don't specify a value, Storage Gateway uses the value that was previously used for this volume as the new target name.</p>
    pub fn target_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.target_name(input.into());
        self
    }
    /// <p>The name of the iSCSI target used by an initiator to connect to a volume and used as a suffix for the target ARN. For example, specifying <code>TargetName</code> as <i>myvolume</i> results in the target ARN of <code>arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/target/iqn.1997-05.com.amazon:myvolume</code>. The target name must be unique across all volumes on a gateway.</p>
    /// <p>If you don't specify a value, Storage Gateway uses the value that was previously used for this volume as the new target name.</p>
    pub fn set_target_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_target_name(input);
        self
    }
    /// <p>The ARN for an existing volume. Specifying this ARN makes the new volume into an exact copy of the specified existing volume's latest recovery point. The <code>VolumeSizeInBytes</code> value for this new volume must be equal to or larger than the size of the existing volume, in bytes.</p>
    pub fn source_volume_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.source_volume_arn(input.into());
        self
    }
    /// <p>The ARN for an existing volume. Specifying this ARN makes the new volume into an exact copy of the specified existing volume's latest recovery point. The <code>VolumeSizeInBytes</code> value for this new volume must be equal to or larger than the size of the existing volume, in bytes.</p>
    pub fn set_source_volume_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_source_volume_arn(input);
        self
    }
    /// <p>The network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted. Use <code>DescribeGatewayInformation</code> to get a list of the network interfaces available on a gateway.</p>
    /// <p>Valid Values: A valid IP address.</p>
    pub fn network_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.network_interface_id(input.into());
        self
    }
    /// <p>The network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted. Use <code>DescribeGatewayInformation</code> to get a list of the network interfaces available on a gateway.</p>
    /// <p>Valid Values: A valid IP address.</p>
    pub fn set_network_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_network_interface_id(input);
        self
    }
    /// <p>A unique identifier that you use to retry a request. If you retry a request, use the same <code>ClientToken</code> you specified in the initial request.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique identifier that you use to retry a request. If you retry a request, use the same <code>ClientToken</code> you specified in the initial request.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn kms_encrypted(mut self, input: bool) -> Self {
        self.inner = self.inner.kms_encrypted(input);
        self
    }
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn set_kms_encrypted(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_kms_encrypted(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    pub fn kms_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.kms_key(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    pub fn set_kms_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of up to 50 tags that you can assign to a cached volume. Each tag is a key-value pair.</p> <note>
    /// <p>Valid characters for key and value are letters, spaces, and numbers that you can represent in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag's key is 128 characters, and the maximum length for a tag's value is 256 characters.</p>
    /// </note>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of up to 50 tags that you can assign to a cached volume. Each tag is a key-value pair.</p> <note>
    /// <p>Valid characters for key and value are letters, spaces, and numbers that you can represent in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag's key is 128 characters, and the maximum length for a tag's value is 256 characters.</p>
    /// </note>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
