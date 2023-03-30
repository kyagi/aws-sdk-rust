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
//! <fullname>Health</fullname>
//! 
//! <p>The Health API provides programmatic access to the Health information that
//! appears in the <a href="https://phd.aws.amazon.com/phd/home#/">Personal Health Dashboard</a>. You
//! can use the API operations to get information about events that might affect your Amazon Web Services services and resources.</p>
//! <note>
//! <ul>
//! <li>
//! <p>You must have a Business, Enterprise On-Ramp, or Enterprise Support plan from <a href="http://aws.amazon.com/premiumsupport/">Amazon Web Services Support</a> to use the Health
//! API. If you call the Health API from an Amazon Web Services account that
//! doesn't have a Business, Enterprise On-Ramp, or Enterprise Support plan, you receive a
//! <code>SubscriptionRequiredException</code> error.</p>
//! </li>
//! <li>
//! <p>You can use the Health endpoint health.us-east-1.amazonaws.com (HTTPS) to
//! call the Health API operations. Health supports a multi-Region
//! application architecture and has two regional endpoints in an active-passive
//! configuration. You can use the high availability endpoint example to determine
//! which Amazon Web Services Region is active, so that you can get the latest information from the
//! API. For more information, see <a href="https://docs.aws.amazon.com/health/latest/ug/health-api.html">Accessing the Health API</a> in the
//! <i>Health User Guide</i>.</p>
//! </li>
//! </ul>
//! </note>
//! <p>For authentication of requests, Health uses the <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 Signing
//! Process</a>.</p>
//! <p>If your Amazon Web Services account is part of Organizations, you can use the Health organizational
//! view feature. This feature provides a centralized view of Health events across all
//! accounts in your organization. You can aggregate Health events in real time to
//! identify accounts in your organization that are affected by an operational event or get
//! notified of security vulnerabilities. Use the organizational view API operations to enable
//! this feature and return event information. For more information, see <a href="https://docs.aws.amazon.com/health/latest/ug/aggregate-events.html">Aggregating
//! Health events</a> in the <i>Health User Guide</i>.</p>
//! <note>
//! <p>When you use the Health API operations to return Health events, see the
//! following recommendations:</p>
//! <ul>
//! <li>
//! <p>Use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html#AWSHealth-Type-Event-eventScopeCode">eventScopeCode</a> parameter to specify whether to return Health
//! events that are public or account-specific.</p>
//! </li>
//! <li>
//! <p>Use pagination to view all events from the response. For example, if you call
//! the <code>DescribeEventsForOrganization</code> operation to get all events in your
//! organization, you might receive several page results. Specify the
//! <code>nextToken</code> in the next request to return more results.</p>
//! </li>
//! </ul>
//! </note>
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
                    aws_http::user_agent::ApiMetadata::new("health", crate::PKG_VERSION);

pub use aws_types::app_name::AppName;

pub use aws_smithy_http::endpoint::Endpoint;


/// Crate version number.
                pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Client and fluent builders for calling AWS Health APIs and Notifications.
pub mod client;

/// Configuration for AWS Health APIs and Notifications.
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

mod json_errors;

#[doc(inline)]
pub use client::Client;

