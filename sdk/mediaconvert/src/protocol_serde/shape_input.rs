// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Input,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.audio_selector_groups {
        #[allow(unused_mut)]
        let mut object_2 = object.key("audioSelectorGroups").start_object();
        for (key_3, value_4) in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_5 = object_2.key(key_3.as_str()).start_object();
                crate::protocol_serde::shape_audio_selector_group::ser_audio_selector_group(
                    &mut object_5,
                    value_4,
                )?;
                object_5.finish();
            }
        }
        object_2.finish();
    }
    if let Some(var_6) = &input.audio_selectors {
        #[allow(unused_mut)]
        let mut object_7 = object.key("audioSelectors").start_object();
        for (key_8, value_9) in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_10 = object_7.key(key_8.as_str()).start_object();
                crate::protocol_serde::shape_audio_selector::ser_audio_selector(
                    &mut object_10,
                    value_9,
                )?;
                object_10.finish();
            }
        }
        object_7.finish();
    }
    if let Some(var_11) = &input.caption_selectors {
        #[allow(unused_mut)]
        let mut object_12 = object.key("captionSelectors").start_object();
        for (key_13, value_14) in var_11 {
            {
                #[allow(unused_mut)]
                let mut object_15 = object_12.key(key_13.as_str()).start_object();
                crate::protocol_serde::shape_caption_selector::ser_caption_selector(
                    &mut object_15,
                    value_14,
                )?;
                object_15.finish();
            }
        }
        object_12.finish();
    }
    if let Some(var_16) = &input.crop {
        #[allow(unused_mut)]
        let mut object_17 = object.key("crop").start_object();
        crate::protocol_serde::shape_rectangle::ser_rectangle(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.deblock_filter {
        object.key("deblockFilter").string(var_18.as_str());
    }
    if let Some(var_19) = &input.decryption_settings {
        #[allow(unused_mut)]
        let mut object_20 = object.key("decryptionSettings").start_object();
        crate::protocol_serde::shape_input_decryption_settings::ser_input_decryption_settings(
            &mut object_20,
            var_19,
        )?;
        object_20.finish();
    }
    if let Some(var_21) = &input.denoise_filter {
        object.key("denoiseFilter").string(var_21.as_str());
    }
    if let Some(var_22) = &input.dolby_vision_metadata_xml {
        object.key("dolbyVisionMetadataXml").string(var_22.as_str());
    }
    if let Some(var_23) = &input.file_input {
        object.key("fileInput").string(var_23.as_str());
    }
    if let Some(var_24) = &input.filter_enable {
        object.key("filterEnable").string(var_24.as_str());
    }
    if input.filter_strength != 0 {
        object.key("filterStrength").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.filter_strength).into()),
        );
    }
    if let Some(var_25) = &input.image_inserter {
        #[allow(unused_mut)]
        let mut object_26 = object.key("imageInserter").start_object();
        crate::protocol_serde::shape_image_inserter::ser_image_inserter(&mut object_26, var_25)?;
        object_26.finish();
    }
    if let Some(var_27) = &input.input_clippings {
        let mut array_28 = object.key("inputClippings").start_array();
        for item_29 in var_27 {
            {
                #[allow(unused_mut)]
                let mut object_30 = array_28.value().start_object();
                crate::protocol_serde::shape_input_clipping::ser_input_clipping(
                    &mut object_30,
                    item_29,
                )?;
                object_30.finish();
            }
        }
        array_28.finish();
    }
    if let Some(var_31) = &input.input_scan_type {
        object.key("inputScanType").string(var_31.as_str());
    }
    if let Some(var_32) = &input.position {
        #[allow(unused_mut)]
        let mut object_33 = object.key("position").start_object();
        crate::protocol_serde::shape_rectangle::ser_rectangle(&mut object_33, var_32)?;
        object_33.finish();
    }
    if input.program_number != 0 {
        object.key("programNumber").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.program_number).into()),
        );
    }
    if let Some(var_34) = &input.psi_control {
        object.key("psiControl").string(var_34.as_str());
    }
    if let Some(var_35) = &input.supplemental_imps {
        let mut array_36 = object.key("supplementalImps").start_array();
        for item_37 in var_35 {
            {
                array_36.value().string(item_37.as_str());
            }
        }
        array_36.finish();
    }
    if let Some(var_38) = &input.timecode_source {
        object.key("timecodeSource").string(var_38.as_str());
    }
    if let Some(var_39) = &input.timecode_start {
        object.key("timecodeStart").string(var_39.as_str());
    }
    if let Some(var_40) = &input.video_generator {
        #[allow(unused_mut)]
        let mut object_41 = object.key("videoGenerator").start_object();
        crate::protocol_serde::shape_input_video_generator::ser_input_video_generator(
            &mut object_41,
            var_40,
        )?;
        object_41.finish();
    }
    if let Some(var_42) = &input.video_selector {
        #[allow(unused_mut)]
        let mut object_43 = object.key("videoSelector").start_object();
        crate::protocol_serde::shape_video_selector::ser_video_selector(&mut object_43, var_42)?;
        object_43.finish();
    }
    Ok(())
}

