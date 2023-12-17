use async_trait::async_trait;
use std::fmt::Debug;

#[doc(no_inline)]
pub use bytes::Bytes;
#[doc(no_inline)]
pub use http::{Request, Response};
use opentelemetry::{
    propagation::{Extractor, Injector},
    trace::TraceError,
};

pub struct HeaderInjector<'a>(pub &'a mut http::HeaderMap);

impl<'a> Injector for HeaderInjector<'a> {
    /// Set a key and value in the HeaderMap.  Does nothing if the key or value are not valid inputs.
    fn set(&mut self, key: &str, value: String) {
        if let Ok(name) = http::header::HeaderName::from_bytes(key.as_bytes()) {
            if let Ok(val) = http::header::HeaderValue::from_str(&value) {
                self.0.insert(name, val);
            }
        }
    }
}

pub struct HeaderExtractor<'a>(pub &'a http::HeaderMap);

impl<'a> Extractor for HeaderExtractor<'a> {
    /// Get a value for a key from the HeaderMap.  If the value is not valid ASCII, returns None.
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.to_str().ok())
    }

    /// Collect all the keys from the HeaderMap.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|value| value.as_str())
            .collect::<Vec<_>>()
    }
}

pub type HttpError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// A minimal interface necessary for export spans over HTTP.
///
/// Users sometime choose HTTP clients that relay on a certain async runtime. This trait allows
/// users to bring their choice of HTTP client.
#[async_trait]
pub trait HttpClient: Debug + Send + Sync {
    /// Send the specified HTTP request
    ///
    /// Returns the HTTP response including the status code and body.
    ///
    /// Returns an error if it can't connect to the server or the request could not be completed,
    /// e.g. because of a timeout, infinite redirects, or a loss of connection.
    async fn send(&self, request: Request<Vec<u8>>) -> Result<Response<Bytes>, HttpError>;
}

#[cfg(any(feature = "hyper", feature = "hyper_tls"))]
pub mod hyper {
    use super::{async_trait, Bytes, HttpClient, HttpError, Request, Response};
    use http::HeaderValue;
    use http_body_util::{BodyExt, Full};
    use hyper_util::client::legacy::connect::Connect;
    use hyper_util::client::legacy::Client;
    use bytes::Buf;
    use std::fmt::{Debug, Formatter};
    use std::time::Duration;
    use tokio::time;

    #[derive(Clone)]
    pub struct HyperClient<C>
    where
        C: Connect + Clone + Send + Sync + 'static,
    {
        inner: Client<C, Full<Bytes>>,
        timeout: Duration,
        authorization: Option<HeaderValue>,
    }

    impl<C> HyperClient<C>
    where
        C: Connect + Clone + Send + Sync + 'static,
    {
        pub fn new_with_timeout(inner: Client<C, Full<Bytes>>, timeout: Duration) -> Self {
            Self {
                inner,
                timeout,
                authorization: None,
            }
        }

        pub fn new_with_timeout_and_authorization_header(
            inner: Client<C, Full<Bytes>>,
            timeout: Duration,
            authorization: HeaderValue,
        ) -> Self {
            Self {
                inner,
                timeout,
                authorization: Some(authorization),
            }
        }
    }

    impl<C> Debug for HyperClient<C>
    where
        C: Connect + Clone + Send + Sync + 'static,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    #[async_trait]
    impl<C> HttpClient for HyperClient<C>
    where
        C: Connect + Clone + Send + Sync + 'static,
    {
        async fn send(
            &self,
            request: Request<Vec<u8>>,
        ) -> Result<Response<Bytes>, HttpError> {
            let (parts, body) = request.into_parts();

            let mut request = hyper::Request::builder()
                .method(parts.method.as_str())
                .uri(parts.uri.to_string())
                .body(Full::from(body))
                .unwrap();

            // add authorization headers
            if let Some(ref authorization) = self.authorization {
                request
                    .headers_mut()
                    .insert(http::header::AUTHORIZATION, authorization.clone());
            }

            // http call
            let resp = time::timeout(self.timeout, self.inner.request(request))
                .await??;

            // sink all bytes
            let (parts, body) = resp.into_parts();
            let mut body = body.collect().await?.aggregate();
            let bytes = body.copy_to_bytes(body.remaining());
            let mut resp = Response::builder()
                .status(parts.status)
                .version(parts.version)
                .extension(parts.extensions)
                .body(bytes)?;

            *resp.headers_mut() = parts.headers;

            Ok(resp)
        }
    }
}

/// Methods to make working with responses from the [`HttpClient`] trait easier.
pub trait ResponseExt: Sized {
    /// Turn a response into an error if the HTTP status does not indicate success (200 - 299).
    fn error_for_status(self) -> Result<Self, TraceError>;
}

impl<T> ResponseExt for Response<T> {
    fn error_for_status(self) -> Result<Self, TraceError> {
        if self.status().is_success() {
            Ok(self)
        } else {
            Err(format!("request failed with status {}", self.status()).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_headers_get() {
        let mut carrier = http::HeaderMap::new();
        HeaderInjector(&mut carrier).set("headerName", "value".to_string());

        assert_eq!(
            HeaderExtractor(&carrier).get("HEADERNAME"),
            Some("value"),
            "case insensitive extraction"
        )
    }

    #[test]
    fn http_headers_keys() {
        let mut carrier = http::HeaderMap::new();
        HeaderInjector(&mut carrier).set("headerName1", "value1".to_string());
        HeaderInjector(&mut carrier).set("headerName2", "value2".to_string());

        let extractor = HeaderExtractor(&carrier);
        let got = extractor.keys();
        assert_eq!(got.len(), 2);
        assert!(got.contains(&"headername1"));
        assert!(got.contains(&"headername2"));
    }
}
