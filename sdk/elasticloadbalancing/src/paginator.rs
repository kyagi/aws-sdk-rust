// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`DescribeLoadBalancers`](crate::operation::DescribeLoadBalancers)
pub struct DescribeLoadBalancersPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::describe_load_balancers_input::Builder,
}

impl<C, M, R> DescribeLoadBalancersPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::describe_load_balancers_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `load_balancer_descriptions`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::DescribeLoadBalancersPaginatorItems<C, M, R> {
        crate::paginator::DescribeLoadBalancersPaginatorItems(self)
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::DescribeLoadBalancersOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeLoadBalancersError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::DescribeLoadBalancersInputOperationOutputAlias,
            crate::output::DescribeLoadBalancersOutput,
            crate::error::DescribeLoadBalancersError,
            crate::input::DescribeLoadBalancersInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_describe_load_balancers_output_next_marker(resp);
                            if new_token == input.marker.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.marker = new_token.cloned();
                            input.marker.as_deref().unwrap_or_default().is_empty()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Flattened paginator for `DescribeLoadBalancersPaginator`
///
/// This is created with [`.items()`](DescribeLoadBalancersPaginator::items)
pub struct DescribeLoadBalancersPaginatorItems<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
>(DescribeLoadBalancersPaginator<C, M, R>);

impl<C, M, R> DescribeLoadBalancersPaginatorItems<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::LoadBalancerDescription,
            aws_smithy_http::result::SdkError<crate::error::DescribeLoadBalancersError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::DescribeLoadBalancersInputOperationOutputAlias,
            crate::output::DescribeLoadBalancersOutput,
            crate::error::DescribeLoadBalancersError,
            crate::input::DescribeLoadBalancersInputOperationRetryAlias,
        >,
    {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_describe_load_balancers_output_load_balancer_descriptions(page).unwrap_or_default().into_iter())
    }
}
