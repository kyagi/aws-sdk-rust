// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_rule_groups_namespace_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceOutput,
    crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerException" => crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_internal_server_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_rule_groups_namespace_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceOutput,
    crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_rule_groups_namespace::builders::DescribeRuleGroupsNamespaceOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_rule_groups_namespace::de_describe_rule_groups_namespace(response.body().as_ref(), output).map_err(crate::operation::describe_rule_groups_namespace::DescribeRuleGroupsNamespaceError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_describe_rule_groups_namespace(value: &[u8], mut builder: crate::operation::describe_rule_groups_namespace::builders::DescribeRuleGroupsNamespaceOutputBuilder) -> Result<crate::operation::describe_rule_groups_namespace::builders::DescribeRuleGroupsNamespaceOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "ruleGroupsNamespace" => {
                        builder = builder.set_rule_groups_namespace(
                            crate::protocol_serde::shape_rule_groups_namespace_description::de_rule_groups_namespace_description(tokens)?
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
