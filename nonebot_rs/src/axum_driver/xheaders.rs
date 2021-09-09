use headers::{Header, HeaderName, HeaderValue};

lazy_static! {
    static ref XSI: HeaderName = HeaderName::from_static("x-self-id");
    static ref XCR: HeaderName = HeaderName::from_static("x-client-role");
    static ref AUTH: HeaderName = HeaderName::from_static("authorization");
}
pub struct XSelfId(pub String);

impl Header for XSelfId {
    fn name() -> &'static HeaderName {
        &XSI
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;
        if let Ok(id) = value.to_str() {
            Ok(XSelfId(id.to_string()))
        } else {
            Err(headers::Error::invalid())
        }
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        let s = self.0.to_string();
        let value = HeaderValue::from_str(&s).unwrap();
        values.extend(std::iter::once(value));
    }
}

pub struct XClientRole(pub String);

impl Header for XClientRole {
    fn name() -> &'static HeaderName {
        &XCR
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;
        if let Ok(client_role) = value.to_str() {
            Ok(XClientRole(client_role.to_string()))
        } else {
            Err(headers::Error::invalid())
        }
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        let s = self.0.clone();
        let value = HeaderValue::from_str(&s).unwrap();
        values.extend(std::iter::once(value));
    }
}

pub struct Authorization(pub String);

impl Header for Authorization {
    fn name() -> &'static HeaderName {
        &AUTH
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;
        if let Ok(authorization) = value.to_str() {
            Ok(Authorization(authorization.to_string()))
        } else {
            Err(headers::Error::invalid())
        }
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        let s = self.0.clone();
        let value = HeaderValue::from_str(&s).unwrap();
        values.extend(std::iter::once(value));
    }
}
