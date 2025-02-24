// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_user_pool::_update_user_pool_output::UpdateUserPoolOutputBuilder;

pub use crate::operation::update_user_pool::_update_user_pool_input::UpdateUserPoolInputBuilder;

/// Fluent builder constructing a request to `UpdateUserPool`.
///
/// <p>Updates the specified user pool with the specified attributes. You can get a list of the current user pool settings using <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_DescribeUserPool.html">DescribeUserPool</a>. If you don't provide a value for an attribute, it will be set to the default value. </p> <note>
/// <p>This action might generate an SMS text message. Starting June 1, 2021, US telecom carriers require you to register an origination phone number before you can send SMS messages to US phone numbers. If you use SMS text messages in Amazon Cognito, you must register a phone number with <a href="https://console.aws.amazon.com/pinpoint/home/">Amazon Pinpoint</a>. Amazon Cognito uses the registered number automatically. Otherwise, Amazon Cognito users who must receive SMS messages might not be able to sign up, activate their accounts, or sign in.</p>
/// <p>If you have never used SMS text messages with Amazon Cognito or any other Amazon Web Service, Amazon Simple Notification Service might place your account in the SMS sandbox. In <i> <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html">sandbox mode</a> </i>, you can send messages only to verified phone numbers. After you test your app while in the sandbox environment, you can move out of the sandbox and into production. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-sms-userpool-settings.html"> SMS message settings for Amazon Cognito user pools</a> in the <i>Amazon Cognito Developer Guide</i>.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateUserPoolFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_user_pool::builders::UpdateUserPoolInputBuilder,
}
impl UpdateUserPoolFluentBuilder {
    /// Creates a new `UpdateUserPool`.
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
            crate::operation::update_user_pool::UpdateUserPool,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_user_pool::UpdateUserPoolError>,
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
        crate::operation::update_user_pool::UpdateUserPoolOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_user_pool::UpdateUserPoolError>,
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
    /// <p>The user pool ID for the user pool you want to update.</p>
    pub fn user_pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The user pool ID for the user pool you want to update.</p>
    pub fn set_user_pool_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>A container with the policies you want to update in a user pool.</p>
    pub fn policies(mut self, input: crate::types::UserPoolPolicyType) -> Self {
        self.inner = self.inner.policies(input);
        self
    }
    /// <p>A container with the policies you want to update in a user pool.</p>
    pub fn set_policies(
        mut self,
        input: std::option::Option<crate::types::UserPoolPolicyType>,
    ) -> Self {
        self.inner = self.inner.set_policies(input);
        self
    }
    /// <p>When active, <code>DeletionProtection</code> prevents accidental deletion of your user pool. Before you can delete a user pool that you have protected against deletion, you must deactivate this feature.</p>
    /// <p>When you try to delete a protected user pool in a <code>DeleteUserPool</code> API request, Amazon Cognito returns an <code>InvalidParameterException</code> error. To delete a protected user pool, send a new <code>DeleteUserPool</code> request after you deactivate deletion protection in an <code>UpdateUserPool</code> API request.</p>
    pub fn deletion_protection(mut self, input: crate::types::DeletionProtectionType) -> Self {
        self.inner = self.inner.deletion_protection(input);
        self
    }
    /// <p>When active, <code>DeletionProtection</code> prevents accidental deletion of your user pool. Before you can delete a user pool that you have protected against deletion, you must deactivate this feature.</p>
    /// <p>When you try to delete a protected user pool in a <code>DeleteUserPool</code> API request, Amazon Cognito returns an <code>InvalidParameterException</code> error. To delete a protected user pool, send a new <code>DeleteUserPool</code> request after you deactivate deletion protection in an <code>UpdateUserPool</code> API request.</p>
    pub fn set_deletion_protection(
        mut self,
        input: std::option::Option<crate::types::DeletionProtectionType>,
    ) -> Self {
        self.inner = self.inner.set_deletion_protection(input);
        self
    }
    /// <p>The Lambda configuration information from the request to update the user pool.</p>
    pub fn lambda_config(mut self, input: crate::types::LambdaConfigType) -> Self {
        self.inner = self.inner.lambda_config(input);
        self
    }
    /// <p>The Lambda configuration information from the request to update the user pool.</p>
    pub fn set_lambda_config(
        mut self,
        input: std::option::Option<crate::types::LambdaConfigType>,
    ) -> Self {
        self.inner = self.inner.set_lambda_config(input);
        self
    }
    /// Appends an item to `AutoVerifiedAttributes`.
    ///
    /// To override the contents of this collection use [`set_auto_verified_attributes`](Self::set_auto_verified_attributes).
    ///
    /// <p>The attributes that are automatically verified when Amazon Cognito requests to update user pools.</p>
    pub fn auto_verified_attributes(mut self, input: crate::types::VerifiedAttributeType) -> Self {
        self.inner = self.inner.auto_verified_attributes(input);
        self
    }
    /// <p>The attributes that are automatically verified when Amazon Cognito requests to update user pools.</p>
    pub fn set_auto_verified_attributes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::VerifiedAttributeType>>,
    ) -> Self {
        self.inner = self.inner.set_auto_verified_attributes(input);
        self
    }
    /// <p>This parameter is no longer used. See <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_VerificationMessageTemplateType.html">VerificationMessageTemplateType</a>.</p>
    pub fn sms_verification_message(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.sms_verification_message(input.into());
        self
    }
    /// <p>This parameter is no longer used. See <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_VerificationMessageTemplateType.html">VerificationMessageTemplateType</a>.</p>
    pub fn set_sms_verification_message(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_sms_verification_message(input);
        self
    }
    /// <p>This parameter is no longer used. See <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_VerificationMessageTemplateType.html">VerificationMessageTemplateType</a>.</p>
    pub fn email_verification_message(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.email_verification_message(input.into());
        self
    }
    /// <p>This parameter is no longer used. See <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_VerificationMessageTemplateType.html">VerificationMessageTemplateType</a>.</p>
    pub fn set_email_verification_message(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_email_verification_message(input);
        self
    }
    /// <p>This parameter is no longer used. See <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_VerificationMessageTemplateType.html">VerificationMessageTemplateType</a>.</p>
    pub fn email_verification_subject(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.email_verification_subject(input.into());
        self
    }
    /// <p>This parameter is no longer used. See <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_VerificationMessageTemplateType.html">VerificationMessageTemplateType</a>.</p>
    pub fn set_email_verification_subject(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_email_verification_subject(input);
        self
    }
    /// <p>The template for verification messages.</p>
    pub fn verification_message_template(
        mut self,
        input: crate::types::VerificationMessageTemplateType,
    ) -> Self {
        self.inner = self.inner.verification_message_template(input);
        self
    }
    /// <p>The template for verification messages.</p>
    pub fn set_verification_message_template(
        mut self,
        input: std::option::Option<crate::types::VerificationMessageTemplateType>,
    ) -> Self {
        self.inner = self.inner.set_verification_message_template(input);
        self
    }
    /// <p>The contents of the SMS authentication message.</p>
    pub fn sms_authentication_message(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.sms_authentication_message(input.into());
        self
    }
    /// <p>The contents of the SMS authentication message.</p>
    pub fn set_sms_authentication_message(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_sms_authentication_message(input);
        self
    }
    /// <p>The settings for updates to user attributes. These settings include the property <code>AttributesRequireVerificationBeforeUpdate</code>, a user-pool setting that tells Amazon Cognito how to handle changes to the value of your users' email address and phone number attributes. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-email-phone-verification.html#user-pool-settings-verifications-verify-attribute-updates"> Verifying updates to email addresses and phone numbers</a>.</p>
    pub fn user_attribute_update_settings(
        mut self,
        input: crate::types::UserAttributeUpdateSettingsType,
    ) -> Self {
        self.inner = self.inner.user_attribute_update_settings(input);
        self
    }
    /// <p>The settings for updates to user attributes. These settings include the property <code>AttributesRequireVerificationBeforeUpdate</code>, a user-pool setting that tells Amazon Cognito how to handle changes to the value of your users' email address and phone number attributes. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-email-phone-verification.html#user-pool-settings-verifications-verify-attribute-updates"> Verifying updates to email addresses and phone numbers</a>.</p>
    pub fn set_user_attribute_update_settings(
        mut self,
        input: std::option::Option<crate::types::UserAttributeUpdateSettingsType>,
    ) -> Self {
        self.inner = self.inner.set_user_attribute_update_settings(input);
        self
    }
    /// <p>Possible values include:</p>
    /// <ul>
    /// <li> <p> <code>OFF</code> - MFA tokens aren't required and can't be specified during user registration.</p> </li>
    /// <li> <p> <code>ON</code> - MFA tokens are required for all user registrations. You can only specify ON when you're initially creating a user pool. You can use the <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_SetUserPoolMfaConfig.html">SetUserPoolMfaConfig</a> API operation to turn MFA "ON" for existing user pools. </p> </li>
    /// <li> <p> <code>OPTIONAL</code> - Users have the option when registering to create an MFA token.</p> </li>
    /// </ul>
    pub fn mfa_configuration(mut self, input: crate::types::UserPoolMfaType) -> Self {
        self.inner = self.inner.mfa_configuration(input);
        self
    }
    /// <p>Possible values include:</p>
    /// <ul>
    /// <li> <p> <code>OFF</code> - MFA tokens aren't required and can't be specified during user registration.</p> </li>
    /// <li> <p> <code>ON</code> - MFA tokens are required for all user registrations. You can only specify ON when you're initially creating a user pool. You can use the <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_SetUserPoolMfaConfig.html">SetUserPoolMfaConfig</a> API operation to turn MFA "ON" for existing user pools. </p> </li>
    /// <li> <p> <code>OPTIONAL</code> - Users have the option when registering to create an MFA token.</p> </li>
    /// </ul>
    pub fn set_mfa_configuration(
        mut self,
        input: std::option::Option<crate::types::UserPoolMfaType>,
    ) -> Self {
        self.inner = self.inner.set_mfa_configuration(input);
        self
    }
    /// <p>The device-remembering configuration for a user pool. A null value indicates that you have deactivated device remembering in your user pool.</p> <note>
    /// <p>When you provide a value for any <code>DeviceConfiguration</code> field, you activate the Amazon Cognito device-remembering feature.</p>
    /// </note>
    pub fn device_configuration(mut self, input: crate::types::DeviceConfigurationType) -> Self {
        self.inner = self.inner.device_configuration(input);
        self
    }
    /// <p>The device-remembering configuration for a user pool. A null value indicates that you have deactivated device remembering in your user pool.</p> <note>
    /// <p>When you provide a value for any <code>DeviceConfiguration</code> field, you activate the Amazon Cognito device-remembering feature.</p>
    /// </note>
    pub fn set_device_configuration(
        mut self,
        input: std::option::Option<crate::types::DeviceConfigurationType>,
    ) -> Self {
        self.inner = self.inner.set_device_configuration(input);
        self
    }
    /// <p>The email configuration of your user pool. The email configuration type sets your preferred sending method, Amazon Web Services Region, and sender for email invitation and verification messages from your user pool.</p>
    pub fn email_configuration(mut self, input: crate::types::EmailConfigurationType) -> Self {
        self.inner = self.inner.email_configuration(input);
        self
    }
    /// <p>The email configuration of your user pool. The email configuration type sets your preferred sending method, Amazon Web Services Region, and sender for email invitation and verification messages from your user pool.</p>
    pub fn set_email_configuration(
        mut self,
        input: std::option::Option<crate::types::EmailConfigurationType>,
    ) -> Self {
        self.inner = self.inner.set_email_configuration(input);
        self
    }
    /// <p>The SMS configuration with the settings that your Amazon Cognito user pool must use to send an SMS message from your Amazon Web Services account through Amazon Simple Notification Service. To send SMS messages with Amazon SNS in the Amazon Web Services Region that you want, the Amazon Cognito user pool uses an Identity and Access Management (IAM) role in your Amazon Web Services account.</p>
    pub fn sms_configuration(mut self, input: crate::types::SmsConfigurationType) -> Self {
        self.inner = self.inner.sms_configuration(input);
        self
    }
    /// <p>The SMS configuration with the settings that your Amazon Cognito user pool must use to send an SMS message from your Amazon Web Services account through Amazon Simple Notification Service. To send SMS messages with Amazon SNS in the Amazon Web Services Region that you want, the Amazon Cognito user pool uses an Identity and Access Management (IAM) role in your Amazon Web Services account.</p>
    pub fn set_sms_configuration(
        mut self,
        input: std::option::Option<crate::types::SmsConfigurationType>,
    ) -> Self {
        self.inner = self.inner.set_sms_configuration(input);
        self
    }
    /// Adds a key-value pair to `UserPoolTags`.
    ///
    /// To override the contents of this collection use [`set_user_pool_tags`](Self::set_user_pool_tags).
    ///
    /// <p>The tag keys and values to assign to the user pool. A tag is a label that you can use to categorize and manage user pools in different ways, such as by purpose, owner, environment, or other criteria.</p>
    pub fn user_pool_tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.user_pool_tags(k.into(), v.into());
        self
    }
    /// <p>The tag keys and values to assign to the user pool. A tag is a label that you can use to categorize and manage user pools in different ways, such as by purpose, owner, environment, or other criteria.</p>
    pub fn set_user_pool_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_user_pool_tags(input);
        self
    }
    /// <p>The configuration for <code>AdminCreateUser</code> requests.</p>
    pub fn admin_create_user_config(
        mut self,
        input: crate::types::AdminCreateUserConfigType,
    ) -> Self {
        self.inner = self.inner.admin_create_user_config(input);
        self
    }
    /// <p>The configuration for <code>AdminCreateUser</code> requests.</p>
    pub fn set_admin_create_user_config(
        mut self,
        input: std::option::Option<crate::types::AdminCreateUserConfigType>,
    ) -> Self {
        self.inner = self.inner.set_admin_create_user_config(input);
        self
    }
    /// <p>Enables advanced security risk detection. Set the key <code>AdvancedSecurityMode</code> to the value "AUDIT".</p>
    pub fn user_pool_add_ons(mut self, input: crate::types::UserPoolAddOnsType) -> Self {
        self.inner = self.inner.user_pool_add_ons(input);
        self
    }
    /// <p>Enables advanced security risk detection. Set the key <code>AdvancedSecurityMode</code> to the value "AUDIT".</p>
    pub fn set_user_pool_add_ons(
        mut self,
        input: std::option::Option<crate::types::UserPoolAddOnsType>,
    ) -> Self {
        self.inner = self.inner.set_user_pool_add_ons(input);
        self
    }
    /// <p>The available verified method a user can use to recover their password when they call <code>ForgotPassword</code>. You can use this setting to define a preferred method when a user has more than one method available. With this setting, SMS doesn't qualify for a valid password recovery mechanism if the user also has SMS multi-factor authentication (MFA) activated. In the absence of this setting, Amazon Cognito uses the legacy behavior to determine the recovery method where SMS is preferred through email.</p>
    pub fn account_recovery_setting(
        mut self,
        input: crate::types::AccountRecoverySettingType,
    ) -> Self {
        self.inner = self.inner.account_recovery_setting(input);
        self
    }
    /// <p>The available verified method a user can use to recover their password when they call <code>ForgotPassword</code>. You can use this setting to define a preferred method when a user has more than one method available. With this setting, SMS doesn't qualify for a valid password recovery mechanism if the user also has SMS multi-factor authentication (MFA) activated. In the absence of this setting, Amazon Cognito uses the legacy behavior to determine the recovery method where SMS is preferred through email.</p>
    pub fn set_account_recovery_setting(
        mut self,
        input: std::option::Option<crate::types::AccountRecoverySettingType>,
    ) -> Self {
        self.inner = self.inner.set_account_recovery_setting(input);
        self
    }
}
