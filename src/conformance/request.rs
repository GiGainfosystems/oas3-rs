use bytes::Bytes;
use http::HeaderMap;

use super::{OperationSpec, ParamReplacement, TestAuthorization, TestOperation, TestParam};

#[derive(Debug, Clone)]
pub enum RequestSource {
    Example { media_type: String, name: String },
    Raw(Bytes),
    Empty,
}

#[derive(Debug, Clone)]
pub struct RequestSpec {
    pub source: RequestSource,
    pub bad: bool,
    pub auth: Option<TestAuthorization>,
    pub params: Vec<ParamReplacement>,
}

impl RequestSpec {
    pub fn empty() -> Self {
        Self {
            source: RequestSource::Empty,
            bad: false,
            auth: None,
            params: vec![],
        }
    }

    pub fn from_example<M, N>(media_type: M, name: N) -> Self
    where
        M: Into<String>,
        N: Into<String>,
    {
        Self {
            source: RequestSource::Example {
                media_type: media_type.into(),
                name: name.into(),
            },
            ..Self::empty()
        }
    }

    pub fn from_json_example<N>(name: N) -> Self
    where
        N: Into<String>,
    {
        Self {
            source: RequestSource::Example {
                media_type: "application/json".to_owned(),
                name: name.into(),
            },
            ..Self::empty()
        }
    }

    pub fn from_bad_raw<T>(body: T) -> Self
    where
        T: Into<Bytes>,
    {
        Self {
            source: RequestSource::Raw(body.into()),
            bad: true,
            ..Self::empty()
        }
    }

    pub fn with_auth(self, auth: &TestAuthorization) -> Self {
        Self {
            auth: Some(auth.clone()),
            ..self
        }
    }

    pub fn add_param<N, V>(mut self, name: N, val: V) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
        let param = ParamReplacement::new(name, val);
        self.params.push(param);
        self
    }
}

#[derive(Debug, Clone)]
pub struct TestRequest {
    pub operation: TestOperation,
    pub headers: HeaderMap,
    pub params: Vec<TestParam>,
    pub body: Bytes,
}
