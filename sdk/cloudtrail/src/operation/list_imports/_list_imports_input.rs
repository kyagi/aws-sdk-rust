// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListImportsInput {
    /// <p> The maximum number of imports to display on a single page. </p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p> The ARN of the destination event data store. </p>
    #[doc(hidden)]
    pub destination: std::option::Option<std::string::String>,
    /// <p> The status of the import. </p>
    #[doc(hidden)]
    pub import_status: std::option::Option<crate::types::ImportStatus>,
    /// <p> A token you can use to get the next page of import results. </p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListImportsInput {
    /// <p> The maximum number of imports to display on a single page. </p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p> The ARN of the destination event data store. </p>
    pub fn destination(&self) -> std::option::Option<&str> {
        self.destination.as_deref()
    }
    /// <p> The status of the import. </p>
    pub fn import_status(&self) -> std::option::Option<&crate::types::ImportStatus> {
        self.import_status.as_ref()
    }
    /// <p> A token you can use to get the next page of import results. </p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListImportsInput {
    /// Creates a new builder-style object to manufacture [`ListImportsInput`](crate::operation::list_imports::ListImportsInput).
    pub fn builder() -> crate::operation::list_imports::builders::ListImportsInputBuilder {
        crate::operation::list_imports::builders::ListImportsInputBuilder::default()
    }
}

/// A builder for [`ListImportsInput`](crate::operation::list_imports::ListImportsInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ListImportsInputBuilder {
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) destination: std::option::Option<std::string::String>,
    pub(crate) import_status: std::option::Option<crate::types::ImportStatus>,
    pub(crate) next_token: std::option::Option<std::string::String>,
}
impl ListImportsInputBuilder {
    /// <p> The maximum number of imports to display on a single page. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p> The maximum number of imports to display on a single page. </p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p> The ARN of the destination event data store. </p>
    pub fn destination(mut self, input: impl Into<std::string::String>) -> Self {
        self.destination = Some(input.into());
        self
    }
    /// <p> The ARN of the destination event data store. </p>
    pub fn set_destination(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.destination = input;
        self
    }
    /// <p> The status of the import. </p>
    pub fn import_status(mut self, input: crate::types::ImportStatus) -> Self {
        self.import_status = Some(input);
        self
    }
    /// <p> The status of the import. </p>
    pub fn set_import_status(
        mut self,
        input: std::option::Option<crate::types::ImportStatus>,
    ) -> Self {
        self.import_status = input;
        self
    }
    /// <p> A token you can use to get the next page of import results. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p> A token you can use to get the next page of import results. </p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListImportsInput`](crate::operation::list_imports::ListImportsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::list_imports::ListImportsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::list_imports::ListImportsInput {
            max_results: self.max_results,
            destination: self.destination,
            import_status: self.import_status,
            next_token: self.next_token,
        })
    }
}
