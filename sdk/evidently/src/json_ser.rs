// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_evaluate_feature_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchEvaluateFeatureInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.requests {
        let mut array_2 = object.key("requests").start_array();
        for item_3 in var_1 {
            {
                let mut object_4 = array_2.value().start_object();
                crate::json_ser::serialize_structure_crate_model_evaluation_request(
                    &mut object_4,
                    item_3,
                )?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_experiment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateExperimentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_5) = &input.description {
        object.key("description").string(var_5);
    }
    if let Some(var_6) = &input.metric_goals {
        let mut array_7 = object.key("metricGoals").start_array();
        for item_8 in var_6 {
            {
                let mut object_9 = array_7.value().start_object();
                crate::json_ser::serialize_structure_crate_model_metric_goal_config(
                    &mut object_9,
                    item_8,
                )?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.name {
        object.key("name").string(var_10);
    }
    if let Some(var_11) = &input.online_ab_config {
        let mut object_12 = object.key("onlineAbConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_online_ab_config(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.randomization_salt {
        object.key("randomizationSalt").string(var_13);
    }
    if let Some(var_14) = &input.sampling_rate {
        object.key("samplingRate").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    if let Some(var_15) = &input.tags {
        let mut object_16 = object.key("tags").start_object();
        for (key_17, value_18) in var_15 {
            {
                object_16.key(key_17).string(value_18);
            }
        }
        object_16.finish();
    }
    if let Some(var_19) = &input.treatments {
        let mut array_20 = object.key("treatments").start_array();
        for item_21 in var_19 {
            {
                let mut object_22 = array_20.value().start_object();
                crate::json_ser::serialize_structure_crate_model_treatment_config(
                    &mut object_22,
                    item_21,
                )?;
                object_22.finish();
            }
        }
        array_20.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_feature_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateFeatureInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_23) = &input.default_variation {
        object.key("defaultVariation").string(var_23);
    }
    if let Some(var_24) = &input.description {
        object.key("description").string(var_24);
    }
    if let Some(var_25) = &input.entity_overrides {
        let mut object_26 = object.key("entityOverrides").start_object();
        for (key_27, value_28) in var_25 {
            {
                object_26.key(key_27).string(value_28);
            }
        }
        object_26.finish();
    }
    if let Some(var_29) = &input.evaluation_strategy {
        object.key("evaluationStrategy").string(var_29.as_str());
    }
    if let Some(var_30) = &input.name {
        object.key("name").string(var_30);
    }
    if let Some(var_31) = &input.tags {
        let mut object_32 = object.key("tags").start_object();
        for (key_33, value_34) in var_31 {
            {
                object_32.key(key_33).string(value_34);
            }
        }
        object_32.finish();
    }
    if let Some(var_35) = &input.variations {
        let mut array_36 = object.key("variations").start_array();
        for item_37 in var_35 {
            {
                let mut object_38 = array_36.value().start_object();
                crate::json_ser::serialize_structure_crate_model_variation_config(
                    &mut object_38,
                    item_37,
                )?;
                object_38.finish();
            }
        }
        array_36.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_launch_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLaunchInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_39) = &input.description {
        object.key("description").string(var_39);
    }
    if let Some(var_40) = &input.groups {
        let mut array_41 = object.key("groups").start_array();
        for item_42 in var_40 {
            {
                let mut object_43 = array_41.value().start_object();
                crate::json_ser::serialize_structure_crate_model_launch_group_config(
                    &mut object_43,
                    item_42,
                )?;
                object_43.finish();
            }
        }
        array_41.finish();
    }
    if let Some(var_44) = &input.metric_monitors {
        let mut array_45 = object.key("metricMonitors").start_array();
        for item_46 in var_44 {
            {
                let mut object_47 = array_45.value().start_object();
                crate::json_ser::serialize_structure_crate_model_metric_monitor_config(
                    &mut object_47,
                    item_46,
                )?;
                object_47.finish();
            }
        }
        array_45.finish();
    }
    if let Some(var_48) = &input.name {
        object.key("name").string(var_48);
    }
    if let Some(var_49) = &input.randomization_salt {
        object.key("randomizationSalt").string(var_49);
    }
    if let Some(var_50) = &input.scheduled_splits_config {
        let mut object_51 = object.key("scheduledSplitsConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_scheduled_splits_launch_config(
            &mut object_51,
            var_50,
        )?;
        object_51.finish();
    }
    if let Some(var_52) = &input.tags {
        let mut object_53 = object.key("tags").start_object();
        for (key_54, value_55) in var_52 {
            {
                object_53.key(key_54).string(value_55);
            }
        }
        object_53.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_project_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateProjectInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.data_delivery {
        let mut object_57 = object.key("dataDelivery").start_object();
        crate::json_ser::serialize_structure_crate_model_project_data_delivery_config(
            &mut object_57,
            var_56,
        )?;
        object_57.finish();
    }
    if let Some(var_58) = &input.description {
        object.key("description").string(var_58);
    }
    if let Some(var_59) = &input.name {
        object.key("name").string(var_59);
    }
    if let Some(var_60) = &input.tags {
        let mut object_61 = object.key("tags").start_object();
        for (key_62, value_63) in var_60 {
            {
                object_61.key(key_62).string(value_63);
            }
        }
        object_61.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_evaluate_feature_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::EvaluateFeatureInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_64) = &input.entity_id {
        object.key("entityId").string(var_64);
    }
    if let Some(var_65) = &input.evaluation_context {
        object.key("evaluationContext").string(var_65);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_experiment_results_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetExperimentResultsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.base_stat {
        object.key("baseStat").string(var_66.as_str());
    }
    if let Some(var_67) = &input.end_time {
        object
            .key("endTime")
            .date_time(var_67, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_68) = &input.metric_names {
        let mut array_69 = object.key("metricNames").start_array();
        for item_70 in var_68 {
            {
                array_69.value().string(item_70);
            }
        }
        array_69.finish();
    }
    if input.period != 0 {
        object.key("period").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.period).into()),
        );
    }
    if let Some(var_71) = &input.report_names {
        let mut array_72 = object.key("reportNames").start_array();
        for item_73 in var_71 {
            {
                array_72.value().string(item_73.as_str());
            }
        }
        array_72.finish();
    }
    if let Some(var_74) = &input.result_stats {
        let mut array_75 = object.key("resultStats").start_array();
        for item_76 in var_74 {
            {
                array_75.value().string(item_76.as_str());
            }
        }
        array_75.finish();
    }
    if let Some(var_77) = &input.start_time {
        object
            .key("startTime")
            .date_time(var_77, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_78) = &input.treatment_names {
        let mut array_79 = object.key("treatmentNames").start_array();
        for item_80 in var_78 {
            {
                array_79.value().string(item_80);
            }
        }
        array_79.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_project_events_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutProjectEventsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_81) = &input.events {
        let mut array_82 = object.key("events").start_array();
        for item_83 in var_81 {
            {
                let mut object_84 = array_82.value().start_object();
                crate::json_ser::serialize_structure_crate_model_event(&mut object_84, item_83)?;
                object_84.finish();
            }
        }
        array_82.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_experiment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartExperimentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_85) = &input.analysis_complete_time {
        object
            .key("analysisCompleteTime")
            .date_time(var_85, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_experiment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopExperimentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_86) = &input.desired_state {
        object.key("desiredState").string(var_86.as_str());
    }
    if let Some(var_87) = &input.reason {
        object.key("reason").string(var_87);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_launch_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopLaunchInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_88) = &input.desired_state {
        object.key("desiredState").string(var_88.as_str());
    }
    if let Some(var_89) = &input.reason {
        object.key("reason").string(var_89);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_90) = &input.tags {
        let mut object_91 = object.key("tags").start_object();
        for (key_92, value_93) in var_90 {
            {
                object_91.key(key_92).string(value_93);
            }
        }
        object_91.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_experiment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateExperimentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_94) = &input.description {
        object.key("description").string(var_94);
    }
    if let Some(var_95) = &input.metric_goals {
        let mut array_96 = object.key("metricGoals").start_array();
        for item_97 in var_95 {
            {
                let mut object_98 = array_96.value().start_object();
                crate::json_ser::serialize_structure_crate_model_metric_goal_config(
                    &mut object_98,
                    item_97,
                )?;
                object_98.finish();
            }
        }
        array_96.finish();
    }
    if let Some(var_99) = &input.online_ab_config {
        let mut object_100 = object.key("onlineAbConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_online_ab_config(&mut object_100, var_99)?;
        object_100.finish();
    }
    if let Some(var_101) = &input.randomization_salt {
        object.key("randomizationSalt").string(var_101);
    }
    if let Some(var_102) = &input.sampling_rate {
        object.key("samplingRate").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_102).into()),
        );
    }
    if let Some(var_103) = &input.treatments {
        let mut array_104 = object.key("treatments").start_array();
        for item_105 in var_103 {
            {
                let mut object_106 = array_104.value().start_object();
                crate::json_ser::serialize_structure_crate_model_treatment_config(
                    &mut object_106,
                    item_105,
                )?;
                object_106.finish();
            }
        }
        array_104.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_feature_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateFeatureInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_107) = &input.add_or_update_variations {
        let mut array_108 = object.key("addOrUpdateVariations").start_array();
        for item_109 in var_107 {
            {
                let mut object_110 = array_108.value().start_object();
                crate::json_ser::serialize_structure_crate_model_variation_config(
                    &mut object_110,
                    item_109,
                )?;
                object_110.finish();
            }
        }
        array_108.finish();
    }
    if let Some(var_111) = &input.default_variation {
        object.key("defaultVariation").string(var_111);
    }
    if let Some(var_112) = &input.description {
        object.key("description").string(var_112);
    }
    if let Some(var_113) = &input.entity_overrides {
        let mut object_114 = object.key("entityOverrides").start_object();
        for (key_115, value_116) in var_113 {
            {
                object_114.key(key_115).string(value_116);
            }
        }
        object_114.finish();
    }
    if let Some(var_117) = &input.evaluation_strategy {
        object.key("evaluationStrategy").string(var_117.as_str());
    }
    if let Some(var_118) = &input.remove_variations {
        let mut array_119 = object.key("removeVariations").start_array();
        for item_120 in var_118 {
            {
                array_119.value().string(item_120);
            }
        }
        array_119.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_launch_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLaunchInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_121) = &input.description {
        object.key("description").string(var_121);
    }
    if let Some(var_122) = &input.groups {
        let mut array_123 = object.key("groups").start_array();
        for item_124 in var_122 {
            {
                let mut object_125 = array_123.value().start_object();
                crate::json_ser::serialize_structure_crate_model_launch_group_config(
                    &mut object_125,
                    item_124,
                )?;
                object_125.finish();
            }
        }
        array_123.finish();
    }
    if let Some(var_126) = &input.metric_monitors {
        let mut array_127 = object.key("metricMonitors").start_array();
        for item_128 in var_126 {
            {
                let mut object_129 = array_127.value().start_object();
                crate::json_ser::serialize_structure_crate_model_metric_monitor_config(
                    &mut object_129,
                    item_128,
                )?;
                object_129.finish();
            }
        }
        array_127.finish();
    }
    if let Some(var_130) = &input.randomization_salt {
        object.key("randomizationSalt").string(var_130);
    }
    if let Some(var_131) = &input.scheduled_splits_config {
        let mut object_132 = object.key("scheduledSplitsConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_scheduled_splits_launch_config(
            &mut object_132,
            var_131,
        )?;
        object_132.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_project_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateProjectInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_133) = &input.description {
        object.key("description").string(var_133);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_project_data_delivery_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateProjectDataDeliveryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_134) = &input.cloud_watch_logs {
        let mut object_135 = object.key("cloudWatchLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_cloud_watch_logs_destination_config(
            &mut object_135,
            var_134,
        )?;
        object_135.finish();
    }
    if let Some(var_136) = &input.s3_destination {
        let mut object_137 = object.key("s3Destination").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_destination_config(
            &mut object_137,
            var_136,
        )?;
        object_137.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_evaluation_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EvaluationRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_138) = &input.feature {
        object.key("feature").string(var_138);
    }
    if let Some(var_139) = &input.entity_id {
        object.key("entityId").string(var_139);
    }
    if let Some(var_140) = &input.evaluation_context {
        object.key("evaluationContext").string(var_140);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_metric_goal_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MetricGoalConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_141) = &input.metric_definition {
        let mut object_142 = object.key("metricDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_metric_definition_config(
            &mut object_142,
            var_141,
        )?;
        object_142.finish();
    }
    if let Some(var_143) = &input.desired_change {
        object.key("desiredChange").string(var_143.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_online_ab_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OnlineAbConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_144) = &input.control_treatment_name {
        object.key("controlTreatmentName").string(var_144);
    }
    if let Some(var_145) = &input.treatment_weights {
        let mut object_146 = object.key("treatmentWeights").start_object();
        for (key_147, value_148) in var_145 {
            {
                object_146.key(key_147).number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::NegInt((*value_148).into()),
                );
            }
        }
        object_146.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_treatment_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TreatmentConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_149) = &input.name {
        object.key("name").string(var_149);
    }
    if let Some(var_150) = &input.description {
        object.key("description").string(var_150);
    }
    if let Some(var_151) = &input.feature {
        object.key("feature").string(var_151);
    }
    if let Some(var_152) = &input.variation {
        object.key("variation").string(var_152);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_variation_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VariationConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_153) = &input.name {
        object.key("name").string(var_153);
    }
    if let Some(var_154) = &input.value {
        let mut object_155 = object.key("value").start_object();
        crate::json_ser::serialize_union_crate_model_variable_value(&mut object_155, var_154)?;
        object_155.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_launch_group_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LaunchGroupConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_156) = &input.name {
        object.key("name").string(var_156);
    }
    if let Some(var_157) = &input.description {
        object.key("description").string(var_157);
    }
    if let Some(var_158) = &input.feature {
        object.key("feature").string(var_158);
    }
    if let Some(var_159) = &input.variation {
        object.key("variation").string(var_159);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_metric_monitor_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MetricMonitorConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_160) = &input.metric_definition {
        let mut object_161 = object.key("metricDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_metric_definition_config(
            &mut object_161,
            var_160,
        )?;
        object_161.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_scheduled_splits_launch_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ScheduledSplitsLaunchConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_162) = &input.steps {
        let mut array_163 = object.key("steps").start_array();
        for item_164 in var_162 {
            {
                let mut object_165 = array_163.value().start_object();
                crate::json_ser::serialize_structure_crate_model_scheduled_split_config(
                    &mut object_165,
                    item_164,
                )?;
                object_165.finish();
            }
        }
        array_163.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_project_data_delivery_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ProjectDataDeliveryConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_166) = &input.s3_destination {
        let mut object_167 = object.key("s3Destination").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_destination_config(
            &mut object_167,
            var_166,
        )?;
        object_167.finish();
    }
    if let Some(var_168) = &input.cloud_watch_logs {
        let mut object_169 = object.key("cloudWatchLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_cloud_watch_logs_destination_config(
            &mut object_169,
            var_168,
        )?;
        object_169.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_event(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Event,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_170) = &input.timestamp {
        object
            .key("timestamp")
            .date_time(var_170, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_171) = &input.r#type {
        object.key("type").string(var_171.as_str());
    }
    if let Some(var_172) = &input.data {
        object.key("data").string(var_172);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cloud_watch_logs_destination_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CloudWatchLogsDestinationConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_173) = &input.log_group {
        object.key("logGroup").string(var_173);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_destination_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3DestinationConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_174) = &input.bucket {
        object.key("bucket").string(var_174);
    }
    if let Some(var_175) = &input.prefix {
        object.key("prefix").string(var_175);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_metric_definition_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MetricDefinitionConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_176) = &input.name {
        object.key("name").string(var_176);
    }
    if let Some(var_177) = &input.entity_id_key {
        object.key("entityIdKey").string(var_177);
    }
    if let Some(var_178) = &input.value_key {
        object.key("valueKey").string(var_178);
    }
    if let Some(var_179) = &input.event_pattern {
        object.key("eventPattern").string(var_179);
    }
    if let Some(var_180) = &input.unit_label {
        object.key("unitLabel").string(var_180);
    }
    Ok(())
}

pub fn serialize_union_crate_model_variable_value(
    object_155: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VariableValue,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::VariableValue::BoolValue(inner) => {
            object_155.key("boolValue").boolean(*inner);
        }
        crate::model::VariableValue::StringValue(inner) => {
            object_155.key("stringValue").string(inner);
        }
        crate::model::VariableValue::LongValue(inner) => {
            object_155.key("longValue").number(
                #[allow(clippy::useless_conversion)]
                aws_smithy_types::Number::NegInt((*inner).into()),
            );
        }
        crate::model::VariableValue::DoubleValue(inner) => {
            object_155.key("doubleValue").number(
                #[allow(clippy::useless_conversion)]
                aws_smithy_types::Number::Float((*inner).into()),
            );
        }
        crate::model::VariableValue::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant("VariableValue"),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_scheduled_split_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ScheduledSplitConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_181) = &input.start_time {
        object
            .key("startTime")
            .date_time(var_181, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_182) = &input.group_weights {
        let mut object_183 = object.key("groupWeights").start_object();
        for (key_184, value_185) in var_182 {
            {
                object_183.key(key_184).number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::NegInt((*value_185).into()),
                );
            }
        }
        object_183.finish();
    }
    Ok(())
}
