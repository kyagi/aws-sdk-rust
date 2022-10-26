// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_report_creation_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeReportCreationOutput,
    crate::error::DescribeReportCreationError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeReportCreationError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::DescribeReportCreationError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConstraintViolationException" => crate::error::DescribeReportCreationError {
            meta: generic,
            kind: crate::error::DescribeReportCreationErrorKind::ConstraintViolationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::constraint_violation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_constraint_violation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeReportCreationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::DescribeReportCreationError {
            meta: generic,
            kind: crate::error::DescribeReportCreationErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeReportCreationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidParameterException" => crate::error::DescribeReportCreationError {
            meta: generic,
            kind: crate::error::DescribeReportCreationErrorKind::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeReportCreationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottledException" => crate::error::DescribeReportCreationError {
            meta: generic,
            kind: crate::error::DescribeReportCreationErrorKind::ThrottledException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttled_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeReportCreationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeReportCreationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_report_creation_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeReportCreationOutput,
    crate::error::DescribeReportCreationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_report_creation_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_report_creation(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeReportCreationError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_compliance_summary_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetComplianceSummaryOutput,
    crate::error::GetComplianceSummaryError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetComplianceSummaryError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetComplianceSummaryError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConstraintViolationException" => crate::error::GetComplianceSummaryError {
            meta: generic,
            kind: crate::error::GetComplianceSummaryErrorKind::ConstraintViolationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::constraint_violation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_constraint_violation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetComplianceSummaryError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::GetComplianceSummaryError {
            meta: generic,
            kind: crate::error::GetComplianceSummaryErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetComplianceSummaryError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidParameterException" => crate::error::GetComplianceSummaryError {
            meta: generic,
            kind: crate::error::GetComplianceSummaryErrorKind::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetComplianceSummaryError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottledException" => crate::error::GetComplianceSummaryError {
            meta: generic,
            kind: crate::error::GetComplianceSummaryErrorKind::ThrottledException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttled_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetComplianceSummaryError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetComplianceSummaryError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_compliance_summary_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetComplianceSummaryOutput,
    crate::error::GetComplianceSummaryError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_compliance_summary_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_compliance_summary(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetComplianceSummaryError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_resources_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetResourcesOutput, crate::error::GetResourcesError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetResourcesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetResourcesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => crate::error::GetResourcesError {
            meta: generic,
            kind: crate::error::GetResourcesErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetResourcesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidParameterException" => crate::error::GetResourcesError {
            meta: generic,
            kind: crate::error::GetResourcesErrorKind::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetResourcesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "PaginationTokenExpiredException" => crate::error::GetResourcesError {
            meta: generic,
            kind: crate::error::GetResourcesErrorKind::PaginationTokenExpiredException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::pagination_token_expired_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_pagination_token_expired_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetResourcesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottledException" => crate::error::GetResourcesError {
            meta: generic,
            kind: crate::error::GetResourcesErrorKind::ThrottledException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttled_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetResourcesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetResourcesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_resources_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetResourcesOutput, crate::error::GetResourcesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_resources_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_resources(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetResourcesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_tag_keys_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetTagKeysOutput, crate::error::GetTagKeysError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetTagKeysError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetTagKeysError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => crate::error::GetTagKeysError {
            meta: generic,
            kind: crate::error::GetTagKeysErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetTagKeysError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidParameterException" => crate::error::GetTagKeysError {
            meta: generic,
            kind: crate::error::GetTagKeysErrorKind::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetTagKeysError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "PaginationTokenExpiredException" => crate::error::GetTagKeysError {
            meta: generic,
            kind: crate::error::GetTagKeysErrorKind::PaginationTokenExpiredException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::pagination_token_expired_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_pagination_token_expired_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetTagKeysError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottledException" => crate::error::GetTagKeysError {
            meta: generic,
            kind: crate::error::GetTagKeysErrorKind::ThrottledException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttled_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetTagKeysError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetTagKeysError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_tag_keys_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetTagKeysOutput, crate::error::GetTagKeysError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_tag_keys_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_tag_keys(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetTagKeysError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_tag_values_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetTagValuesOutput, crate::error::GetTagValuesError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetTagValuesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetTagValuesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => crate::error::GetTagValuesError {
            meta: generic,
            kind: crate::error::GetTagValuesErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetTagValuesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidParameterException" => crate::error::GetTagValuesError {
            meta: generic,
            kind: crate::error::GetTagValuesErrorKind::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetTagValuesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "PaginationTokenExpiredException" => crate::error::GetTagValuesError {
            meta: generic,
            kind: crate::error::GetTagValuesErrorKind::PaginationTokenExpiredException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::pagination_token_expired_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_pagination_token_expired_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetTagValuesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottledException" => crate::error::GetTagValuesError {
            meta: generic,
            kind: crate::error::GetTagValuesErrorKind::ThrottledException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttled_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetTagValuesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetTagValuesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_tag_values_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetTagValuesOutput, crate::error::GetTagValuesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_tag_values_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_tag_values(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetTagValuesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_start_report_creation_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::StartReportCreationOutput,
    crate::error::StartReportCreationError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::StartReportCreationError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::StartReportCreationError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModificationException" => crate::error::StartReportCreationError {
            meta: generic,
            kind: crate::error::StartReportCreationErrorKind::ConcurrentModificationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::concurrent_modification_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_concurrent_modification_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartReportCreationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ConstraintViolationException" => crate::error::StartReportCreationError {
            meta: generic,
            kind: crate::error::StartReportCreationErrorKind::ConstraintViolationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::constraint_violation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_constraint_violation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartReportCreationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::StartReportCreationError {
            meta: generic,
            kind: crate::error::StartReportCreationErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartReportCreationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidParameterException" => crate::error::StartReportCreationError {
            meta: generic,
            kind: crate::error::StartReportCreationErrorKind::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartReportCreationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottledException" => crate::error::StartReportCreationError {
            meta: generic,
            kind: crate::error::StartReportCreationErrorKind::ThrottledException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttled_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartReportCreationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::StartReportCreationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_start_report_creation_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::StartReportCreationOutput,
    crate::error::StartReportCreationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::start_report_creation_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_tag_resources_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::TagResourcesOutput, crate::error::TagResourcesError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::TagResourcesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::TagResourcesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => crate::error::TagResourcesError {
            meta: generic,
            kind: crate::error::TagResourcesErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::TagResourcesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidParameterException" => crate::error::TagResourcesError {
            meta: generic,
            kind: crate::error::TagResourcesErrorKind::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::TagResourcesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottledException" => crate::error::TagResourcesError {
            meta: generic,
            kind: crate::error::TagResourcesErrorKind::ThrottledException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttled_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::TagResourcesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::TagResourcesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_tag_resources_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::TagResourcesOutput, crate::error::TagResourcesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::tag_resources_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_tag_resources(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::TagResourcesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_untag_resources_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::UntagResourcesOutput, crate::error::UntagResourcesError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::UntagResourcesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::UntagResourcesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => crate::error::UntagResourcesError {
            meta: generic,
            kind: crate::error::UntagResourcesErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UntagResourcesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidParameterException" => crate::error::UntagResourcesError {
            meta: generic,
            kind: crate::error::UntagResourcesErrorKind::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UntagResourcesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottledException" => crate::error::UntagResourcesError {
            meta: generic,
            kind: crate::error::UntagResourcesErrorKind::ThrottledException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttled_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UntagResourcesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::UntagResourcesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_untag_resources_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::UntagResourcesOutput, crate::error::UntagResourcesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::untag_resources_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_untag_resources(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::UntagResourcesError::unhandled)?;
        output.build()
    })
}
