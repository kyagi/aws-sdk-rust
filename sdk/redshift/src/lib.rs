#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(rustdoc::bare_urls)]

#![warn(missing_docs)]
//! <fullname>Amazon Redshift</fullname>
//! <p>
//! <b>Overview</b>
//! </p>
//! <p>This is an interface reference for Amazon Redshift. It contains documentation for one of
//! the programming or command line interfaces you can use to manage Amazon Redshift clusters.
//! Note that Amazon Redshift is asynchronous, which means that some interfaces may require
//! techniques, such as polling or asynchronous callback handlers, to determine when a
//! command has been applied. In this reference, the parameter descriptions indicate whether
//! a change is applied immediately, on the next instance reboot, or during the next
//! maintenance window. For a summary of the Amazon Redshift cluster management interfaces, go to
//! <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/using-aws-sdk.html">Using the
//! Amazon Redshift Management Interfaces</a>.</p>
//! <p>Amazon Redshift manages all the work of setting up, operating, and scaling a data
//! warehouse: provisioning capacity, monitoring and backing up the cluster, and applying
//! patches and upgrades to the Amazon Redshift engine. You can focus on using your data to
//! acquire new insights for your business and customers.</p>
//! <p>If you are a first-time user of Amazon Redshift, we recommend that you begin by reading
//! the <a href="https://docs.aws.amazon.com/redshift/latest/gsg/getting-started.html">Amazon Redshift Getting Started Guide</a>.</p>
//! 
//! <p>If you are a database developer, the <a href="https://docs.aws.amazon.com/redshift/latest/dg/welcome.html">Amazon Redshift Database Developer Guide</a> explains how to design,
//! build, query, and maintain the databases that make up your data warehouse. </p>
//! 
//! # Crate Organization
//! 
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//! 
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//! 
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//! 
//! The other modules within this crate are not required for normal usage.


// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

pub use aws_credential_types::Credentials;

pub use aws_types::region::Region;

pub(crate) static API_METADATA: aws_http::user_agent::ApiMetadata =
                    aws_http::user_agent::ApiMetadata::new("redshift", crate::PKG_VERSION);

pub use aws_types::app_name::AppName;

pub use aws_smithy_http::endpoint::Endpoint;


/// Crate version number.
                pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Client and fluent builders for calling Amazon Redshift.
pub mod client;

/// Configuration for Amazon Redshift.
pub mod config;

/// Endpoint resolution functionality.
pub mod endpoint;

/// All error types that operations can return. Documentation on these types is copied from the model.
pub mod error;

mod error_meta;

/// Input structures for operations. Documentation on these types is copied from the model.
pub mod input;

/// Data structures used by operation inputs/outputs.
pub mod model;

/// All operations that this crate can perform.
pub mod operation;

/// Output structures for operations. Documentation on these types is copied from the model.
pub mod output;

/// Data primitives referenced by other data types.
pub mod types;

/// 
pub mod middleware;

/// 
mod no_credentials;

/// Paginators for the service
pub mod paginator;

mod lens;

pub(crate) mod protocol_serde;

mod endpoint_lib;

mod rest_xml_wrapped_errors;

#[doc(inline)]
pub use client::Client;

