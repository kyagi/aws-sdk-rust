// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The description of the plugin.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PluginDescription {
    /// <p>Details about a custom plugin.</p>
    #[doc(hidden)]
    pub custom_plugin: std::option::Option<crate::types::CustomPluginDescription>,
}
impl PluginDescription {
    /// <p>Details about a custom plugin.</p>
    pub fn custom_plugin(&self) -> std::option::Option<&crate::types::CustomPluginDescription> {
        self.custom_plugin.as_ref()
    }
}
impl PluginDescription {
    /// Creates a new builder-style object to manufacture [`PluginDescription`](crate::types::PluginDescription).
    pub fn builder() -> crate::types::builders::PluginDescriptionBuilder {
        crate::types::builders::PluginDescriptionBuilder::default()
    }
}

/// A builder for [`PluginDescription`](crate::types::PluginDescription).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct PluginDescriptionBuilder {
    pub(crate) custom_plugin: std::option::Option<crate::types::CustomPluginDescription>,
}
impl PluginDescriptionBuilder {
    /// <p>Details about a custom plugin.</p>
    pub fn custom_plugin(mut self, input: crate::types::CustomPluginDescription) -> Self {
        self.custom_plugin = Some(input);
        self
    }
    /// <p>Details about a custom plugin.</p>
    pub fn set_custom_plugin(
        mut self,
        input: std::option::Option<crate::types::CustomPluginDescription>,
    ) -> Self {
        self.custom_plugin = input;
        self
    }
    /// Consumes the builder and constructs a [`PluginDescription`](crate::types::PluginDescription).
    pub fn build(self) -> crate::types::PluginDescription {
        crate::types::PluginDescription {
            custom_plugin: self.custom_plugin,
        }
    }
}
