// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_publish_metrics_input(
    input: &crate::operation::publish_metrics::PublishMetricsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_publish_metrics_input::ser_publish_metrics_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_publish_metrics_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::publish_metrics::PublishMetricsOutput,
    crate::operation::publish_metrics::PublishMetricsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::publish_metrics::PublishMetricsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::publish_metrics::PublishMetricsError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => {
            crate::operation::publish_metrics::PublishMetricsError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::publish_metrics::PublishMetricsError::unhandled)?;
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
            crate::operation::publish_metrics::PublishMetricsError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::publish_metrics::PublishMetricsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::publish_metrics::PublishMetricsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_publish_metrics_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::publish_metrics::PublishMetricsOutput,
    crate::operation::publish_metrics::PublishMetricsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::publish_metrics::builders::PublishMetricsOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
