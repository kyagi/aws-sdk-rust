// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_dead_letter_source_queues_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesOutput,
    crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AWS.SimpleQueueService.NonExistentQueue" => crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::QueueDoesNotExist({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::QueueDoesNotExistBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_queue_does_not_exist::de_queue_does_not_exist_xml_err(response.body().as_ref(), output).map_err(crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_dead_letter_source_queues_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesOutput,
    crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_dead_letter_source_queues::builders::ListDeadLetterSourceQueuesOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_dead_letter_source_queues::de_list_dead_letter_source_queues(response.body().as_ref(), output).map_err(crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_list_dead_letter_source_queues(inp: &[u8], mut builder: crate::operation::list_dead_letter_source_queues::builders::ListDeadLetterSourceQueuesOutputBuilder) -> Result<crate::operation::list_dead_letter_source_queues::builders::ListDeadLetterSourceQueuesOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ListDeadLetterSourceQueuesResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ListDeadLetterSourceQueuesResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ListDeadLetterSourceQueuesResult")) {
            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ListDeadLetterSourceQueuesResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("QueueUrl") /* queueUrls com.amazonaws.sqs.synthetic#ListDeadLetterSourceQueuesOutput$queueUrls */ =>  {
                let var_1 =
                    Some(
                        Result::<std::vec::Vec<std::string::String>, aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_2 = builder.queue_urls.take().unwrap_or_default();
                            list_2.push(
                                Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                                    aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                                    .into()
                                )
                                ?
                            );
                            list_2
                        })
                        ?
                    )
                ;
                builder = builder.set_queue_urls(var_1);
            }
            ,
            s if s.matches("NextToken") /* NextToken com.amazonaws.sqs.synthetic#ListDeadLetterSourceQueuesOutput$NextToken */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_3);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected ListDeadLetterSourceQueuesResult tag",
        ));
    };
    Ok(builder)
}
