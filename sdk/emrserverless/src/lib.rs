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
//! <p>Amazon EMR Serverless is a new deployment option for Amazon EMR. EMR Serverless provides
//! a serverless runtime environment that simplifies running analytics applications using the
//! latest open source frameworks such as Apache Spark and Apache Hive. With EMR Serverless,
//! you don’t have to configure, optimize, secure, or operate clusters to run applications with
//! these frameworks.</p>
//! <p>The API reference to Amazon EMR Serverless is <code>emr-serverless</code>. The
//! <code>emr-serverless</code> prefix is used in the following scenarios: </p>
//! <ul>
//! <li>
//! <p>It is the prefix in the CLI commands for Amazon EMR Serverless. For example,
//! <code>aws emr-serverless start-job-run</code>.</p>
//! </li>
//! <li>
//! <p>It is the prefix before IAM policy actions for Amazon EMR Serverless. For example,
//! <code>"Action": ["emr-serverless:StartJobRun"]</code>. For more information, see
//! <a href="https://docs.aws.amazon.com/emr/latest/EMR-Serverless-UserGuide/security_iam_service-with-iam.html#security_iam_service-with-iam-id-based-policies-actions">Policy actions for Amazon EMR Serverless</a>.</p>
//! </li>
//! <li>
//! <p>It is the prefix used in Amazon EMR Serverless service endpoints. For example,
//! <code>emr-serverless.us-east-2.amazonaws.com</code>.</p>
//! </li>
//! </ul>
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
                    aws_http::user_agent::ApiMetadata::new("emrserverless", crate::PKG_VERSION);

pub use aws_types::app_name::AppName;

pub use aws_smithy_http::endpoint::Endpoint;


/// Crate version number.
                pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Client and fluent builders for calling EMR Serverless.
pub mod client;

/// Configuration for EMR Serverless.
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

mod idempotency_token;

/// 
pub mod middleware;

/// 
mod no_credentials;

/// Paginators for the service
pub mod paginator;

mod lens;

pub(crate) mod protocol_serde;

mod endpoint_lib;

mod json_errors;

#[doc(inline)]
pub use client::Client;

