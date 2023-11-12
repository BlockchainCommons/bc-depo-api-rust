use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;

use crate::DELETE_ACCOUNT_FUNCTION;

use super::{parse_request, parse_response, request_body, request_envelope, response_envelope};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteAccountRequest {
    id: ARID,
    key: PublicKeyBase,
}

impl DeleteAccountRequest {
    pub fn new(key: impl AsRef<PublicKeyBase>) -> Self {
        Self::new_opt(ARID::new(), key.as_ref().clone())
    }

    pub fn new_opt(id: ARID, key: PublicKeyBase) -> Self {
        Self { id, key }
    }

    pub fn id(&self) -> &ARID {
        &self.id
    }

    pub fn key(&self) -> &PublicKeyBase {
        &self.key
    }
}

impl EnvelopeEncodable for DeleteAccountRequest {
    fn envelope(self) -> Envelope {
        request_envelope(self.id, request_body(DELETE_ACCOUNT_FUNCTION, self.key))
    }
}

impl From<DeleteAccountRequest> for Envelope {
    fn from(value: DeleteAccountRequest) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for DeleteAccountRequest {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, key, _body) = parse_request(DELETE_ACCOUNT_FUNCTION, envelope)?;
        Ok(Self::new_opt(id, key))
    }
}

impl TryFrom<Envelope> for DeleteAccountRequest {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for DeleteAccountRequest {}

//
// Response
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteAccountResponse {
    id: ARID,
}

impl DeleteAccountResponse {
    pub fn new(id: ARID) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &ARID {
        &self.id
    }
}

impl EnvelopeEncodable for DeleteAccountResponse {
    fn envelope(self) -> Envelope {
        response_envelope(self.id, None)
    }
}

impl From<DeleteAccountResponse> for Envelope {
    fn from(value: DeleteAccountResponse) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for DeleteAccountResponse {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, _result) = parse_response(envelope)?;
        Ok(Self::new(id))
    }
}

impl TryFrom<Envelope> for DeleteAccountResponse {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for DeleteAccountResponse {}

#[cfg(test)]
mod tests {
    use bc_components::PrivateKeyBase;
    use indoc::indoc;

    use super::*;

    fn id() -> ARID {
        ARID::from_data_ref(hex_literal::hex!(
            "8712dfac3d0ebfa910736b2a9ee39d4b68f64222a77bcc0074f3f5f1c9216d30"
        ))
        .unwrap()
    }

    #[test]
    fn test_request() {
        let private_key = PrivateKeyBase::new();
        let key = private_key.public_keys();

        let request = DeleteAccountRequest::new_opt(id(), key);
        let request_envelope = request.clone().envelope();
        assert_eq!(
            request_envelope.format(),
            indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"deleteAccount"» [
                ❰"key"❱: PublicKeyBase
            ]
        ]
        "#}
            .trim()
        );
        let decoded = DeleteAccountRequest::try_from(request_envelope).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let response = DeleteAccountResponse::new(id());
        let response_envelope = response.clone().envelope();
        assert_eq!(
            response_envelope.format(),
            indoc! {r#"
        response(ARID(8712dfac)) [
            'result': 'OK'
        ]
        "#}
            .trim()
        );
        let decoded = DeleteAccountResponse::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);
    }
}
