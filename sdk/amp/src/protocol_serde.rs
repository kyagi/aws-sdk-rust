// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_create_alert_manager_definition;

pub fn parse_http_error_metadata(
    response: &http::Response<bytes::Bytes>,
) -> Result<
    aws_smithy_types::error::metadata::Builder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response.body(), response.headers())
}

pub(crate) mod shape_create_logging_configuration;

pub(crate) mod shape_create_rule_groups_namespace;

pub(crate) mod shape_create_workspace;

pub(crate) mod shape_delete_alert_manager_definition;

pub(crate) mod shape_delete_logging_configuration;

pub(crate) mod shape_delete_rule_groups_namespace;

pub(crate) mod shape_delete_workspace;

pub(crate) mod shape_describe_alert_manager_definition;

pub(crate) mod shape_describe_logging_configuration;

pub(crate) mod shape_describe_rule_groups_namespace;

pub(crate) mod shape_describe_workspace;

pub(crate) mod shape_list_rule_groups_namespaces;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_workspaces;

pub(crate) mod shape_put_alert_manager_definition;

pub(crate) mod shape_put_rule_groups_namespace;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_logging_configuration;

pub(crate) mod shape_update_workspace_alias;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_alert_manager_definition_input;

pub(crate) mod shape_create_logging_configuration_input;

pub(crate) mod shape_create_rule_groups_namespace_input;

pub(crate) mod shape_create_workspace_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_put_alert_manager_definition_input;

pub(crate) mod shape_put_rule_groups_namespace_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_logging_configuration_input;

pub(crate) mod shape_update_workspace_alias_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_alert_manager_definition_description;

pub(crate) mod shape_alert_manager_definition_status;

pub(crate) mod shape_logging_configuration_metadata;

pub(crate) mod shape_logging_configuration_status;

pub(crate) mod shape_rule_groups_namespace_description;

pub(crate) mod shape_rule_groups_namespace_status;

pub(crate) mod shape_rule_groups_namespace_summary_list;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_validation_exception_field_list;

pub(crate) mod shape_workspace_description;

pub(crate) mod shape_workspace_status;

pub(crate) mod shape_workspace_summary_list;

pub(crate) mod shape_rule_groups_namespace_summary;

pub(crate) mod shape_validation_exception_field;

pub(crate) mod shape_workspace_summary;