pub(crate) fn de_input<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::types::Input>, aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<
        Item = Result<
            aws_smithy_json::deserialize::Token<'a>,
            aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::InputBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "audioSelectorGroups" => {
                                builder = builder.set_audio_selector_groups(
                                    crate::protocol_serde::shape___map_of_audio_selector_group::de___map_of_audio_selector_group(tokens)?
                                );
                            }
                            "audioSelectors" => {
                                builder = builder.set_audio_selectors(
                                    crate::protocol_serde::shape___map_of_audio_selector::de___map_of_audio_selector(tokens)?
                                );
                            }
                            "captionSelectors" => {
                                builder = builder.set_caption_selectors(
                                    crate::protocol_serde::shape___map_of_caption_selector::de___map_of_caption_selector(tokens)?
                                );
                            }
                            "crop" => {
                                builder = builder.set_crop(
                                    crate::protocol_serde::shape_rectangle::de_rectangle(tokens)?,
                                );
                            }
                            "deblockFilter" => {
                                builder = builder.set_deblock_filter(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::InputDeblockFilter::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "decryptionSettings" => {
                                builder = builder.set_decryption_settings(
                                    crate::protocol_serde::shape_input_decryption_settings::de_input_decryption_settings(tokens)?
                                );
                            }
                            "denoiseFilter" => {
                                builder = builder.set_denoise_filter(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::InputDenoiseFilter::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "dolbyVisionMetadataXml" => {
                                builder = builder.set_dolby_vision_metadata_xml(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "fileInput" => {
                                builder = builder.set_file_input(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "filterEnable" => {
                                builder = builder.set_filter_enable(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::InputFilterEnable::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "filterStrength" => {
                                builder = builder.set_filter_strength(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "imageInserter" => {
                                builder = builder.set_image_inserter(
                                    crate::protocol_serde::shape_image_inserter::de_image_inserter(
                                        tokens,
                                    )?,
                                );
                            }
                            "inputClippings" => {
                                builder = builder.set_input_clippings(
                                    crate::protocol_serde::shape___list_of_input_clipping::de___list_of_input_clipping(tokens)?
                                );
                            }
                            "inputScanType" => {
                                builder = builder.set_input_scan_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::InputScanType::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "position" => {
                                builder = builder.set_position(
                                    crate::protocol_serde::shape_rectangle::de_rectangle(tokens)?,
                                );
                            }
                            "programNumber" => {
                                builder = builder.set_program_number(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "psiControl" => {
                                builder = builder.set_psi_control(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::InputPsiControl::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "supplementalImps" => {
                                builder = builder.set_supplemental_imps(
                                    crate::protocol_serde::shape___list_of__string_pattern_s3_assetmap_xml::de___list_of__string_pattern_s3_assetmap_xml(tokens)?
                                );
                            }
                            "timecodeSource" => {
                                builder = builder.set_timecode_source(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::InputTimecodeSource::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "timecodeStart" => {
                                builder = builder.set_timecode_start(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "videoGenerator" => {
                                builder = builder.set_video_generator(
                                    crate::protocol_serde::shape_input_video_generator::de_input_video_generator(tokens)?
                                );
                            }
                            "videoSelector" => {
                                builder = builder.set_video_selector(
                                    crate::protocol_serde::shape_video_selector::de_video_selector(
                                        tokens,
                                    )?,
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
            Ok(Some(builder.build()))
        }
        _ => Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ),
        ),
    }
}
