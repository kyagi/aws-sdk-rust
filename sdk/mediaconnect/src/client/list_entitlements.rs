// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListEntitlements`](crate::operation::list_entitlements::builders::ListEntitlementsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_entitlements::builders::ListEntitlementsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_entitlements::builders::ListEntitlementsFluentBuilder::max_results) / [`set_max_results(i32)`](crate::operation::list_entitlements::builders::ListEntitlementsFluentBuilder::set_max_results): The maximum number of results to return per API request. For example, you submit a ListEntitlements request with MaxResults set at 5. Although 20 items match your request, the service returns no more than the first 5 items. (The service also returns a NextToken value that you can use to fetch the next batch of results.) The service might return fewer results than the MaxResults value. If MaxResults is not included in the request, the service defaults to pagination with a maximum of 20 results per page.
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_entitlements::builders::ListEntitlementsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_entitlements::builders::ListEntitlementsFluentBuilder::set_next_token): The token that identifies which batch of results that you want to see. For example, you submit a ListEntitlements request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListEntitlements request a second time and specify the NextToken value.
    /// - On success, responds with [`ListEntitlementsOutput`](crate::operation::list_entitlements::ListEntitlementsOutput) with field(s):
    ///   - [`entitlements(Option<Vec<ListedEntitlement>>)`](crate::operation::list_entitlements::ListEntitlementsOutput::entitlements): A list of entitlements that have been granted to you from other AWS accounts.
    ///   - [`next_token(Option<String>)`](crate::operation::list_entitlements::ListEntitlementsOutput::next_token): The token that identifies which batch of results that you want to see. For example, you submit a ListEntitlements request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListEntitlements request a second time and specify the NextToken value.
    /// - On failure, responds with [`SdkError<ListEntitlementsError>`](crate::operation::list_entitlements::ListEntitlementsError)
    pub fn list_entitlements(
        &self,
    ) -> crate::operation::list_entitlements::builders::ListEntitlementsFluentBuilder {
        crate::operation::list_entitlements::builders::ListEntitlementsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
