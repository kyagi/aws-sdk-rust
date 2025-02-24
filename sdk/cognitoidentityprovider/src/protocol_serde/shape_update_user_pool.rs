// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_user_pool_input(
    input: &crate::operation::update_user_pool::UpdateUserPoolInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_user_pool_input::ser_update_user_pool_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_user_pool_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_user_pool::UpdateUserPoolOutput,
    crate::operation::update_user_pool::UpdateUserPoolError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::update_user_pool::UpdateUserPoolError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::update_user_pool::UpdateUserPoolError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModificationException" => crate::operation::update_user_pool::UpdateUserPoolError::ConcurrentModificationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConcurrentModificationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_concurrent_modification_exception::de_concurrent_modification_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_user_pool::UpdateUserPoolError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalErrorException" => crate::operation::update_user_pool::UpdateUserPoolError::InternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalErrorExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_error_exception::de_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_user_pool::UpdateUserPoolError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidEmailRoleAccessPolicyException" => crate::operation::update_user_pool::UpdateUserPoolError::InvalidEmailRoleAccessPolicyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidEmailRoleAccessPolicyExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_email_role_access_policy_exception::de_invalid_email_role_access_policy_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_user_pool::UpdateUserPoolError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::operation::update_user_pool::UpdateUserPoolError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_user_pool::UpdateUserPoolError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidSmsRoleAccessPolicyException" => crate::operation::update_user_pool::UpdateUserPoolError::InvalidSmsRoleAccessPolicyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidSmsRoleAccessPolicyExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_sms_role_access_policy_exception::de_invalid_sms_role_access_policy_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_user_pool::UpdateUserPoolError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidSmsRoleTrustRelationshipException" => crate::operation::update_user_pool::UpdateUserPoolError::InvalidSmsRoleTrustRelationshipException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidSmsRoleTrustRelationshipExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_sms_role_trust_relationship_exception::de_invalid_sms_role_trust_relationship_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_user_pool::UpdateUserPoolError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotAuthorizedException" => crate::operation::update_user_pool::UpdateUserPoolError::NotAuthorizedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NotAuthorizedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_authorized_exception::de_not_authorized_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_user_pool::UpdateUserPoolError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::update_user_pool::UpdateUserPoolError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_user_pool::UpdateUserPoolError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::update_user_pool::UpdateUserPoolError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_user_pool::UpdateUserPoolError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UserImportInProgressException" => crate::operation::update_user_pool::UpdateUserPoolError::UserImportInProgressException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UserImportInProgressExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_user_import_in_progress_exception::de_user_import_in_progress_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_user_pool::UpdateUserPoolError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UserPoolTaggingException" => crate::operation::update_user_pool::UpdateUserPoolError::UserPoolTaggingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UserPoolTaggingExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_user_pool_tagging_exception::de_user_pool_tagging_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_user_pool::UpdateUserPoolError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::update_user_pool::UpdateUserPoolError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_user_pool_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_user_pool::UpdateUserPoolOutput,
    crate::operation::update_user_pool::UpdateUserPoolError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::update_user_pool::builders::UpdateUserPoolOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
