// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_start_campaign_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::start_campaign::StartCampaignOutput,
    crate::operation::start_campaign::StartCampaignError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::start_campaign::StartCampaignError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::start_campaign::StartCampaignError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::start_campaign::StartCampaignError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_campaign::StartCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_access_denied_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::start_campaign::StartCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ConflictException" => {
            crate::operation::start_campaign::StartCampaignError::ConflictException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ConflictExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_campaign::StartCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_conflict_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::start_campaign::StartCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InternalServerException" => {
            crate::operation::start_campaign::StartCampaignError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_campaign::StartCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_internal_server_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::start_campaign::StartCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidCampaignStateException" => {
            crate::operation::start_campaign::StartCampaignError::InvalidCampaignStateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidCampaignStateExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_campaign_state_exception::de_invalid_campaign_state_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_campaign::StartCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_invalid_campaign_state_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::start_campaign::StartCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceNotFoundException" => {
            crate::operation::start_campaign::StartCampaignError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_campaign::StartCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_resource_not_found_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::start_campaign::StartCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ThrottlingException" => {
            crate::operation::start_campaign::StartCampaignError::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_campaign::StartCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_throttling_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::start_campaign::StartCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ValidationException" => {
            crate::operation::start_campaign::StartCampaignError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_campaign::StartCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_validation_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::start_campaign::StartCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::start_campaign::StartCampaignError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_campaign_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::start_campaign::StartCampaignOutput,
    crate::operation::start_campaign::StartCampaignError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::start_campaign::builders::StartCampaignOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
