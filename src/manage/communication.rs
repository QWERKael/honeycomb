#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CmdRequest {
    #[prost(enumeration = "cmd_request::CmdType", tag = "1")]
    pub cmd: i32,
    #[prost(bytes, repeated, tag = "2")]
    pub args: ::std::vec::Vec<std::vec::Vec<u8>>,
}
pub mod cmd_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CmdType {
        Set = 0,
        Get = 1,
        Delete = 2,
        UseDb = 3,
        ShowDb = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CmdReply {
    #[prost(enumeration = "cmd_reply::ExeState", tag = "1")]
    pub status: i32,
    #[prost(bytes, tag = "2")]
    pub result: std::vec::Vec<u8>,
}
pub mod cmd_reply {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExeState {
        Ok = 0,
        Err = 1,
    }
}
#[doc = r" Generated client implementations."]
pub mod commander_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct CommanderClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CommanderClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CommanderClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn cmd_call(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::CmdRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::CmdReply>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/communication.Commander/CmdCall");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for CommanderClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CommanderClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CommanderClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod commander_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CommanderServer."]
    #[async_trait]
    pub trait Commander: Send + Sync + 'static {
        #[doc = "Server streaming response type for the CmdCall method."]
        type CmdCallStream: Stream<Item = Result<super::CmdReply, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn cmd_call(
            &self,
            request: tonic::Request<tonic::Streaming<super::CmdRequest>>,
        ) -> Result<tonic::Response<Self::CmdCallStream>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct CommanderServer<T: Commander> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Commander> CommanderServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for CommanderServer<T>
    where
        T: Commander,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/communication.Commander/CmdCall" => {
                    #[allow(non_camel_case_types)]
                    struct CmdCallSvc<T: Commander>(pub Arc<T>);
                    impl<T: Commander> tonic::server::StreamingService<super::CmdRequest> for CmdCallSvc<T> {
                        type Response = super::CmdReply;
                        type ResponseStream = T::CmdCallStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::CmdRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.cmd_call(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = CmdCallSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Commander> Clone for CommanderServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Commander> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Commander> tonic::transport::NamedService for CommanderServer<T> {
        const NAME: &'static str = "communication.Commander";
    }
}
