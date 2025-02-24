// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The <code>VpcConfig</code> configuration object that specifies the VPC that you want the compilation jobs to connect to. For more information on controlling access to your Amazon S3 buckets used for compilation job, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/neo-vpc.html">Give Amazon SageMaker Compilation Jobs Access to Resources in Your Amazon VPC</a>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct NeoVpcConfig {
    /// <p>The VPC security group IDs. IDs have the form of <code>sg-xxxxxxxx</code>. Specify the security groups for the VPC that is specified in the <code>Subnets</code> field.</p>
    #[doc(hidden)]
    pub security_group_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The ID of the subnets in the VPC that you want to connect the compilation job to for accessing the model in Amazon S3.</p>
    #[doc(hidden)]
    pub subnets: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl NeoVpcConfig {
    /// <p>The VPC security group IDs. IDs have the form of <code>sg-xxxxxxxx</code>. Specify the security groups for the VPC that is specified in the <code>Subnets</code> field.</p>
    pub fn security_group_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.security_group_ids.as_deref()
    }
    /// <p>The ID of the subnets in the VPC that you want to connect the compilation job to for accessing the model in Amazon S3.</p>
    pub fn subnets(&self) -> std::option::Option<&[std::string::String]> {
        self.subnets.as_deref()
    }
}
impl NeoVpcConfig {
    /// Creates a new builder-style object to manufacture [`NeoVpcConfig`](crate::types::NeoVpcConfig).
    pub fn builder() -> crate::types::builders::NeoVpcConfigBuilder {
        crate::types::builders::NeoVpcConfigBuilder::default()
    }
}

/// A builder for [`NeoVpcConfig`](crate::types::NeoVpcConfig).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct NeoVpcConfigBuilder {
    pub(crate) security_group_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) subnets: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl NeoVpcConfigBuilder {
    /// Appends an item to `security_group_ids`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>The VPC security group IDs. IDs have the form of <code>sg-xxxxxxxx</code>. Specify the security groups for the VPC that is specified in the <code>Subnets</code> field.</p>
    pub fn security_group_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.security_group_ids.unwrap_or_default();
        v.push(input.into());
        self.security_group_ids = Some(v);
        self
    }
    /// <p>The VPC security group IDs. IDs have the form of <code>sg-xxxxxxxx</code>. Specify the security groups for the VPC that is specified in the <code>Subnets</code> field.</p>
    pub fn set_security_group_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.security_group_ids = input;
        self
    }
    /// Appends an item to `subnets`.
    ///
    /// To override the contents of this collection use [`set_subnets`](Self::set_subnets).
    ///
    /// <p>The ID of the subnets in the VPC that you want to connect the compilation job to for accessing the model in Amazon S3.</p>
    pub fn subnets(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.subnets.unwrap_or_default();
        v.push(input.into());
        self.subnets = Some(v);
        self
    }
    /// <p>The ID of the subnets in the VPC that you want to connect the compilation job to for accessing the model in Amazon S3.</p>
    pub fn set_subnets(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.subnets = input;
        self
    }
    /// Consumes the builder and constructs a [`NeoVpcConfig`](crate::types::NeoVpcConfig).
    pub fn build(self) -> crate::types::NeoVpcConfig {
        crate::types::NeoVpcConfig {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}
