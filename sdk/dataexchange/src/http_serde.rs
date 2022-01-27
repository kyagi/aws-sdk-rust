// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_send_api_asset(
    input: &crate::input::SendApiAssetInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_1) = &input.asset_id {
        let formatted_2 = AsRef::<str>::as_ref(inner_1);
        if !formatted_2.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_2;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "asset_id",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amzn-dataexchange-asset-id", header_value);
        }
    }
    if let Some(inner_3) = &input.data_set_id {
        let formatted_4 = AsRef::<str>::as_ref(inner_3);
        if !formatted_4.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_4;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "data_set_id",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amzn-dataexchange-data-set-id", header_value);
        }
    }
    if let Some(inner_5) = &input.method {
        let formatted_6 = AsRef::<str>::as_ref(inner_5);
        if !formatted_6.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_6;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "method",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amzn-dataexchange-http-method", header_value);
        }
    }
    if let Some(inner_7) = &input.path {
        let formatted_8 = AsRef::<str>::as_ref(inner_7);
        if !formatted_8.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_8;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "path",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amzn-dataexchange-path", header_value);
        }
    }
    if let Some(inner_9) = &input.revision_id {
        let formatted_10 = AsRef::<str>::as_ref(inner_9);
        if !formatted_10.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_10;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "revision_id",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amzn-dataexchange-revision-id", header_value);
        }
    }
    if let Some(inner_11) = &input.request_headers {
        for (k, v) in inner_11 {
            use std::str::FromStr;
            let header_name = http::header::HeaderName::from_str(&format!(
                "{}{}",
                "x-amzn-dataexchange-header-", &k
            ))
            .map_err(|err| aws_smithy_http::operation::BuildError::InvalidField {
                field: "request_headers",
                details: format!("`{}` cannot be used as a header name: {}", k, err),
            })?;
            use std::convert::TryFrom;
            let header_value = AsRef::<str>::as_ref(v);
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "request_headers",
                        details: format!("`{}` cannot be used as a header value: {}", v, err),
                    }
                })?;
            builder = builder.header(header_name, header_value);
        }
    }
    Ok(builder)
}

pub fn deser_payload_send_api_asset_send_api_asset_output_body(
    body: &[u8],
) -> std::result::Result<std::option::Option<std::string::String>, crate::error::SendApiAssetError>
{
    (!body.is_empty())
        .then(|| {
            let body_str =
                std::str::from_utf8(body).map_err(crate::error::SendApiAssetError::unhandled)?;
            Ok(body_str.to_string())
        })
        .transpose()
}

pub fn deser_prefix_header_send_api_asset_send_api_asset_output_response_headers(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    aws_smithy_http::header::ParseError,
> {
    let headers = aws_smithy_http::header::headers_for_prefix(header_map, "");
    let out: std::result::Result<_, _> = headers.map(|(key, header_name)| {
                            let values = header_map.get_all(header_name);
                            crate::http_serde::deser_prefix_header_send_api_asset_send_api_asset_output_response_headers_inner(values.iter()).map(|v| (key.to_string(), v.unwrap()))
                        }).collect();
    out.map(Some)
}

pub fn deser_prefix_header_send_api_asset_send_api_asset_output_response_headers_inner(
    headers: http::header::ValueIter<http::HeaderValue>,
) -> std::result::Result<Option<std::string::String>, aws_smithy_http::header::ParseError> {
    aws_smithy_http::header::one_or_none(headers)
}
