// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_associate_scram_secret_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchAssociateScramSecretInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.secret_arn_list {
        let mut array_2 = object.key("secretArnList").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3);
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_disassociate_scram_secret_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchDisassociateScramSecretInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_4) = &input.secret_arn_list {
        let mut array_5 = object.key("secretArnList").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6);
            }
        }
        array_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateClusterInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_7) = &input.broker_node_group_info {
        let mut object_8 = object.key("brokerNodeGroupInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_broker_node_group_info(
            &mut object_8,
            var_7,
        )?;
        object_8.finish();
    }
    if let Some(var_9) = &input.client_authentication {
        let mut object_10 = object.key("clientAuthentication").start_object();
        crate::json_ser::serialize_structure_crate_model_client_authentication(
            &mut object_10,
            var_9,
        )?;
        object_10.finish();
    }
    if let Some(var_11) = &input.cluster_name {
        object.key("clusterName").string(var_11);
    }
    if let Some(var_12) = &input.configuration_info {
        let mut object_13 = object.key("configurationInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_configuration_info(
            &mut object_13,
            var_12,
        )?;
        object_13.finish();
    }
    if let Some(var_14) = &input.encryption_info {
        let mut object_15 = object.key("encryptionInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_info(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.enhanced_monitoring {
        object.key("enhancedMonitoring").string(var_16.as_str());
    }
    if let Some(var_17) = &input.kafka_version {
        object.key("kafkaVersion").string(var_17);
    }
    if let Some(var_18) = &input.logging_info {
        let mut object_19 = object.key("loggingInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_logging_info(&mut object_19, var_18)?;
        object_19.finish();
    }
    {
        object.key("numberOfBrokerNodes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.number_of_broker_nodes).into()),
        );
    }
    if let Some(var_20) = &input.open_monitoring {
        let mut object_21 = object.key("openMonitoring").start_object();
        crate::json_ser::serialize_structure_crate_model_open_monitoring_info(
            &mut object_21,
            var_20,
        )?;
        object_21.finish();
    }
    if let Some(var_22) = &input.tags {
        let mut object_23 = object.key("tags").start_object();
        for (key_24, value_25) in var_22 {
            {
                object_23.key(key_24).string(value_25);
            }
        }
        object_23.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_cluster_v2_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateClusterV2Input,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_26) = &input.cluster_name {
        object.key("clusterName").string(var_26);
    }
    if let Some(var_27) = &input.provisioned {
        let mut object_28 = object.key("provisioned").start_object();
        crate::json_ser::serialize_structure_crate_model_provisioned_request(
            &mut object_28,
            var_27,
        )?;
        object_28.finish();
    }
    if let Some(var_29) = &input.serverless {
        let mut object_30 = object.key("serverless").start_object();
        crate::json_ser::serialize_structure_crate_model_serverless_request(
            &mut object_30,
            var_29,
        )?;
        object_30.finish();
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
    Ok(())
}

pub fn serialize_structure_crate_input_create_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_35) = &input.description {
        object.key("description").string(var_35);
    }
    if let Some(var_36) = &input.kafka_versions {
        let mut array_37 = object.key("kafkaVersions").start_array();
        for item_38 in var_36 {
            {
                array_37.value().string(item_38);
            }
        }
        array_37.finish();
    }
    if let Some(var_39) = &input.name {
        object.key("name").string(var_39);
    }
    if let Some(var_40) = &input.server_properties {
        object
            .key("serverProperties")
            .string_unchecked(&aws_smithy_types::base64::encode(var_40));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_reboot_broker_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RebootBrokerInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_41) = &input.broker_ids {
        let mut array_42 = object.key("brokerIds").start_array();
        for item_43 in var_41 {
            {
                array_42.value().string(item_43);
            }
        }
        array_42.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.tags {
        let mut object_45 = object.key("tags").start_object();
        for (key_46, value_47) in var_44 {
            {
                object_45.key(key_46).string(value_47);
            }
        }
        object_45.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_broker_count_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBrokerCountInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_48) = &input.current_version {
        object.key("currentVersion").string(var_48);
    }
    {
        object.key("targetNumberOfBrokerNodes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.target_number_of_broker_nodes).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_broker_storage_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBrokerStorageInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.current_version {
        object.key("currentVersion").string(var_49);
    }
    if let Some(var_50) = &input.target_broker_ebs_volume_info {
        let mut array_51 = object.key("targetBrokerEBSVolumeInfo").start_array();
        for item_52 in var_50 {
            {
                let mut object_53 = array_51.value().start_object();
                crate::json_ser::serialize_structure_crate_model_broker_ebs_volume_info(
                    &mut object_53,
                    item_52,
                )?;
                object_53.finish();
            }
        }
        array_51.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_broker_type_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBrokerTypeInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.current_version {
        object.key("currentVersion").string(var_54);
    }
    if let Some(var_55) = &input.target_instance_type {
        object.key("targetInstanceType").string(var_55);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_cluster_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateClusterConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.configuration_info {
        let mut object_57 = object.key("configurationInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_configuration_info(
            &mut object_57,
            var_56,
        )?;
        object_57.finish();
    }
    if let Some(var_58) = &input.current_version {
        object.key("currentVersion").string(var_58);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_cluster_kafka_version_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateClusterKafkaVersionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.configuration_info {
        let mut object_60 = object.key("configurationInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_configuration_info(
            &mut object_60,
            var_59,
        )?;
        object_60.finish();
    }
    if let Some(var_61) = &input.current_version {
        object.key("currentVersion").string(var_61);
    }
    if let Some(var_62) = &input.target_kafka_version {
        object.key("targetKafkaVersion").string(var_62);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_63) = &input.description {
        object.key("description").string(var_63);
    }
    if let Some(var_64) = &input.server_properties {
        object
            .key("serverProperties")
            .string_unchecked(&aws_smithy_types::base64::encode(var_64));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_connectivity_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateConnectivityInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_65) = &input.connectivity_info {
        let mut object_66 = object.key("connectivityInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_connectivity_info(&mut object_66, var_65)?;
        object_66.finish();
    }
    if let Some(var_67) = &input.current_version {
        object.key("currentVersion").string(var_67);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_monitoring_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateMonitoringInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_68) = &input.current_version {
        object.key("currentVersion").string(var_68);
    }
    if let Some(var_69) = &input.enhanced_monitoring {
        object.key("enhancedMonitoring").string(var_69.as_str());
    }
    if let Some(var_70) = &input.logging_info {
        let mut object_71 = object.key("loggingInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_logging_info(&mut object_71, var_70)?;
        object_71.finish();
    }
    if let Some(var_72) = &input.open_monitoring {
        let mut object_73 = object.key("openMonitoring").start_object();
        crate::json_ser::serialize_structure_crate_model_open_monitoring_info(
            &mut object_73,
            var_72,
        )?;
        object_73.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_security_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSecurityInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_74) = &input.client_authentication {
        let mut object_75 = object.key("clientAuthentication").start_object();
        crate::json_ser::serialize_structure_crate_model_client_authentication(
            &mut object_75,
            var_74,
        )?;
        object_75.finish();
    }
    if let Some(var_76) = &input.current_version {
        object.key("currentVersion").string(var_76);
    }
    if let Some(var_77) = &input.encryption_info {
        let mut object_78 = object.key("encryptionInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_info(&mut object_78, var_77)?;
        object_78.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_broker_node_group_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BrokerNodeGroupInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.broker_az_distribution {
        object.key("brokerAZDistribution").string(var_79.as_str());
    }
    if let Some(var_80) = &input.client_subnets {
        let mut array_81 = object.key("clientSubnets").start_array();
        for item_82 in var_80 {
            {
                array_81.value().string(item_82);
            }
        }
        array_81.finish();
    }
    if let Some(var_83) = &input.instance_type {
        object.key("instanceType").string(var_83);
    }
    if let Some(var_84) = &input.security_groups {
        let mut array_85 = object.key("securityGroups").start_array();
        for item_86 in var_84 {
            {
                array_85.value().string(item_86);
            }
        }
        array_85.finish();
    }
    if let Some(var_87) = &input.storage_info {
        let mut object_88 = object.key("storageInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_storage_info(&mut object_88, var_87)?;
        object_88.finish();
    }
    if let Some(var_89) = &input.connectivity_info {
        let mut object_90 = object.key("connectivityInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_connectivity_info(&mut object_90, var_89)?;
        object_90.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_client_authentication(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ClientAuthentication,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_91) = &input.sasl {
        let mut object_92 = object.key("sasl").start_object();
        crate::json_ser::serialize_structure_crate_model_sasl(&mut object_92, var_91)?;
        object_92.finish();
    }
    if let Some(var_93) = &input.tls {
        let mut object_94 = object.key("tls").start_object();
        crate::json_ser::serialize_structure_crate_model_tls(&mut object_94, var_93)?;
        object_94.finish();
    }
    if let Some(var_95) = &input.unauthenticated {
        let mut object_96 = object.key("unauthenticated").start_object();
        crate::json_ser::serialize_structure_crate_model_unauthenticated(&mut object_96, var_95)?;
        object_96.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_configuration_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ConfigurationInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_97) = &input.arn {
        object.key("arn").string(var_97);
    }
    {
        object.key("revision").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.revision).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_encryption_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EncryptionInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_98) = &input.encryption_at_rest {
        let mut object_99 = object.key("encryptionAtRest").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_at_rest(
            &mut object_99,
            var_98,
        )?;
        object_99.finish();
    }
    if let Some(var_100) = &input.encryption_in_transit {
        let mut object_101 = object.key("encryptionInTransit").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_in_transit(
            &mut object_101,
            var_100,
        )?;
        object_101.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_logging_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LoggingInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_102) = &input.broker_logs {
        let mut object_103 = object.key("brokerLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_broker_logs(&mut object_103, var_102)?;
        object_103.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_open_monitoring_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OpenMonitoringInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_104) = &input.prometheus {
        let mut object_105 = object.key("prometheus").start_object();
        crate::json_ser::serialize_structure_crate_model_prometheus_info(&mut object_105, var_104)?;
        object_105.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_provisioned_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ProvisionedRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_106) = &input.broker_node_group_info {
        let mut object_107 = object.key("brokerNodeGroupInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_broker_node_group_info(
            &mut object_107,
            var_106,
        )?;
        object_107.finish();
    }
    if let Some(var_108) = &input.client_authentication {
        let mut object_109 = object.key("clientAuthentication").start_object();
        crate::json_ser::serialize_structure_crate_model_client_authentication(
            &mut object_109,
            var_108,
        )?;
        object_109.finish();
    }
    if let Some(var_110) = &input.configuration_info {
        let mut object_111 = object.key("configurationInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_configuration_info(
            &mut object_111,
            var_110,
        )?;
        object_111.finish();
    }
    if let Some(var_112) = &input.encryption_info {
        let mut object_113 = object.key("encryptionInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_info(&mut object_113, var_112)?;
        object_113.finish();
    }
    if let Some(var_114) = &input.enhanced_monitoring {
        object.key("enhancedMonitoring").string(var_114.as_str());
    }
    if let Some(var_115) = &input.open_monitoring {
        let mut object_116 = object.key("openMonitoring").start_object();
        crate::json_ser::serialize_structure_crate_model_open_monitoring_info(
            &mut object_116,
            var_115,
        )?;
        object_116.finish();
    }
    if let Some(var_117) = &input.kafka_version {
        object.key("kafkaVersion").string(var_117);
    }
    if let Some(var_118) = &input.logging_info {
        let mut object_119 = object.key("loggingInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_logging_info(&mut object_119, var_118)?;
        object_119.finish();
    }
    {
        object.key("numberOfBrokerNodes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.number_of_broker_nodes).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_serverless_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ServerlessRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_120) = &input.vpc_configs {
        let mut array_121 = object.key("vpcConfigs").start_array();
        for item_122 in var_120 {
            {
                let mut object_123 = array_121.value().start_object();
                crate::json_ser::serialize_structure_crate_model_vpc_config(
                    &mut object_123,
                    item_122,
                )?;
                object_123.finish();
            }
        }
        array_121.finish();
    }
    if let Some(var_124) = &input.client_authentication {
        let mut object_125 = object.key("clientAuthentication").start_object();
        crate::json_ser::serialize_structure_crate_model_serverless_client_authentication(
            &mut object_125,
            var_124,
        )?;
        object_125.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_broker_ebs_volume_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BrokerEbsVolumeInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_126) = &input.kafka_broker_node_id {
        object.key("kafkaBrokerNodeId").string(var_126);
    }
    {
        object.key("volumeSizeGB").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.volume_size_gb).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_connectivity_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ConnectivityInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_127) = &input.public_access {
        let mut object_128 = object.key("publicAccess").start_object();
        crate::json_ser::serialize_structure_crate_model_public_access(&mut object_128, var_127)?;
        object_128.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_storage_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StorageInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_129) = &input.ebs_storage_info {
        let mut object_130 = object.key("ebsStorageInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_ebs_storage_info(
            &mut object_130,
            var_129,
        )?;
        object_130.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_sasl(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Sasl,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_131) = &input.scram {
        let mut object_132 = object.key("scram").start_object();
        crate::json_ser::serialize_structure_crate_model_scram(&mut object_132, var_131)?;
        object_132.finish();
    }
    if let Some(var_133) = &input.iam {
        let mut object_134 = object.key("iam").start_object();
        crate::json_ser::serialize_structure_crate_model_iam(&mut object_134, var_133)?;
        object_134.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tls(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tls,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_135) = &input.certificate_authority_arn_list {
        let mut array_136 = object.key("certificateAuthorityArnList").start_array();
        for item_137 in var_135 {
            {
                array_136.value().string(item_137);
            }
        }
        array_136.finish();
    }
    if input.enabled {
        object.key("enabled").boolean(input.enabled);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_unauthenticated(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Unauthenticated,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.enabled {
        object.key("enabled").boolean(input.enabled);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_encryption_at_rest(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EncryptionAtRest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_138) = &input.data_volume_kms_key_id {
        object.key("dataVolumeKMSKeyId").string(var_138);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_encryption_in_transit(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EncryptionInTransit,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_139) = &input.client_broker {
        object.key("clientBroker").string(var_139.as_str());
    }
    if input.in_cluster {
        object.key("inCluster").boolean(input.in_cluster);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_broker_logs(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BrokerLogs,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_140) = &input.cloud_watch_logs {
        let mut object_141 = object.key("cloudWatchLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_cloud_watch_logs(
            &mut object_141,
            var_140,
        )?;
        object_141.finish();
    }
    if let Some(var_142) = &input.firehose {
        let mut object_143 = object.key("firehose").start_object();
        crate::json_ser::serialize_structure_crate_model_firehose(&mut object_143, var_142)?;
        object_143.finish();
    }
    if let Some(var_144) = &input.s3 {
        let mut object_145 = object.key("s3").start_object();
        crate::json_ser::serialize_structure_crate_model_s3(&mut object_145, var_144)?;
        object_145.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_prometheus_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PrometheusInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_146) = &input.jmx_exporter {
        let mut object_147 = object.key("jmxExporter").start_object();
        crate::json_ser::serialize_structure_crate_model_jmx_exporter_info(
            &mut object_147,
            var_146,
        )?;
        object_147.finish();
    }
    if let Some(var_148) = &input.node_exporter {
        let mut object_149 = object.key("nodeExporter").start_object();
        crate::json_ser::serialize_structure_crate_model_node_exporter_info(
            &mut object_149,
            var_148,
        )?;
        object_149.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_vpc_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VpcConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_150) = &input.subnet_ids {
        let mut array_151 = object.key("subnetIds").start_array();
        for item_152 in var_150 {
            {
                array_151.value().string(item_152);
            }
        }
        array_151.finish();
    }
    if let Some(var_153) = &input.security_group_ids {
        let mut array_154 = object.key("securityGroupIds").start_array();
        for item_155 in var_153 {
            {
                array_154.value().string(item_155);
            }
        }
        array_154.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_serverless_client_authentication(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ServerlessClientAuthentication,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_156) = &input.sasl {
        let mut object_157 = object.key("sasl").start_object();
        crate::json_ser::serialize_structure_crate_model_serverless_sasl(&mut object_157, var_156)?;
        object_157.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_public_access(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PublicAccess,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_158) = &input.r#type {
        object.key("type").string(var_158);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ebs_storage_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EbsStorageInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.volume_size != 0 {
        object.key("volumeSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.volume_size).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_scram(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Scram,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.enabled {
        object.key("enabled").boolean(input.enabled);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_iam(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Iam,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.enabled {
        object.key("enabled").boolean(input.enabled);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cloud_watch_logs(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CloudWatchLogs,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    {
        object.key("enabled").boolean(input.enabled);
    }
    if let Some(var_159) = &input.log_group {
        object.key("logGroup").string(var_159);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_firehose(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Firehose,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_160) = &input.delivery_stream {
        object.key("deliveryStream").string(var_160);
    }
    {
        object.key("enabled").boolean(input.enabled);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_161) = &input.bucket {
        object.key("bucket").string(var_161);
    }
    {
        object.key("enabled").boolean(input.enabled);
    }
    if let Some(var_162) = &input.prefix {
        object.key("prefix").string(var_162);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_jmx_exporter_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::JmxExporterInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    {
        object
            .key("enabledInBroker")
            .boolean(input.enabled_in_broker);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_node_exporter_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NodeExporterInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    {
        object
            .key("enabledInBroker")
            .boolean(input.enabled_in_broker);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_serverless_sasl(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ServerlessSasl,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_163) = &input.iam {
        let mut object_164 = object.key("iam").start_object();
        crate::json_ser::serialize_structure_crate_model_iam(&mut object_164, var_163)?;
        object_164.finish();
    }
    Ok(())
}
