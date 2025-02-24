// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_image_pipeline_images_input(
    input: &crate::operation::list_image_pipeline_images::ListImagePipelineImagesInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_image_pipeline_images_input::ser_list_image_pipeline_images_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_image_pipeline_images_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::list_image_pipeline_images::ListImagePipelineImagesOutput,
    crate::operation::list_image_pipeline_images::ListImagePipelineImagesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "CallRateLimitExceededException" => crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::CallRateLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::CallRateLimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_call_rate_limit_exceeded_exception::de_call_rate_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ClientException" => crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::ClientException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ClientExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_client_exception::de_client_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ForbiddenException" => crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::ForbiddenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ForbiddenExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidPaginationTokenException" => crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::InvalidPaginationTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidPaginationTokenExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_pagination_token_exception::de_invalid_pagination_token_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_image_pipeline_images_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::list_image_pipeline_images::ListImagePipelineImagesOutput,
    crate::operation::list_image_pipeline_images::ListImagePipelineImagesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_image_pipeline_images::builders::ListImagePipelineImagesOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_image_pipeline_images::de_list_image_pipeline_images(response.body().as_ref(), output).map_err(crate::operation::list_image_pipeline_images::ListImagePipelineImagesError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_list_image_pipeline_images(
    value: &[u8],
    mut builder: crate::operation::list_image_pipeline_images::builders::ListImagePipelineImagesOutputBuilder,
) -> Result<
    crate::operation::list_image_pipeline_images::builders::ListImagePipelineImagesOutputBuilder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "imageSummaryList" => {
                        builder = builder.set_image_summary_list(
                            crate::protocol_serde::shape_image_summary_list::de_image_summary_list(
                                tokens,
                            )?,
                        );
                    }
                    "nextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "requestId" => {
                        builder = builder.set_request_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(
                    aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )),
                )
            }
        }
    }
    if tokens.next().is_some() {
        return Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "found more JSON tokens after completing parsing",
            ),
        );
    }
    Ok(builder)
}
