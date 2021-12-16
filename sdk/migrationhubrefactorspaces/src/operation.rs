// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_application`](crate::client::Client::create_application).
///
/// See [`crate::client::fluent_builders::CreateApplication`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateApplication {
    _private: (),
}
impl CreateApplication {
    /// Creates a new builder-style object to manufacture [`CreateApplicationInput`](crate::input::CreateApplicationInput)
    pub fn builder() -> crate::input::create_application_input::Builder {
        crate::input::create_application_input::Builder::default()
    }
    /// Creates a new `CreateApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateApplication {
    type Output = std::result::Result<
        crate::output::CreateApplicationOutput,
        crate::error::CreateApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_application_error(response)
        } else {
            crate::operation_deser::parse_create_application_response(response)
        }
    }
}

/// Operation shape for `CreateEnvironment`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_environment`](crate::client::Client::create_environment).
///
/// See [`crate::client::fluent_builders::CreateEnvironment`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateEnvironment {
    _private: (),
}
impl CreateEnvironment {
    /// Creates a new builder-style object to manufacture [`CreateEnvironmentInput`](crate::input::CreateEnvironmentInput)
    pub fn builder() -> crate::input::create_environment_input::Builder {
        crate::input::create_environment_input::Builder::default()
    }
    /// Creates a new `CreateEnvironment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateEnvironment {
    type Output = std::result::Result<
        crate::output::CreateEnvironmentOutput,
        crate::error::CreateEnvironmentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_environment_error(response)
        } else {
            crate::operation_deser::parse_create_environment_response(response)
        }
    }
}

/// Operation shape for `CreateRoute`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_route`](crate::client::Client::create_route).
///
/// See [`crate::client::fluent_builders::CreateRoute`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateRoute {
    _private: (),
}
impl CreateRoute {
    /// Creates a new builder-style object to manufacture [`CreateRouteInput`](crate::input::CreateRouteInput)
    pub fn builder() -> crate::input::create_route_input::Builder {
        crate::input::create_route_input::Builder::default()
    }
    /// Creates a new `CreateRoute` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateRoute {
    type Output =
        std::result::Result<crate::output::CreateRouteOutput, crate::error::CreateRouteError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_route_error(response)
        } else {
            crate::operation_deser::parse_create_route_response(response)
        }
    }
}

/// Operation shape for `CreateService`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_service`](crate::client::Client::create_service).
///
/// See [`crate::client::fluent_builders::CreateService`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateService {
    _private: (),
}
impl CreateService {
    /// Creates a new builder-style object to manufacture [`CreateServiceInput`](crate::input::CreateServiceInput)
    pub fn builder() -> crate::input::create_service_input::Builder {
        crate::input::create_service_input::Builder::default()
    }
    /// Creates a new `CreateService` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateService {
    type Output =
        std::result::Result<crate::output::CreateServiceOutput, crate::error::CreateServiceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_service_error(response)
        } else {
            crate::operation_deser::parse_create_service_response(response)
        }
    }
}

/// Operation shape for `DeleteApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_application`](crate::client::Client::delete_application).
///
/// See [`crate::client::fluent_builders::DeleteApplication`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteApplication {
    _private: (),
}
impl DeleteApplication {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationInput`](crate::input::DeleteApplicationInput)
    pub fn builder() -> crate::input::delete_application_input::Builder {
        crate::input::delete_application_input::Builder::default()
    }
    /// Creates a new `DeleteApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteApplication {
    type Output = std::result::Result<
        crate::output::DeleteApplicationOutput,
        crate::error::DeleteApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_application_error(response)
        } else {
            crate::operation_deser::parse_delete_application_response(response)
        }
    }
}

/// Operation shape for `DeleteEnvironment`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_environment`](crate::client::Client::delete_environment).
///
/// See [`crate::client::fluent_builders::DeleteEnvironment`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteEnvironment {
    _private: (),
}
impl DeleteEnvironment {
    /// Creates a new builder-style object to manufacture [`DeleteEnvironmentInput`](crate::input::DeleteEnvironmentInput)
    pub fn builder() -> crate::input::delete_environment_input::Builder {
        crate::input::delete_environment_input::Builder::default()
    }
    /// Creates a new `DeleteEnvironment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteEnvironment {
    type Output = std::result::Result<
        crate::output::DeleteEnvironmentOutput,
        crate::error::DeleteEnvironmentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_environment_error(response)
        } else {
            crate::operation_deser::parse_delete_environment_response(response)
        }
    }
}

/// Operation shape for `DeleteResourcePolicy`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_resource_policy`](crate::client::Client::delete_resource_policy).
///
/// See [`crate::client::fluent_builders::DeleteResourcePolicy`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteResourcePolicy {
    _private: (),
}
impl DeleteResourcePolicy {
    /// Creates a new builder-style object to manufacture [`DeleteResourcePolicyInput`](crate::input::DeleteResourcePolicyInput)
    pub fn builder() -> crate::input::delete_resource_policy_input::Builder {
        crate::input::delete_resource_policy_input::Builder::default()
    }
    /// Creates a new `DeleteResourcePolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteResourcePolicy {
    type Output = std::result::Result<
        crate::output::DeleteResourcePolicyOutput,
        crate::error::DeleteResourcePolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_resource_policy_error(response)
        } else {
            crate::operation_deser::parse_delete_resource_policy_response(response)
        }
    }
}

