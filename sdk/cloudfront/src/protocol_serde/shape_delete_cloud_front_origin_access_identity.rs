// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_cloud_front_origin_access_identity_headers(
    input: &crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.if_match {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "if_match",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("If-Match", header_value);
        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_cloud_front_origin_access_identity_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityOutput, crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDenied" => crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError::AccessDenied({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied::de_access_denied_xml_err(response.body().as_ref(), output).map_err(crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CloudFrontOriginAccessIdentityInUse" => crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError::CloudFrontOriginAccessIdentityInUse({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::CloudFrontOriginAccessIdentityInUseBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_front_origin_access_identity_in_use::de_cloud_front_origin_access_identity_in_use_xml_err(response.body().as_ref(), output).map_err(crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidIfMatchVersion" => crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError::InvalidIfMatchVersion({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidIfMatchVersionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_if_match_version::de_invalid_if_match_version_xml_err(response.body().as_ref(), output).map_err(crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchCloudFrontOriginAccessIdentity" => crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchCloudFrontOriginAccessIdentityBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_cloud_front_origin_access_identity::de_no_such_cloud_front_origin_access_identity_xml_err(response.body().as_ref(), output).map_err(crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PreconditionFailed" => crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError::PreconditionFailed({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::PreconditionFailedBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_precondition_failed::de_precondition_failed_xml_err(response.body().as_ref(), output).map_err(crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_cloud_front_origin_access_identity_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityOutput, crate::operation::delete_cloud_front_origin_access_identity::DeleteCloudFrontOriginAccessIdentityError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_cloud_front_origin_access_identity::builders::DeleteCloudFrontOriginAccessIdentityOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
