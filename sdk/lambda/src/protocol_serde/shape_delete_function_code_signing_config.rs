// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_function_code_signing_config_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigOutput,
    crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CodeSigningConfigNotFoundException" => crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::CodeSigningConfigNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::CodeSigningConfigNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_code_signing_config_not_found_exception::de_code_signing_config_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceConflictException" => crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::ResourceConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceConflictExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_conflict_exception::de_resource_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_function_code_signing_config_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigOutput,
    crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_function_code_signing_config::builders::DeleteFunctionCodeSigningConfigOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
