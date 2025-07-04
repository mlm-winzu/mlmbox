// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRequest {
    /// Необязательный фильтр по статусу.
    /// По умолчанию могут возвращаться только активные локали.
    #[prost(enumeration = "super::super::types::locale::Status", optional, tag = "1")]
    pub status: ::core::option::Option<i32>,
    /// "Курсор", с которого начинать выборку. Указывается идентификатор локали.
    #[prost(message, optional, tag = "2")]
    pub cursor: ::core::option::Option<super::super::types::locale::Id>,
    /// Необязательные параметры сортировки и лимита.
    #[prost(message, optional, tag = "3")]
    pub sort: ::core::option::Option<super::super::types::Sort>,
}
/// Generated server implementations.
pub mod locale_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LocaleServiceServer.
    #[async_trait]
    pub trait LocaleService: std::marker::Send + std::marker::Sync + 'static {
        /// Получает текущую установленную локаль пользователя.
        async fn get_current(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::types::Locale>,
            tonic::Status,
        >;
        /// Устанавливает новую локаль для текущего пользователя.
        async fn set_current(
            &self,
            request: tonic::Request<super::super::super::types::locale::Id>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::types::Locale>,
            tonic::Status,
        >;
        /// Получает информацию о конкретной локали по ее коду или ID.
        async fn get(
            &self,
            request: tonic::Request<super::super::super::types::locale::Id>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::types::Locale>,
            tonic::Status,
        >;
        /// Возвращает список всех локалей, доступных в системе.
        async fn list(
            &self,
            request: tonic::Request<super::ListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::types::locale::List>,
            tonic::Status,
        >;
    }
    /// Сервис для управления локалями и языковыми предпочтениями пользователя.
    #[derive(Debug)]
    pub struct LocaleServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> LocaleServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LocaleServiceServer<T>
    where
        T: LocaleService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/biconom.client.locale.LocaleService/GetCurrent" => {
                    #[allow(non_camel_case_types)]
                    struct GetCurrentSvc<T: LocaleService>(pub Arc<T>);
                    impl<T: LocaleService> tonic::server::UnaryService<()>
                    for GetCurrentSvc<T> {
                        type Response = super::super::super::types::Locale;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LocaleService>::get_current(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetCurrentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/biconom.client.locale.LocaleService/SetCurrent" => {
                    #[allow(non_camel_case_types)]
                    struct SetCurrentSvc<T: LocaleService>(pub Arc<T>);
                    impl<
                        T: LocaleService,
                    > tonic::server::UnaryService<super::super::super::types::locale::Id>
                    for SetCurrentSvc<T> {
                        type Response = super::super::super::types::Locale;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::types::locale::Id,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LocaleService>::set_current(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SetCurrentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/biconom.client.locale.LocaleService/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: LocaleService>(pub Arc<T>);
                    impl<
                        T: LocaleService,
                    > tonic::server::UnaryService<super::super::super::types::locale::Id>
                    for GetSvc<T> {
                        type Response = super::super::super::types::Locale;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::types::locale::Id,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LocaleService>::get(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/biconom.client.locale.LocaleService/List" => {
                    #[allow(non_camel_case_types)]
                    struct ListSvc<T: LocaleService>(pub Arc<T>);
                    impl<
                        T: LocaleService,
                    > tonic::server::UnaryService<super::ListRequest> for ListSvc<T> {
                        type Response = super::super::super::types::locale::List;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LocaleService>::list(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for LocaleServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "biconom.client.locale.LocaleService";
    impl<T> tonic::server::NamedService for LocaleServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
