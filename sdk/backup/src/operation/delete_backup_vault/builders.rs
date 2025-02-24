// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_backup_vault::_delete_backup_vault_output::DeleteBackupVaultOutputBuilder;

pub use crate::operation::delete_backup_vault::_delete_backup_vault_input::DeleteBackupVaultInputBuilder;

/// Fluent builder constructing a request to `DeleteBackupVault`.
///
/// <p>Deletes the backup vault identified by its name. A vault can be deleted only if it is empty.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteBackupVaultFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_backup_vault::builders::DeleteBackupVaultInputBuilder,
}
impl DeleteBackupVaultFluentBuilder {
    /// Creates a new `DeleteBackupVault`.
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
            crate::operation::delete_backup_vault::DeleteBackupVault,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_backup_vault::DeleteBackupVaultError,
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
        crate::operation::delete_backup_vault::DeleteBackupVaultOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_backup_vault::DeleteBackupVaultError,
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
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    pub fn backup_vault_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.backup_vault_name(input.into());
        self
    }
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    pub fn set_backup_vault_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_backup_vault_name(input);
        self
    }
}
