// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_team_member::_associate_team_member_output::AssociateTeamMemberOutputBuilder;

pub use crate::operation::associate_team_member::_associate_team_member_input::AssociateTeamMemberInputBuilder;

/// Fluent builder constructing a request to `AssociateTeamMember`.
///
/// <p>Adds an IAM user to the team for an AWS CodeStar project.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AssociateTeamMemberFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_team_member::builders::AssociateTeamMemberInputBuilder,
}
impl AssociateTeamMemberFluentBuilder {
    /// Creates a new `AssociateTeamMember`.
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
            crate::operation::associate_team_member::AssociateTeamMember,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::associate_team_member::AssociateTeamMemberError,
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
        crate::operation::associate_team_member::AssociateTeamMemberOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::associate_team_member::AssociateTeamMemberError,
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
    /// <p>The ID of the project to which you will add the IAM user.</p>
    pub fn project_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.project_id(input.into());
        self
    }
    /// <p>The ID of the project to which you will add the IAM user.</p>
    pub fn set_project_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_project_id(input);
        self
    }
    /// <p>A user- or system-generated token that identifies the entity that requested the team member association to the project. This token can be used to repeat the request.</p>
    pub fn client_request_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>A user- or system-generated token that identifies the entity that requested the team member association to the project. This token can be used to repeat the request.</p>
    pub fn set_client_request_token(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the IAM user you want to add to the AWS CodeStar project.</p>
    pub fn user_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.user_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the IAM user you want to add to the AWS CodeStar project.</p>
    pub fn set_user_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_user_arn(input);
        self
    }
    /// <p>The AWS CodeStar project role that will apply to this user. This role determines what actions a user can take in an AWS CodeStar project.</p>
    pub fn project_role(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.project_role(input.into());
        self
    }
    /// <p>The AWS CodeStar project role that will apply to this user. This role determines what actions a user can take in an AWS CodeStar project.</p>
    pub fn set_project_role(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_project_role(input);
        self
    }
    /// <p>Whether the team member is allowed to use an SSH public/private key pair to remotely access project resources, for example Amazon EC2 instances.</p>
    pub fn remote_access_allowed(mut self, input: bool) -> Self {
        self.inner = self.inner.remote_access_allowed(input);
        self
    }
    /// <p>Whether the team member is allowed to use an SSH public/private key pair to remotely access project resources, for example Amazon EC2 instances.</p>
    pub fn set_remote_access_allowed(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_remote_access_allowed(input);
        self
    }
}