/// Operation shape for `DeleteRoute`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_route`](crate::client::Client::delete_route).
///
/// See [`crate::client::fluent_builders::DeleteRoute`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteRoute {
    _private: (),
}
impl DeleteRoute {
    /// Creates a new builder-style object to manufacture [`DeleteRouteInput`](crate::input::DeleteRouteInput)
    pub fn builder() -> crate::input::delete_route_input::Builder {
        crate::input::delete_route_input::Builder::default()
    }
    /// Creates a new `DeleteRoute` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteRoute {
    type Output =
        std::result::Result<crate::output::DeleteRouteOutput, crate::error::DeleteRouteError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_route_error(response)
        } else {
            crate::operation_deser::parse_delete_route_response(response)
        }
    }
}

/// Operation shape for `DeleteService`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_service`](crate::client::Client::delete_service).
///
/// See [`crate::client::fluent_builders::DeleteService`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteService {
    _private: (),
}
impl DeleteService {
    /// Creates a new builder-style object to manufacture [`DeleteServiceInput`](crate::input::DeleteServiceInput)
    pub fn builder() -> crate::input::delete_service_input::Builder {
        crate::input::delete_service_input::Builder::default()
    }
    /// Creates a new `DeleteService` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteService {
    type Output =
        std::result::Result<crate::output::DeleteServiceOutput, crate::error::DeleteServiceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_service_error(response)
        } else {
            crate::operation_deser::parse_delete_service_response(response)
        }
    }
}

/// Operation shape for `GetApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_application`](crate::client::Client::get_application).
///
/// See [`crate::client::fluent_builders::GetApplication`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetApplication {
    _private: (),
}
impl GetApplication {
    /// Creates a new builder-style object to manufacture [`GetApplicationInput`](crate::input::GetApplicationInput)
    pub fn builder() -> crate::input::get_application_input::Builder {
        crate::input::get_application_input::Builder::default()
    }
    /// Creates a new `GetApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetApplication {
    type Output =
        std::result::Result<crate::output::GetApplicationOutput, crate::error::GetApplicationError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_application_error(response)
        } else {
            crate::operation_deser::parse_get_application_response(response)
        }
    }
}

/// Operation shape for `GetEnvironment`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_environment`](crate::client::Client::get_environment).
///
/// See [`crate::client::fluent_builders::GetEnvironment`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetEnvironment {
    _private: (),
}
impl GetEnvironment {
    /// Creates a new builder-style object to manufacture [`GetEnvironmentInput`](crate::input::GetEnvironmentInput)
    pub fn builder() -> crate::input::get_environment_input::Builder {
        crate::input::get_environment_input::Builder::default()
    }
    /// Creates a new `GetEnvironment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEnvironment {
    type Output =
        std::result::Result<crate::output::GetEnvironmentOutput, crate::error::GetEnvironmentError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_environment_error(response)
        } else {
            crate::operation_deser::parse_get_environment_response(response)
        }
    }
}

/// Operation shape for `GetResourcePolicy`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_resource_policy`](crate::client::Client::get_resource_policy).
///
/// See [`crate::client::fluent_builders::GetResourcePolicy`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourcePolicy {
    _private: (),
}
impl GetResourcePolicy {
    /// Creates a new builder-style object to manufacture [`GetResourcePolicyInput`](crate::input::GetResourcePolicyInput)
    pub fn builder() -> crate::input::get_resource_policy_input::Builder {
        crate::input::get_resource_policy_input::Builder::default()
    }
    /// Creates a new `GetResourcePolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetResourcePolicy {
    type Output = std::result::Result<
        crate::output::GetResourcePolicyOutput,
        crate::error::GetResourcePolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_policy_error(response)
        } else {
            crate::operation_deser::parse_get_resource_policy_response(response)
        }
    }
}

/// Operation shape for `GetRoute`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_route`](crate::client::Client::get_route).
///
/// See [`crate::client::fluent_builders::GetRoute`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetRoute {
    _private: (),
}
impl GetRoute {
    /// Creates a new builder-style object to manufacture [`GetRouteInput`](crate::input::GetRouteInput)
    pub fn builder() -> crate::input::get_route_input::Builder {
        crate::input::get_route_input::Builder::default()
    }
    /// Creates a new `GetRoute` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRoute {
    type Output = std::result::Result<crate::output::GetRouteOutput, crate::error::GetRouteError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_route_error(response)
        } else {
            crate::operation_deser::parse_get_route_response(response)
        }
    }
}

