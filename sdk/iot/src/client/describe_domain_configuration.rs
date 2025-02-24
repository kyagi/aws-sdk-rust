// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDomainConfiguration`](crate::operation::describe_domain_configuration::builders::DescribeDomainConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_configuration_name(impl Into<String>)`](crate::operation::describe_domain_configuration::builders::DescribeDomainConfigurationFluentBuilder::domain_configuration_name) / [`set_domain_configuration_name(Option<String>)`](crate::operation::describe_domain_configuration::builders::DescribeDomainConfigurationFluentBuilder::set_domain_configuration_name): <p>The name of the domain configuration.</p>
    /// - On success, responds with [`DescribeDomainConfigurationOutput`](crate::operation::describe_domain_configuration::DescribeDomainConfigurationOutput) with field(s):
    ///   - [`domain_configuration_name(Option<String>)`](crate::operation::describe_domain_configuration::DescribeDomainConfigurationOutput::domain_configuration_name): <p>The name of the domain configuration.</p>
    ///   - [`domain_configuration_arn(Option<String>)`](crate::operation::describe_domain_configuration::DescribeDomainConfigurationOutput::domain_configuration_arn): <p>The ARN of the domain configuration.</p>
    ///   - [`domain_name(Option<String>)`](crate::operation::describe_domain_configuration::DescribeDomainConfigurationOutput::domain_name): <p>The name of the domain.</p>
    ///   - [`server_certificates(Option<Vec<ServerCertificateSummary>>)`](crate::operation::describe_domain_configuration::DescribeDomainConfigurationOutput::server_certificates): <p>A list containing summary information about the server certificate included in the domain configuration.</p>
    ///   - [`authorizer_config(Option<AuthorizerConfig>)`](crate::operation::describe_domain_configuration::DescribeDomainConfigurationOutput::authorizer_config): <p>An object that specifies the authorization service for a domain.</p>
    ///   - [`domain_configuration_status(Option<DomainConfigurationStatus>)`](crate::operation::describe_domain_configuration::DescribeDomainConfigurationOutput::domain_configuration_status): <p>A Boolean value that specifies the current state of the domain configuration.</p>
    ///   - [`service_type(Option<ServiceType>)`](crate::operation::describe_domain_configuration::DescribeDomainConfigurationOutput::service_type): <p>The type of service delivered by the endpoint.</p>
    ///   - [`domain_type(Option<DomainType>)`](crate::operation::describe_domain_configuration::DescribeDomainConfigurationOutput::domain_type): <p>The type of the domain.</p>
    ///   - [`last_status_change_date(Option<DateTime>)`](crate::operation::describe_domain_configuration::DescribeDomainConfigurationOutput::last_status_change_date): <p>The date and time the domain configuration's status was last changed.</p>
    /// - On failure, responds with [`SdkError<DescribeDomainConfigurationError>`](crate::operation::describe_domain_configuration::DescribeDomainConfigurationError)
    pub fn describe_domain_configuration(&self) -> crate::operation::describe_domain_configuration::builders::DescribeDomainConfigurationFluentBuilder{
        crate::operation::describe_domain_configuration::builders::DescribeDomainConfigurationFluentBuilder::new(self.handle.clone())
    }
}
