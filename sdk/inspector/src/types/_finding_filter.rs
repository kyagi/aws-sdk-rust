// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This data type is used as a request parameter in the <code>ListFindings</code> action.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct FindingFilter {
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>agentId</b> property of the <code>Finding</code> data type.</p>
    #[doc(hidden)]
    pub agent_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>autoScalingGroup</b> property of the <code>Finding</code> data type.</p>
    #[doc(hidden)]
    pub auto_scaling_groups: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>ruleName</b> property of the <code>Finding</code> data type.</p>
    #[doc(hidden)]
    pub rule_names: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>severity</b> property of the <code>Finding</code> data type.</p>
    #[doc(hidden)]
    pub severities: std::option::Option<std::vec::Vec<crate::types::Severity>>,
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>rulesPackageArn</b> property of the <code>Finding</code> data type.</p>
    #[doc(hidden)]
    pub rules_package_arns: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>For a record to match a filter, the list of values that are specified for this data type property must be contained in the list of values of the <b>attributes</b> property of the <code>Finding</code> data type.</p>
    #[doc(hidden)]
    pub attributes: std::option::Option<std::vec::Vec<crate::types::Attribute>>,
    /// <p>For a record to match a filter, the value that is specified for this data type property must be contained in the list of values of the <b>userAttributes</b> property of the <code>Finding</code> data type.</p>
    #[doc(hidden)]
    pub user_attributes: std::option::Option<std::vec::Vec<crate::types::Attribute>>,
    /// <p>The time range during which the finding is generated.</p>
    #[doc(hidden)]
    pub creation_time_range: std::option::Option<crate::types::TimestampRange>,
}
impl FindingFilter {
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>agentId</b> property of the <code>Finding</code> data type.</p>
    pub fn agent_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.agent_ids.as_deref()
    }
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>autoScalingGroup</b> property of the <code>Finding</code> data type.</p>
    pub fn auto_scaling_groups(&self) -> std::option::Option<&[std::string::String]> {
        self.auto_scaling_groups.as_deref()
    }
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>ruleName</b> property of the <code>Finding</code> data type.</p>
    pub fn rule_names(&self) -> std::option::Option<&[std::string::String]> {
        self.rule_names.as_deref()
    }
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>severity</b> property of the <code>Finding</code> data type.</p>
    pub fn severities(&self) -> std::option::Option<&[crate::types::Severity]> {
        self.severities.as_deref()
    }
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>rulesPackageArn</b> property of the <code>Finding</code> data type.</p>
    pub fn rules_package_arns(&self) -> std::option::Option<&[std::string::String]> {
        self.rules_package_arns.as_deref()
    }
    /// <p>For a record to match a filter, the list of values that are specified for this data type property must be contained in the list of values of the <b>attributes</b> property of the <code>Finding</code> data type.</p>
    pub fn attributes(&self) -> std::option::Option<&[crate::types::Attribute]> {
        self.attributes.as_deref()
    }
    /// <p>For a record to match a filter, the value that is specified for this data type property must be contained in the list of values of the <b>userAttributes</b> property of the <code>Finding</code> data type.</p>
    pub fn user_attributes(&self) -> std::option::Option<&[crate::types::Attribute]> {
        self.user_attributes.as_deref()
    }
    /// <p>The time range during which the finding is generated.</p>
    pub fn creation_time_range(&self) -> std::option::Option<&crate::types::TimestampRange> {
        self.creation_time_range.as_ref()
    }
}
impl FindingFilter {
    /// Creates a new builder-style object to manufacture [`FindingFilter`](crate::types::FindingFilter).
    pub fn builder() -> crate::types::builders::FindingFilterBuilder {
        crate::types::builders::FindingFilterBuilder::default()
    }
}

