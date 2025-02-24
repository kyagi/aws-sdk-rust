// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_get_frame_metric_data_input(
    input: &crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_batch_get_frame_metric_data_input::ser_batch_get_frame_metric_data_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_get_frame_metric_data_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataOutput,
    crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_get_frame_metric_data_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataOutput,
    crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::batch_get_frame_metric_data::builders::BatchGetFrameMetricDataOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_batch_get_frame_metric_data::de_batch_get_frame_metric_data(response.body().as_ref(), output).map_err(crate::operation::batch_get_frame_metric_data::BatchGetFrameMetricDataError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_batch_get_frame_metric_data(
    value: &[u8],
    mut builder: crate::operation::batch_get_frame_metric_data::builders::BatchGetFrameMetricDataOutputBuilder,
) -> Result<
    crate::operation::batch_get_frame_metric_data::builders::BatchGetFrameMetricDataOutputBuilder,
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
                    "endTime" => {
                        builder = builder.set_end_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                aws_smithy_types::date_time::Format::DateTimeWithOffset,
                            )?,
                        );
                    }
                    "endTimes" => {
                        builder = builder.set_end_times(
                            crate::protocol_serde::shape_list_of_timestamps::de_list_of_timestamps(
                                tokens,
                            )?,
                        );
                    }
                    "frameMetricData" => {
                        builder = builder.set_frame_metric_data(
                            crate::protocol_serde::shape_frame_metric_data::de_frame_metric_data(
                                tokens,
                            )?,
                        );
                    }
                    "resolution" => {
                        builder = builder.set_resolution(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| {
                                s.to_unescaped()
                                    .map(|u| crate::types::AggregationPeriod::from(u.as_ref()))
                            })
                            .transpose()?,
                        );
                    }
                    "startTime" => {
                        builder = builder.set_start_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                aws_smithy_types::date_time::Format::DateTimeWithOffset,
                            )?,
                        );
                    }
                    "unprocessedEndTimes" => {
                        builder = builder.set_unprocessed_end_times(
                            crate::protocol_serde::shape_unprocessed_end_time_map::de_unprocessed_end_time_map(tokens)?
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
