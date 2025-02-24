// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_bgp_failover_test_input(
    input: &crate::operation::stop_bgp_failover_test::StopBgpFailoverTestInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_stop_bgp_failover_test_input::ser_stop_bgp_failover_test_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_bgp_failover_test_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::stop_bgp_failover_test::StopBgpFailoverTestOutput,
    crate::operation::stop_bgp_failover_test::StopBgpFailoverTestError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::stop_bgp_failover_test::StopBgpFailoverTestError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::stop_bgp_failover_test::StopBgpFailoverTestError::unhandled(
                    generic,
                ),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DirectConnectClientException" => crate::operation::stop_bgp_failover_test::StopBgpFailoverTestError::DirectConnectClientException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DirectConnectClientExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_direct_connect_client_exception::de_direct_connect_client_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::stop_bgp_failover_test::StopBgpFailoverTestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DirectConnectServerException" => crate::operation::stop_bgp_failover_test::StopBgpFailoverTestError::DirectConnectServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DirectConnectServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_direct_connect_server_exception::de_direct_connect_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::stop_bgp_failover_test::StopBgpFailoverTestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::stop_bgp_failover_test::StopBgpFailoverTestError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_bgp_failover_test_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::stop_bgp_failover_test::StopBgpFailoverTestOutput,
    crate::operation::stop_bgp_failover_test::StopBgpFailoverTestError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::stop_bgp_failover_test::builders::StopBgpFailoverTestOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_stop_bgp_failover_test::de_stop_bgp_failover_test(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::stop_bgp_failover_test::StopBgpFailoverTestError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_stop_bgp_failover_test(
    value: &[u8],
    mut builder: crate::operation::stop_bgp_failover_test::builders::StopBgpFailoverTestOutputBuilder,
) -> Result<
    crate::operation::stop_bgp_failover_test::builders::StopBgpFailoverTestOutputBuilder,
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
                    "virtualInterfaceTest" => {
                        builder = builder.set_virtual_interface_test(
                            crate::protocol_serde::shape_virtual_interface_test_history::de_virtual_interface_test_history(tokens)?
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
