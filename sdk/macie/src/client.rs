// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    client: aws_smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// Client for Amazon Macie
///
/// Client for invoking operations on Amazon Macie. Each operation on Amazon Macie is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_macie::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::RetryConfig;
/// # async fn docs() {
///     let shared_config = aws_config::load_from_env().await;
///     let config = aws_sdk_macie::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_macie::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl<C, M, R> From<aws_smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: aws_smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    /// Creates a client with the given service configuration.
    pub fn with_config(client: aws_smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Constructs a fluent builder for the `AssociateMemberAccount` operation.
    ///
    /// See [`AssociateMemberAccount`](crate::client::fluent_builders::AssociateMemberAccount) for more information about the
    /// operation and its arguments.
    pub fn associate_member_account(&self) -> fluent_builders::AssociateMemberAccount<C, M, R> {
        fluent_builders::AssociateMemberAccount::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `AssociateS3Resources` operation.
    ///
    /// See [`AssociateS3Resources`](crate::client::fluent_builders::AssociateS3Resources) for more information about the
    /// operation and its arguments.
    pub fn associate_s3_resources(&self) -> fluent_builders::AssociateS3Resources<C, M, R> {
        fluent_builders::AssociateS3Resources::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `DisassociateMemberAccount` operation.
    ///
    /// See [`DisassociateMemberAccount`](crate::client::fluent_builders::DisassociateMemberAccount) for more information about the
    /// operation and its arguments.
    pub fn disassociate_member_account(
        &self,
    ) -> fluent_builders::DisassociateMemberAccount<C, M, R> {
        fluent_builders::DisassociateMemberAccount::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `DisassociateS3Resources` operation.
    ///
    /// See [`DisassociateS3Resources`](crate::client::fluent_builders::DisassociateS3Resources) for more information about the
    /// operation and its arguments.
    pub fn disassociate_s3_resources(&self) -> fluent_builders::DisassociateS3Resources<C, M, R> {
        fluent_builders::DisassociateS3Resources::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `ListMemberAccounts` operation.
    ///
    /// See [`ListMemberAccounts`](crate::client::fluent_builders::ListMemberAccounts) for more information about the
    /// operation and its arguments.
    pub fn list_member_accounts(&self) -> fluent_builders::ListMemberAccounts<C, M, R> {
        fluent_builders::ListMemberAccounts::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `ListS3Resources` operation.
    ///
    /// See [`ListS3Resources`](crate::client::fluent_builders::ListS3Resources) for more information about the
    /// operation and its arguments.
    pub fn list_s3_resources(&self) -> fluent_builders::ListS3Resources<C, M, R> {
        fluent_builders::ListS3Resources::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `UpdateS3Resources` operation.
    ///
    /// See [`UpdateS3Resources`](crate::client::fluent_builders::UpdateS3Resources) for more information about the
    /// operation and its arguments.
    pub fn update_s3_resources(&self) -> fluent_builders::UpdateS3Resources<C, M, R> {
        fluent_builders::UpdateS3Resources::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `AssociateMemberAccount`.
    ///
    /// <p>Associates a specified AWS account with Amazon Macie Classic as a member
    /// account.</p>
    #[derive(std::fmt::Debug)]
    pub struct AssociateMemberAccount<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::associate_member_account_input::Builder,
    }
    impl<C, M, R> AssociateMemberAccount<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `AssociateMemberAccount`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::AssociateMemberAccountOutput,
            aws_smithy_http::result::SdkError<crate::error::AssociateMemberAccountError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::AssociateMemberAccountInputOperationOutputAlias,
                crate::output::AssociateMemberAccountOutput,
                crate::error::AssociateMemberAccountError,
                crate::input::AssociateMemberAccountInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the AWS account that you want to associate with Amazon Macie Classic as a
        /// member account.</p>
        pub fn member_account_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.member_account_id(inp);
            self
        }
        /// <p>The ID of the AWS account that you want to associate with Amazon Macie Classic as a
        /// member account.</p>
        pub fn set_member_account_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_member_account_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `AssociateS3Resources`.
    ///
    /// <p>Associates specified S3 resources with Amazon Macie Classic for monitoring and data
    /// classification. If memberAccountId isn't specified, the action associates specified S3
    /// resources with Macie Classic for the current Macie Classic administrator account. If memberAccountId is specified,
    /// the action associates specified S3 resources with Macie Classic for the specified member
    /// account. </p>
    #[derive(std::fmt::Debug)]
    pub struct AssociateS3Resources<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::associate_s3_resources_input::Builder,
    }
    impl<C, M, R> AssociateS3Resources<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `AssociateS3Resources`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::AssociateS3ResourcesOutput,
            aws_smithy_http::result::SdkError<crate::error::AssociateS3ResourcesError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::AssociateS3ResourcesInputOperationOutputAlias,
                crate::output::AssociateS3ResourcesOutput,
                crate::error::AssociateS3ResourcesError,
                crate::input::AssociateS3ResourcesInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the Amazon Macie Classic member account whose resources you want to associate
        /// with Macie Classic. </p>
        pub fn member_account_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.member_account_id(inp);
            self
        }
        /// <p>The ID of the Amazon Macie Classic member account whose resources you want to associate
        /// with Macie Classic. </p>
        pub fn set_member_account_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_member_account_id(input);
            self
        }
        /// Appends an item to `s3Resources`.
        ///
        /// To override the contents of this collection use [`set_s3_resources`](Self::set_s3_resources).
        ///
        /// <p>The S3 resources that you want to associate with Amazon Macie Classic for monitoring
        /// and data classification. </p>
        pub fn s3_resources(
            mut self,
            inp: impl Into<crate::model::S3ResourceClassification>,
        ) -> Self {
            self.inner = self.inner.s3_resources(inp);
            self
        }
        /// <p>The S3 resources that you want to associate with Amazon Macie Classic for monitoring
        /// and data classification. </p>
        pub fn set_s3_resources(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::S3ResourceClassification>>,
        ) -> Self {
            self.inner = self.inner.set_s3_resources(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DisassociateMemberAccount`.
    ///
    /// <p>Removes the specified member account from Amazon Macie Classic.</p>
    #[derive(std::fmt::Debug)]
    pub struct DisassociateMemberAccount<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::disassociate_member_account_input::Builder,
    }
    impl<C, M, R> DisassociateMemberAccount<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `DisassociateMemberAccount`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DisassociateMemberAccountOutput,
            aws_smithy_http::result::SdkError<crate::error::DisassociateMemberAccountError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DisassociateMemberAccountInputOperationOutputAlias,
                crate::output::DisassociateMemberAccountOutput,
                crate::error::DisassociateMemberAccountError,
                crate::input::DisassociateMemberAccountInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the member account that you want to remove from Amazon Macie
        /// Classic.</p>
        pub fn member_account_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.member_account_id(inp);
            self
        }
        /// <p>The ID of the member account that you want to remove from Amazon Macie
        /// Classic.</p>
        pub fn set_member_account_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_member_account_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DisassociateS3Resources`.
    ///
    /// <p>Removes specified S3 resources from being monitored by Amazon Macie Classic. If
    /// memberAccountId isn't specified, the action removes specified S3 resources from Macie Classic
    /// for the current Macie Classic administrator account. If memberAccountId is specified, the action removes specified
    /// S3 resources from Macie Classic for the specified member account.</p>
    #[derive(std::fmt::Debug)]
    pub struct DisassociateS3Resources<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::disassociate_s3_resources_input::Builder,
    }
    impl<C, M, R> DisassociateS3Resources<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `DisassociateS3Resources`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DisassociateS3ResourcesOutput,
            aws_smithy_http::result::SdkError<crate::error::DisassociateS3ResourcesError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DisassociateS3ResourcesInputOperationOutputAlias,
                crate::output::DisassociateS3ResourcesOutput,
                crate::error::DisassociateS3ResourcesError,
                crate::input::DisassociateS3ResourcesInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the Amazon Macie Classic member account whose resources you want to remove
        /// from being monitored by Macie Classic. </p>
        pub fn member_account_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.member_account_id(inp);
            self
        }
        /// <p>The ID of the Amazon Macie Classic member account whose resources you want to remove
        /// from being monitored by Macie Classic. </p>
        pub fn set_member_account_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_member_account_id(input);
            self
        }
        /// Appends an item to `associatedS3Resources`.
        ///
        /// To override the contents of this collection use [`set_associated_s3_resources`](Self::set_associated_s3_resources).
        ///
        /// <p>The S3 resources (buckets or prefixes) that you want to remove from being monitored and
        /// classified by Amazon Macie Classic. </p>
        pub fn associated_s3_resources(mut self, inp: impl Into<crate::model::S3Resource>) -> Self {
            self.inner = self.inner.associated_s3_resources(inp);
            self
        }
        /// <p>The S3 resources (buckets or prefixes) that you want to remove from being monitored and
        /// classified by Amazon Macie Classic. </p>
        pub fn set_associated_s3_resources(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::S3Resource>>,
        ) -> Self {
            self.inner = self.inner.set_associated_s3_resources(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListMemberAccounts`.
    ///
    /// <p>Lists all Amazon Macie Classic member accounts for the current Macie Classic administrator account.</p>
    #[derive(std::fmt::Debug)]
    pub struct ListMemberAccounts<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_member_accounts_input::Builder,
    }
    impl<C, M, R> ListMemberAccounts<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `ListMemberAccounts`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListMemberAccountsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListMemberAccountsError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListMemberAccountsInputOperationOutputAlias,
                crate::output::ListMemberAccountsOutput,
                crate::error::ListMemberAccountsError,
                crate::input::ListMemberAccountsInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>Use this parameter when paginating results. Set the value of this parameter to null on
        /// your first call to the ListMemberAccounts action. Subsequent calls to the action fill
        /// nextToken in the request with the value of nextToken from the previous response to continue
        /// listing data. </p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        /// <p>Use this parameter when paginating results. Set the value of this parameter to null on
        /// your first call to the ListMemberAccounts action. Subsequent calls to the action fill
        /// nextToken in the request with the value of nextToken from the previous response to continue
        /// listing data. </p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>Use this parameter to indicate the maximum number of items that you want in the
        /// response. The default value is 250. </p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        /// <p>Use this parameter to indicate the maximum number of items that you want in the
        /// response. The default value is 250. </p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListS3Resources`.
    ///
    /// <p>Lists all the S3 resources associated with Amazon Macie Classic. If memberAccountId
    /// isn't specified, the action lists the S3 resources associated with Macie Classic for
    /// the current Macie Classic administrator account. If memberAccountId is specified, the action lists the S3 resources
    /// associated with Macie Classic for the specified member account. </p>
    #[derive(std::fmt::Debug)]
    pub struct ListS3Resources<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_s3_resources_input::Builder,
    }
    impl<C, M, R> ListS3Resources<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `ListS3Resources`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListS3ResourcesOutput,
            aws_smithy_http::result::SdkError<crate::error::ListS3ResourcesError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListS3ResourcesInputOperationOutputAlias,
                crate::output::ListS3ResourcesOutput,
                crate::error::ListS3ResourcesError,
                crate::input::ListS3ResourcesInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The Amazon Macie Classic member account ID whose associated S3 resources you want to
        /// list. </p>
        pub fn member_account_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.member_account_id(inp);
            self
        }
        /// <p>The Amazon Macie Classic member account ID whose associated S3 resources you want to
        /// list. </p>
        pub fn set_member_account_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_member_account_id(input);
            self
        }
        /// <p>Use this parameter when paginating results. Set its value to null on your first call to
        /// the ListS3Resources action. Subsequent calls to the action fill nextToken in the request with
        /// the value of nextToken from the previous response to continue listing data. </p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        /// <p>Use this parameter when paginating results. Set its value to null on your first call to
        /// the ListS3Resources action. Subsequent calls to the action fill nextToken in the request with
        /// the value of nextToken from the previous response to continue listing data. </p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>Use this parameter to indicate the maximum number of items that you want in the
        /// response. The default value is 250. </p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        /// <p>Use this parameter to indicate the maximum number of items that you want in the
        /// response. The default value is 250. </p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    /// Fluent builder constructing a request to `UpdateS3Resources`.
    ///
    /// <p>Updates the classification types for the specified S3 resources. If memberAccountId
    /// isn't specified, the action updates the classification types of the S3 resources associated
    /// with Amazon Macie Classic for the current Macie Classic administrator account. If memberAccountId is specified, the
    /// action updates the classification types of the S3 resources associated with Macie
    /// Classic for the specified member account. </p>
    #[derive(std::fmt::Debug)]
    pub struct UpdateS3Resources<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_s3_resources_input::Builder,
    }
    impl<C, M, R> UpdateS3Resources<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `UpdateS3Resources`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UpdateS3ResourcesOutput,
            aws_smithy_http::result::SdkError<crate::error::UpdateS3ResourcesError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateS3ResourcesInputOperationOutputAlias,
                crate::output::UpdateS3ResourcesOutput,
                crate::error::UpdateS3ResourcesError,
                crate::input::UpdateS3ResourcesInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The AWS ID of the Amazon Macie Classic member account whose S3 resources'
        /// classification types you want to update. </p>
        pub fn member_account_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.member_account_id(inp);
            self
        }
        /// <p>The AWS ID of the Amazon Macie Classic member account whose S3 resources'
        /// classification types you want to update. </p>
        pub fn set_member_account_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_member_account_id(input);
            self
        }
        /// Appends an item to `s3ResourcesUpdate`.
        ///
        /// To override the contents of this collection use [`set_s3_resources_update`](Self::set_s3_resources_update).
        ///
        /// <p>The S3 resources whose classification types you want to update.</p>
        pub fn s3_resources_update(
            mut self,
            inp: impl Into<crate::model::S3ResourceClassificationUpdate>,
        ) -> Self {
            self.inner = self.inner.s3_resources_update(inp);
            self
        }
        /// <p>The S3 resources whose classification types you want to update.</p>
        pub fn set_s3_resources_update(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::S3ResourceClassificationUpdate>>,
        ) -> Self {
            self.inner = self.inner.set_s3_resources_update(input);
            self
        }
    }
}
impl<C> Client<C, crate::middleware::DefaultMiddleware, aws_smithy_client::retry::Standard> {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::new()
            .connector(conn)
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        aws_smithy_client::erase::DynConnector,
        crate::middleware::DefaultMiddleware,
        aws_smithy_client::retry::Standard,
    >
{
    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::dyn_https()
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        // the builder maintains a try-state. To avoid suppressing the warning when sleep is unset,
        // only set it if we actually have a sleep impl.
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