/// A builder for [`FindingFilter`](crate::types::FindingFilter).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct FindingFilterBuilder {
    pub(crate) agent_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) auto_scaling_groups: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) rule_names: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) severities: std::option::Option<std::vec::Vec<crate::types::Severity>>,
    pub(crate) rules_package_arns: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) attributes: std::option::Option<std::vec::Vec<crate::types::Attribute>>,
    pub(crate) user_attributes: std::option::Option<std::vec::Vec<crate::types::Attribute>>,
    pub(crate) creation_time_range: std::option::Option<crate::types::TimestampRange>,
}
impl FindingFilterBuilder {
    /// Appends an item to `agent_ids`.
    ///
    /// To override the contents of this collection use [`set_agent_ids`](Self::set_agent_ids).
    ///
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>agentId</b> property of the <code>Finding</code> data type.</p>
    pub fn agent_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.agent_ids.unwrap_or_default();
        v.push(input.into());
        self.agent_ids = Some(v);
        self
    }
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>agentId</b> property of the <code>Finding</code> data type.</p>
    pub fn set_agent_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.agent_ids = input;
        self
    }
    /// Appends an item to `auto_scaling_groups`.
    ///
    /// To override the contents of this collection use [`set_auto_scaling_groups`](Self::set_auto_scaling_groups).
    ///
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>autoScalingGroup</b> property of the <code>Finding</code> data type.</p>
    pub fn auto_scaling_groups(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.auto_scaling_groups.unwrap_or_default();
        v.push(input.into());
        self.auto_scaling_groups = Some(v);
        self
    }
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>autoScalingGroup</b> property of the <code>Finding</code> data type.</p>
    pub fn set_auto_scaling_groups(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.auto_scaling_groups = input;
        self
    }
    /// Appends an item to `rule_names`.
    ///
    /// To override the contents of this collection use [`set_rule_names`](Self::set_rule_names).
    ///
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>ruleName</b> property of the <code>Finding</code> data type.</p>
    pub fn rule_names(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.rule_names.unwrap_or_default();
        v.push(input.into());
        self.rule_names = Some(v);
        self
    }
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>ruleName</b> property of the <code>Finding</code> data type.</p>
    pub fn set_rule_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.rule_names = input;
        self
    }
    /// Appends an item to `severities`.
    ///
    /// To override the contents of this collection use [`set_severities`](Self::set_severities).
    ///
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>severity</b> property of the <code>Finding</code> data type.</p>
    pub fn severities(mut self, input: crate::types::Severity) -> Self {
        let mut v = self.severities.unwrap_or_default();
        v.push(input);
        self.severities = Some(v);
        self
    }
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>severity</b> property of the <code>Finding</code> data type.</p>
    pub fn set_severities(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Severity>>,
    ) -> Self {
        self.severities = input;
        self
    }
    /// Appends an item to `rules_package_arns`.
    ///
    /// To override the contents of this collection use [`set_rules_package_arns`](Self::set_rules_package_arns).
    ///
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>rulesPackageArn</b> property of the <code>Finding</code> data type.</p>
    pub fn rules_package_arns(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.rules_package_arns.unwrap_or_default();
        v.push(input.into());
        self.rules_package_arns = Some(v);
        self
    }
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>rulesPackageArn</b> property of the <code>Finding</code> data type.</p>
    pub fn set_rules_package_arns(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.rules_package_arns = input;
        self
    }
    /// Appends an item to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>For a record to match a filter, the list of values that are specified for this data type property must be contained in the list of values of the <b>attributes</b> property of the <code>Finding</code> data type.</p>
    pub fn attributes(mut self, input: crate::types::Attribute) -> Self {
        let mut v = self.attributes.unwrap_or_default();
        v.push(input);
        self.attributes = Some(v);
        self
    }
    /// <p>For a record to match a filter, the list of values that are specified for this data type property must be contained in the list of values of the <b>attributes</b> property of the <code>Finding</code> data type.</p>
    pub fn set_attributes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Attribute>>,
    ) -> Self {
        self.attributes = input;
        self
    }
    /// Appends an item to `user_attributes`.
    ///
    /// To override the contents of this collection use [`set_user_attributes`](Self::set_user_attributes).
    ///
    /// <p>For a record to match a filter, the value that is specified for this data type property must be contained in the list of values of the <b>userAttributes</b> property of the <code>Finding</code> data type.</p>
    pub fn user_attributes(mut self, input: crate::types::Attribute) -> Self {
        let mut v = self.user_attributes.unwrap_or_default();
        v.push(input);
        self.user_attributes = Some(v);
        self
    }
    /// <p>For a record to match a filter, the value that is specified for this data type property must be contained in the list of values of the <b>userAttributes</b> property of the <code>Finding</code> data type.</p>
    pub fn set_user_attributes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Attribute>>,
    ) -> Self {
        self.user_attributes = input;
        self
    }
    /// <p>The time range during which the finding is generated.</p>
    pub fn creation_time_range(mut self, input: crate::types::TimestampRange) -> Self {
        self.creation_time_range = Some(input);
        self
    }
    /// <p>The time range during which the finding is generated.</p>
    pub fn set_creation_time_range(
        mut self,
        input: std::option::Option<crate::types::TimestampRange>,
    ) -> Self {
        self.creation_time_range = input;
        self
    }
    /// Consumes the builder and constructs a [`FindingFilter`](crate::types::FindingFilter).
    pub fn build(self) -> crate::types::FindingFilter {
        crate::types::FindingFilter {
            agent_ids: self.agent_ids,
            auto_scaling_groups: self.auto_scaling_groups,
            rule_names: self.rule_names,
            severities: self.severities,
            rules_package_arns: self.rules_package_arns,
            attributes: self.attributes,
            user_attributes: self.user_attributes,
            creation_time_range: self.creation_time_range,
        }
    }
}
