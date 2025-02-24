// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_cluster_iam_roles_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::modify_cluster_iam_roles::ModifyClusterIamRolesOutput,
    crate::operation::modify_cluster_iam_roles::ModifyClusterIamRolesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::modify_cluster_iam_roles::ModifyClusterIamRolesError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code =
        match generic.code() {
            Some(code) => code,
            None => return Err(
                crate::operation::modify_cluster_iam_roles::ModifyClusterIamRolesError::unhandled(
                    generic,
                ),
            ),
        };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ClusterNotFound" => crate::operation::modify_cluster_iam_roles::ModifyClusterIamRolesError::ClusterNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ClusterNotFoundFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cluster_not_found_fault::de_cluster_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::operation::modify_cluster_iam_roles::ModifyClusterIamRolesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidClusterState" => crate::operation::modify_cluster_iam_roles::ModifyClusterIamRolesError::InvalidClusterStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidClusterStateFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_cluster_state_fault::de_invalid_cluster_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::operation::modify_cluster_iam_roles::ModifyClusterIamRolesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::modify_cluster_iam_roles::ModifyClusterIamRolesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_cluster_iam_roles_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::modify_cluster_iam_roles::ModifyClusterIamRolesOutput,
    crate::operation::modify_cluster_iam_roles::ModifyClusterIamRolesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_cluster_iam_roles::builders::ModifyClusterIamRolesOutputBuilder::default();
        let _ = response;
        output =
            crate::protocol_serde::shape_modify_cluster_iam_roles::de_modify_cluster_iam_roles(
                response.body().as_ref(),
                output,
            )
            .map_err(
                crate::operation::modify_cluster_iam_roles::ModifyClusterIamRolesError::unhandled,
            )?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_cluster_iam_roles(
    inp: &[u8],
    mut builder: crate::operation::modify_cluster_iam_roles::builders::ModifyClusterIamRolesOutputBuilder,
) -> Result<
    crate::operation::modify_cluster_iam_roles::builders::ModifyClusterIamRolesOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyClusterIamRolesResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyClusterIamRolesResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ModifyClusterIamRolesResult")) {
            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ModifyClusterIamRolesResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Cluster") /* Cluster com.amazonaws.redshift.synthetic#ModifyClusterIamRolesOutput$Cluster */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_cluster::de_cluster(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cluster(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected ModifyClusterIamRolesResult tag",
        ));
    };
    Ok(builder)
}
