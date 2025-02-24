// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_cors_policy_input(
    input: &crate::operation::delete_cors_policy::DeleteCorsPolicyInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_cors_policy_input::ser_delete_cors_policy_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_cors_policy_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_cors_policy::DeleteCorsPolicyOutput,
    crate::operation::delete_cors_policy::DeleteCorsPolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::delete_cors_policy::DeleteCorsPolicyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::delete_cors_policy::DeleteCorsPolicyError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ContainerInUseException" => {
            crate::operation::delete_cors_policy::DeleteCorsPolicyError::ContainerInUseException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ContainerInUseExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_container_in_use_exception::de_container_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_cors_policy::DeleteCorsPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ContainerNotFoundException" => {
            crate::operation::delete_cors_policy::DeleteCorsPolicyError::ContainerNotFoundException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ContainerNotFoundExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_container_not_found_exception::de_container_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_cors_policy::DeleteCorsPolicyError::unhandled)?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        "CorsPolicyNotFoundException" => {
            crate::operation::delete_cors_policy::DeleteCorsPolicyError::CorsPolicyNotFoundException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::CorsPolicyNotFoundExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_cors_policy_not_found_exception::de_cors_policy_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_cors_policy::DeleteCorsPolicyError::unhandled)?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        "InternalServerError" => {
            crate::operation::delete_cors_policy::DeleteCorsPolicyError::InternalServerError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServerErrorBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_cors_policy::DeleteCorsPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::delete_cors_policy::DeleteCorsPolicyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_cors_policy_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_cors_policy::DeleteCorsPolicyOutput,
    crate::operation::delete_cors_policy::DeleteCorsPolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::delete_cors_policy::builders::DeleteCorsPolicyOutputBuilder::default(
            );
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
