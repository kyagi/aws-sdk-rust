// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_get_media;

pub fn parse_http_error_metadata(
    response: &http::Response<bytes::Bytes>,
) -> Result<
    aws_smithy_types::error::metadata::Builder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response.body(), response.headers())
}

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_client_limit_exceeded_exception;

pub(crate) mod shape_connection_limit_exceeded_exception;

pub(crate) mod shape_get_media_input;

pub(crate) mod shape_get_media_output;

pub(crate) mod shape_invalid_argument_exception;

pub(crate) mod shape_invalid_endpoint_exception;

pub(crate) mod shape_not_authorized_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_start_selector;