/// Operation shape for `GetService`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_service`](crate::client::Client::get_service).
///
/// See [`crate::client::fluent_builders::GetService`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetService {
    _private: (),
}
impl GetService {
    /// Creates a new builder-style object to manufacture [`GetServiceInput`](crate::input::GetServiceInput)
    pub fn builder() -> crate::input::get_service_input::Builder {
        crate::input::get_service_input::Builder::default()
    }
    /// Creates a new `GetService` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetService {
    type Output =
        std::result::Result<crate::output::GetServiceOutput, crate::error::GetServiceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_service_error(response)
        } else {
            crate::operation_deser::parse_get_service_response(response)
        }
    }
}

/// Operation shape for `ListApplications`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_applications`](crate::client::Client::list_applications).
///
/// See [`crate::client::fluent_builders::ListApplications`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListApplications {
    _private: (),
}
impl ListApplications {
    /// Creates a new builder-style object to manufacture [`ListApplicationsInput`](crate::input::ListApplicationsInput)
    pub fn builder() -> crate::input::list_applications_input::Builder {
        crate::input::list_applications_input::Builder::default()
    }
    /// Creates a new `ListApplications` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListApplications {
    type Output = std::result::Result<
        crate::output::ListApplicationsOutput,
        crate::error::ListApplicationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_applications_error(response)
        } else {
            crate::operation_deser::parse_list_applications_response(response)
        }
    }
}

/// Operation shape for `ListEnvironments`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_environments`](crate::client::Client::list_environments).
///
/// See [`crate::client::fluent_builders::ListEnvironments`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListEnvironments {
    _private: (),
}
impl ListEnvironments {
    /// Creates a new builder-style object to manufacture [`ListEnvironmentsInput`](crate::input::ListEnvironmentsInput)
    pub fn builder() -> crate::input::list_environments_input::Builder {
        crate::input::list_environments_input::Builder::default()
    }
    /// Creates a new `ListEnvironments` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListEnvironments {
    type Output = std::result::Result<
        crate::output::ListEnvironmentsOutput,
        crate::error::ListEnvironmentsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_environments_error(response)
        } else {
            crate::operation_deser::parse_list_environments_response(response)
        }
    }
}

/// Operation shape for `ListEnvironmentVpcs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_environment_vpcs`](crate::client::Client::list_environment_vpcs).
///
/// See [`crate::client::fluent_builders::ListEnvironmentVpcs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListEnvironmentVpcs {
    _private: (),
}
impl ListEnvironmentVpcs {
    /// Creates a new builder-style object to manufacture [`ListEnvironmentVpcsInput`](crate::input::ListEnvironmentVpcsInput)
    pub fn builder() -> crate::input::list_environment_vpcs_input::Builder {
        crate::input::list_environment_vpcs_input::Builder::default()
    }
    /// Creates a new `ListEnvironmentVpcs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListEnvironmentVpcs {
    type Output = std::result::Result<
        crate::output::ListEnvironmentVpcsOutput,
        crate::error::ListEnvironmentVpcsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_environment_vpcs_error(response)
        } else {
            crate::operation_deser::parse_list_environment_vpcs_response(response)
        }
    }
}

/// Operation shape for `ListRoutes`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_routes`](crate::client::Client::list_routes).
///
/// See [`crate::client::fluent_builders::ListRoutes`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListRoutes {
    _private: (),
}
impl ListRoutes {
    /// Creates a new builder-style object to manufacture [`ListRoutesInput`](crate::input::ListRoutesInput)
    pub fn builder() -> crate::input::list_routes_input::Builder {
        crate::input::list_routes_input::Builder::default()
    }
    /// Creates a new `ListRoutes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRoutes {
    type Output =
        std::result::Result<crate::output::ListRoutesOutput, crate::error::ListRoutesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_routes_error(response)
        } else {
            crate::operation_deser::parse_list_routes_response(response)
        }
    }
}

/// Operation shape for `ListServices`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_services`](crate::client::Client::list_services).
///
/// See [`crate::client::fluent_builders::ListServices`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListServices {
    _private: (),
}
impl ListServices {
    /// Creates a new builder-style object to manufacture [`ListServicesInput`](crate::input::ListServicesInput)
    pub fn builder() -> crate::input::list_services_input::Builder {
        crate::input::list_services_input::Builder::default()
    }
    /// Creates a new `ListServices` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListServices {
    type Output =
        std::result::Result<crate::output::ListServicesOutput, crate::error::ListServicesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_services_error(response)
        } else {
            crate::operation_deser::parse_list_services_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `PutResourcePolicy`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_resource_policy`](crate::client::Client::put_resource_policy).
///
/// See [`crate::client::fluent_builders::PutResourcePolicy`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutResourcePolicy {
    _private: (),
}
impl PutResourcePolicy {
    /// Creates a new builder-style object to manufacture [`PutResourcePolicyInput`](crate::input::PutResourcePolicyInput)
    pub fn builder() -> crate::input::put_resource_policy_input::Builder {
        crate::input::put_resource_policy_input::Builder::default()
    }
    /// Creates a new `PutResourcePolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutResourcePolicy {
    type Output = std::result::Result<
        crate::output::PutResourcePolicyOutput,
        crate::error::PutResourcePolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_resource_policy_error(response)
        } else {
            crate::operation_deser::parse_put_resource_policy_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}
