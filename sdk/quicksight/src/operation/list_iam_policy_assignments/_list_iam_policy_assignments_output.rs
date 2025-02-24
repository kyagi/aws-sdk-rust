// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListIamPolicyAssignmentsOutput {
    /// <p>Information describing the IAM policy assignments.</p>
    #[doc(hidden)]
    pub iam_policy_assignments:
        std::option::Option<std::vec::Vec<crate::types::IamPolicyAssignmentSummary>>,
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The Amazon Web Services request ID for this operation.</p>
    #[doc(hidden)]
    pub request_id: std::option::Option<std::string::String>,
    /// <p>The HTTP status of the request.</p>
    #[doc(hidden)]
    pub status: i32,
    _request_id: Option<String>,
}
impl ListIamPolicyAssignmentsOutput {
    /// <p>Information describing the IAM policy assignments.</p>
    pub fn iam_policy_assignments(
        &self,
    ) -> std::option::Option<&[crate::types::IamPolicyAssignmentSummary]> {
        self.iam_policy_assignments.as_deref()
    }
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The Amazon Web Services request ID for this operation.</p>
    pub fn request_id(&self) -> std::option::Option<&str> {
        self.request_id.as_deref()
    }
    /// <p>The HTTP status of the request.</p>
    pub fn status(&self) -> i32 {
        self.status
    }
}
impl aws_http::request_id::RequestId for ListIamPolicyAssignmentsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListIamPolicyAssignmentsOutput {
    /// Creates a new builder-style object to manufacture [`ListIamPolicyAssignmentsOutput`](crate::operation::list_iam_policy_assignments::ListIamPolicyAssignmentsOutput).
    pub fn builder() -> crate::operation::list_iam_policy_assignments::builders::ListIamPolicyAssignmentsOutputBuilder{
        crate::operation::list_iam_policy_assignments::builders::ListIamPolicyAssignmentsOutputBuilder::default()
    }
}

/// A builder for [`ListIamPolicyAssignmentsOutput`](crate::operation::list_iam_policy_assignments::ListIamPolicyAssignmentsOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ListIamPolicyAssignmentsOutputBuilder {
    pub(crate) iam_policy_assignments:
        std::option::Option<std::vec::Vec<crate::types::IamPolicyAssignmentSummary>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) request_id: std::option::Option<std::string::String>,
    pub(crate) status: std::option::Option<i32>,
    _request_id: Option<String>,
}
impl ListIamPolicyAssignmentsOutputBuilder {
    /// Appends an item to `iam_policy_assignments`.
    ///
    /// To override the contents of this collection use [`set_iam_policy_assignments`](Self::set_iam_policy_assignments).
    ///
    /// <p>Information describing the IAM policy assignments.</p>
    pub fn iam_policy_assignments(
        mut self,
        input: crate::types::IamPolicyAssignmentSummary,
    ) -> Self {
        let mut v = self.iam_policy_assignments.unwrap_or_default();
        v.push(input);
        self.iam_policy_assignments = Some(v);
        self
    }
    /// <p>Information describing the IAM policy assignments.</p>
    pub fn set_iam_policy_assignments(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::IamPolicyAssignmentSummary>>,
    ) -> Self {
        self.iam_policy_assignments = input;
        self
    }
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The Amazon Web Services request ID for this operation.</p>
    pub fn request_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.request_id = Some(input.into());
        self
    }
    /// <p>The Amazon Web Services request ID for this operation.</p>
    pub fn set_request_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.request_id = input;
        self
    }
    /// <p>The HTTP status of the request.</p>
    pub fn status(mut self, input: i32) -> Self {
        self.status = Some(input);
        self
    }
    /// <p>The HTTP status of the request.</p>
    pub fn set_status(mut self, input: std::option::Option<i32>) -> Self {
        self.status = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListIamPolicyAssignmentsOutput`](crate::operation::list_iam_policy_assignments::ListIamPolicyAssignmentsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_iam_policy_assignments::ListIamPolicyAssignmentsOutput {
        crate::operation::list_iam_policy_assignments::ListIamPolicyAssignmentsOutput {
            iam_policy_assignments: self.iam_policy_assignments,
            next_token: self.next_token,
            request_id: self.request_id,
            status: self.status.unwrap_or_default(),
            _request_id: self._request_id,
        }
    }
}
