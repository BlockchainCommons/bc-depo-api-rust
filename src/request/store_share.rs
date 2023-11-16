use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;
use bytes::Bytes;

use crate::{STORE_SHARE_FUNCTION, DATA_PARAM, receipt::Receipt, util::{Abbrev, FlankedFunction}};

use super::{request_body, request_envelope, parse_request, response_envelope, parse_response};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreShareRequest {
    id: ARID,
    key: PublicKeyBase,
    data: Bytes,
}

impl StoreShareRequest {
    pub fn new(key: impl AsRef<PublicKeyBase>, data: impl AsRef<[u8]>) -> Self {
        Self::new_opt(ARID::new(), key.as_ref().clone(), Bytes::copy_from_slice(data.as_ref()))
    }

    pub fn new_opt(id: ARID, key: PublicKeyBase, data: Bytes) -> Self {
        Self {
            id,
            key,
            data,
        }
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }

    pub fn key(&self) -> &PublicKeyBase {
        &self.key
    }

    pub fn data(&self) -> &Bytes {
        &self.data
    }
}

impl EnvelopeEncodable for StoreShareRequest {
    fn envelope(self) -> Envelope {
        let body = request_body(STORE_SHARE_FUNCTION, self.key)
            .add_parameter(DATA_PARAM, self.data);
        request_envelope(self.id, body)
    }
}

impl From<StoreShareRequest> for Envelope {
    fn from(value: StoreShareRequest) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for StoreShareRequest {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, key, body) = parse_request(STORE_SHARE_FUNCTION, envelope)?;
        let data: Bytes = body.extract_object_for_parameter(DATA_PARAM)?;
        Ok(Self::new_opt(id, key, data))
    }
}

impl TryFrom<Envelope> for StoreShareRequest {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for StoreShareRequest {}

impl std::fmt::Display for StoreShareRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {} {} key {}",
            self.id().abbrev(),
            "storeShare".flanked_function(),
            self.data().abbrev(),
            self.key().abbrev()
        ))
    }
}

//
// Response
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreShareResponse {
    id: ARID,
    receipt: Receipt,
}

impl StoreShareResponse {
    pub fn new(id: ARID, receipt: Receipt) -> Self {
        Self {
            id,
            receipt,
        }
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }

    pub fn receipt(&self) -> Receipt {
        self.receipt.clone()
    }
}

impl EnvelopeEncodable for StoreShareResponse {
    fn envelope(self) -> Envelope {
        response_envelope(self.id, Some(self.receipt.into()))
    }
}

impl From<StoreShareResponse> for Envelope {
    fn from(value: StoreShareResponse) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for StoreShareResponse {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, result) = parse_response(envelope)?;
        Ok(Self::new(id, result.try_into()?))
    }
}

impl TryFrom<Envelope> for StoreShareResponse {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for StoreShareResponse {}

impl std::fmt::Display for StoreShareResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {} OK receipt {}",
            self.id().abbrev(),
            "storeShare".flanked_function(),
            self.receipt().abbrev()
        ))
    }
}

#[cfg(test)]
mod tests {
    use bc_components::PrivateKeyBase;
    use indoc::indoc;

    use super::*;

    fn id() -> ARID {
        ARID::from_data_ref(hex_literal::hex!("8712dfac3d0ebfa910736b2a9ee39d4b68f64222a77bcc0074f3f5f1c9216d30")).unwrap()
    }

    fn user_id() -> ARID {
        ARID::from_data_ref(hex_literal::hex!("8712dfac3d0ebfa910736b2a9ee39d4b68f64222a77bcc0074f3f5f1c9216d30")).unwrap()
    }

    fn data_1() -> Bytes {
        Bytes::from_static(b"data_1")
    }

    fn receipt_1() -> Receipt {
        Receipt::new(&user_id(), data_1())
    }

    #[test]
    fn test_request() {
        let private_key = PrivateKeyBase::new();
        let key = private_key.public_keys();

        let request = StoreShareRequest::new_opt(id(), key, Bytes::from_static(b"data"));
        let request_envelope = request.clone().envelope();
        assert_eq!(request_envelope.format(),
        indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"storeShare"» [
                ❰"data"❱: Bytes(4)
                ❰"key"❱: PublicKeyBase
            ]
        ]
        "#}.trim()
        );
        let decoded = StoreShareRequest::try_from(request_envelope).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let response = StoreShareResponse::new(id(), receipt_1());
        let response_envelope = response.clone().envelope();
        assert_eq!(response_envelope.format(),
        indoc! {r#"
        response(ARID(8712dfac)) [
            'result': Bytes(32) [
                'isA': "Receipt"
            ]
        ]
        "#}.trim()
        );
        let decoded = StoreShareResponse::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);
    }
}
