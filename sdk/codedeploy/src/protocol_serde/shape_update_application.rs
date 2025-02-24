// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_application_input(
    input: &crate::operation::update_application::UpdateApplicationInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_application_input::ser_update_application_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_application_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_application::UpdateApplicationOutput,
    crate::operation::update_application::UpdateApplicationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::update_application::UpdateApplicationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::update_application::UpdateApplicationError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ApplicationAlreadyExistsException" => crate::operation::update_application::UpdateApplicationError::ApplicationAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ApplicationAlreadyExistsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_application_already_exists_exception::de_application_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_application::UpdateApplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ApplicationDoesNotExistException" => crate::operation::update_application::UpdateApplicationError::ApplicationDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ApplicationDoesNotExistExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_application_does_not_exist_exception::de_application_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_application::UpdateApplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ApplicationNameRequiredException" => crate::operation::update_application::UpdateApplicationError::ApplicationNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ApplicationNameRequiredExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_application_name_required_exception::de_application_name_required_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_application::UpdateApplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidApplicationNameException" => crate::operation::update_application::UpdateApplicationError::InvalidApplicationNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidApplicationNameExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_application_name_exception::de_invalid_application_name_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_application::UpdateApplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::update_application::UpdateApplicationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_application_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_application::UpdateApplicationOutput,
    crate::operation::update_application::UpdateApplicationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::update_application::builders::UpdateApplicationOutputBuilder::default(
            );
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
