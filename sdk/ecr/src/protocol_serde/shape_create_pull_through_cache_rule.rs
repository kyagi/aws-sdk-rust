// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_pull_through_cache_rule_input(
    input: &crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_pull_through_cache_rule_input::ser_create_pull_through_cache_rule_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_pull_through_cache_rule_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleOutput,
    crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidParameterException" => crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PullThroughCacheRuleAlreadyExistsException" => crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::PullThroughCacheRuleAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::PullThroughCacheRuleAlreadyExistsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_pull_through_cache_rule_already_exists_exception::de_pull_through_cache_rule_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServerException" => crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::ServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_server_exception::de_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedUpstreamRegistryException" => crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::UnsupportedUpstreamRegistryException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedUpstreamRegistryExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_upstream_registry_exception::de_unsupported_upstream_registry_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_pull_through_cache_rule_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleOutput,
    crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_pull_through_cache_rule::builders::CreatePullThroughCacheRuleOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_pull_through_cache_rule::de_create_pull_through_cache_rule(response.body().as_ref(), output).map_err(crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_create_pull_through_cache_rule(value: &[u8], mut builder: crate::operation::create_pull_through_cache_rule::builders::CreatePullThroughCacheRuleOutputBuilder) -> Result<crate::operation::create_pull_through_cache_rule::builders::CreatePullThroughCacheRuleOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "ecrRepositoryPrefix" => {
                        builder = builder.set_ecr_repository_prefix(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "upstreamRegistryUrl" => {
                        builder = builder.set_upstream_registry_url(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "createdAt" => {
                        builder = builder.set_created_at(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                aws_smithy_types::date_time::Format::EpochSeconds,
                            )?,
                        );
                    }
                    "registryId" => {
                        builder = builder.set_registry_id(
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
