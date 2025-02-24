// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `DeliveryStatus`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let deliverystatus = unimplemented!();
/// match deliverystatus {
///     DeliveryStatus::Duplicate => { /* ... */ },
///     DeliveryStatus::OptOut => { /* ... */ },
///     DeliveryStatus::PermanentFailure => { /* ... */ },
///     DeliveryStatus::Successful => { /* ... */ },
///     DeliveryStatus::TemporaryFailure => { /* ... */ },
///     DeliveryStatus::Throttled => { /* ... */ },
///     DeliveryStatus::UnknownFailure => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `deliverystatus` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `DeliveryStatus::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `DeliveryStatus::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `DeliveryStatus::NewFeature` is defined.
/// Specifically, when `deliverystatus` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `DeliveryStatus::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum DeliveryStatus {
    #[allow(missing_docs)] // documentation missing in model
    Duplicate,
    #[allow(missing_docs)] // documentation missing in model
    OptOut,
    #[allow(missing_docs)] // documentation missing in model
    PermanentFailure,
    #[allow(missing_docs)] // documentation missing in model
    Successful,
    #[allow(missing_docs)] // documentation missing in model
    TemporaryFailure,
    #[allow(missing_docs)] // documentation missing in model
    Throttled,
    #[allow(missing_docs)] // documentation missing in model
    UnknownFailure,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for DeliveryStatus {
    fn from(s: &str) -> Self {
        match s {
            "DUPLICATE" => DeliveryStatus::Duplicate,
            "OPT_OUT" => DeliveryStatus::OptOut,
            "PERMANENT_FAILURE" => DeliveryStatus::PermanentFailure,
            "SUCCESSFUL" => DeliveryStatus::Successful,
            "TEMPORARY_FAILURE" => DeliveryStatus::TemporaryFailure,
            "THROTTLED" => DeliveryStatus::Throttled,
            "UNKNOWN_FAILURE" => DeliveryStatus::UnknownFailure,
            other => {
                DeliveryStatus::Unknown(crate::primitives::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for DeliveryStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DeliveryStatus::from(s))
    }
}
impl DeliveryStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            DeliveryStatus::Duplicate => "DUPLICATE",
            DeliveryStatus::OptOut => "OPT_OUT",
            DeliveryStatus::PermanentFailure => "PERMANENT_FAILURE",
            DeliveryStatus::Successful => "SUCCESSFUL",
            DeliveryStatus::TemporaryFailure => "TEMPORARY_FAILURE",
            DeliveryStatus::Throttled => "THROTTLED",
            DeliveryStatus::UnknownFailure => "UNKNOWN_FAILURE",
            DeliveryStatus::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "DUPLICATE",
            "OPT_OUT",
            "PERMANENT_FAILURE",
            "SUCCESSFUL",
            "TEMPORARY_FAILURE",
            "THROTTLED",
            "UNKNOWN_FAILURE",
        ]
    }
}
impl AsRef<str> for DeliveryStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
