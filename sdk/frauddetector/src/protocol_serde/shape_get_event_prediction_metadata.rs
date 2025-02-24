// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_event_prediction_metadata_input(
    input: &crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_event_prediction_metadata_input::ser_get_event_prediction_metadata_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_event_prediction_metadata_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataOutput,
    crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerException" => crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_event_prediction_metadata_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataOutput,
    crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_event_prediction_metadata::builders::GetEventPredictionMetadataOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_event_prediction_metadata::de_get_event_prediction_metadata(response.body().as_ref(), output).map_err(crate::operation::get_event_prediction_metadata::GetEventPredictionMetadataError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_get_event_prediction_metadata(value: &[u8], mut builder: crate::operation::get_event_prediction_metadata::builders::GetEventPredictionMetadataOutputBuilder) -> Result<crate::operation::get_event_prediction_metadata::builders::GetEventPredictionMetadataOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "eventId" => {
                        builder = builder.set_event_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "eventTypeName" => {
                        builder = builder.set_event_type_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "entityId" => {
                        builder = builder.set_entity_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "entityType" => {
                        builder = builder.set_entity_type(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "eventTimestamp" => {
                        builder = builder.set_event_timestamp(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "detectorId" => {
                        builder = builder.set_detector_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "detectorVersionId" => {
                        builder = builder.set_detector_version_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "detectorVersionStatus" => {
                        builder = builder.set_detector_version_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "eventVariables" => {
                        builder = builder.set_event_variables(
                            crate::protocol_serde::shape_list_of_event_variable_summaries::de_list_of_event_variable_summaries(tokens)?
                        );
                    }
                    "rules" => {
                        builder = builder.set_rules(
                            crate::protocol_serde::shape_evaluated_rule_list::de_evaluated_rule_list(tokens)?
                        );
                    }
                    "ruleExecutionMode" => {
                        builder = builder.set_rule_execution_mode(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| {
                                s.to_unescaped()
                                    .map(|u| crate::types::RuleExecutionMode::from(u.as_ref()))
                            })
                            .transpose()?,
                        );
                    }
                    "outcomes" => {
                        builder = builder.set_outcomes(
                            crate::protocol_serde::shape_list_of_strings::de_list_of_strings(
                                tokens,
                            )?,
                        );
                    }
                    "evaluatedModelVersions" => {
                        builder = builder.set_evaluated_model_versions(
                            crate::protocol_serde::shape_list_of_evaluated_model_versions::de_list_of_evaluated_model_versions(tokens)?
                        );
                    }
                    "evaluatedExternalModels" => {
                        builder = builder.set_evaluated_external_models(
                            crate::protocol_serde::shape_list_of_evaluated_external_models::de_list_of_evaluated_external_models(tokens)?
                        );
                    }
                    "predictionTimestamp" => {
                        builder = builder.set_prediction_timestamp(
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
