// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_recommendations_input(
    input: &crate::operation::start_recommendations::StartRecommendationsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_start_recommendations_input::ser_start_recommendations_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_recommendations_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::start_recommendations::StartRecommendationsOutput,
    crate::operation::start_recommendations::StartRecommendationsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::start_recommendations::StartRecommendationsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::start_recommendations::StartRecommendationsError::unhandled(
                    generic,
                ),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedFault" => crate::operation::start_recommendations::StartRecommendationsError::AccessDeniedFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_fault::de_access_denied_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::start_recommendations::StartRecommendationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidResourceStateFault" => crate::operation::start_recommendations::StartRecommendationsError::InvalidResourceStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidResourceStateFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_resource_state_fault::de_invalid_resource_state_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::start_recommendations::StartRecommendationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundFault" => crate::operation::start_recommendations::StartRecommendationsError::ResourceNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_fault::de_resource_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::start_recommendations::StartRecommendationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::start_recommendations::StartRecommendationsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_recommendations_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::start_recommendations::StartRecommendationsOutput,
    crate::operation::start_recommendations::StartRecommendationsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::start_recommendations::builders::StartRecommendationsOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
