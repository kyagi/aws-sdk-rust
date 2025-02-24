// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateVpcPeeringConnectionInput {
    /// <p>A unique identifier for the fleet. You can use either the fleet ID or ARN value. This tells Amazon GameLift which GameLift VPC to peer with. </p>
    #[doc(hidden)]
    pub fleet_id: std::option::Option<std::string::String>,
    /// <p>A unique identifier for the Amazon Web Services account with the VPC that you want to peer your Amazon GameLift fleet with. You can find your Account ID in the Amazon Web Services Management Console under account settings.</p>
    #[doc(hidden)]
    pub peer_vpc_aws_account_id: std::option::Option<std::string::String>,
    /// <p>A unique identifier for a VPC with resources to be accessed by your GameLift fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with GameLift Fleets</a>.</p>
    #[doc(hidden)]
    pub peer_vpc_id: std::option::Option<std::string::String>,
}
impl CreateVpcPeeringConnectionInput {
    /// <p>A unique identifier for the fleet. You can use either the fleet ID or ARN value. This tells Amazon GameLift which GameLift VPC to peer with. </p>
    pub fn fleet_id(&self) -> std::option::Option<&str> {
        self.fleet_id.as_deref()
    }
    /// <p>A unique identifier for the Amazon Web Services account with the VPC that you want to peer your Amazon GameLift fleet with. You can find your Account ID in the Amazon Web Services Management Console under account settings.</p>
    pub fn peer_vpc_aws_account_id(&self) -> std::option::Option<&str> {
        self.peer_vpc_aws_account_id.as_deref()
    }
    /// <p>A unique identifier for a VPC with resources to be accessed by your GameLift fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with GameLift Fleets</a>.</p>
    pub fn peer_vpc_id(&self) -> std::option::Option<&str> {
        self.peer_vpc_id.as_deref()
    }
}
impl CreateVpcPeeringConnectionInput {
    /// Creates a new builder-style object to manufacture [`CreateVpcPeeringConnectionInput`](crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionInput).
    pub fn builder() -> crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionInputBuilder{
        crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionInputBuilder::default()
    }
}

/// A builder for [`CreateVpcPeeringConnectionInput`](crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct CreateVpcPeeringConnectionInputBuilder {
    pub(crate) fleet_id: std::option::Option<std::string::String>,
    pub(crate) peer_vpc_aws_account_id: std::option::Option<std::string::String>,
    pub(crate) peer_vpc_id: std::option::Option<std::string::String>,
}
impl CreateVpcPeeringConnectionInputBuilder {
    /// <p>A unique identifier for the fleet. You can use either the fleet ID or ARN value. This tells Amazon GameLift which GameLift VPC to peer with. </p>
    pub fn fleet_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.fleet_id = Some(input.into());
        self
    }
    /// <p>A unique identifier for the fleet. You can use either the fleet ID or ARN value. This tells Amazon GameLift which GameLift VPC to peer with. </p>
    pub fn set_fleet_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.fleet_id = input;
        self
    }
    /// <p>A unique identifier for the Amazon Web Services account with the VPC that you want to peer your Amazon GameLift fleet with. You can find your Account ID in the Amazon Web Services Management Console under account settings.</p>
    pub fn peer_vpc_aws_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.peer_vpc_aws_account_id = Some(input.into());
        self
    }
    /// <p>A unique identifier for the Amazon Web Services account with the VPC that you want to peer your Amazon GameLift fleet with. You can find your Account ID in the Amazon Web Services Management Console under account settings.</p>
    pub fn set_peer_vpc_aws_account_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.peer_vpc_aws_account_id = input;
        self
    }
    /// <p>A unique identifier for a VPC with resources to be accessed by your GameLift fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with GameLift Fleets</a>.</p>
    pub fn peer_vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.peer_vpc_id = Some(input.into());
        self
    }
    /// <p>A unique identifier for a VPC with resources to be accessed by your GameLift fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with GameLift Fleets</a>.</p>
    pub fn set_peer_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.peer_vpc_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateVpcPeeringConnectionInput`](crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionInput {
                fleet_id: self.fleet_id,
                peer_vpc_aws_account_id: self.peer_vpc_aws_account_id,
                peer_vpc_id: self.peer_vpc_id,
            },
        )
    }
}
