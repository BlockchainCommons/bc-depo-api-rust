use std::collections::HashSet;

use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;

use crate::{receipt::Receipt, DELETE_SHARES_FUNCTION, RECEIPT_PARAM};

use super::{request_envelope, request_body, parse_request, response_envelope, parse_response};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteSharesRequest {
    id: ARID,
    key: PublicKeyBase,
    receipts: HashSet<Receipt>,
}

impl DeleteSharesRequest {
    pub fn new(
        id: ARID,
        key: PublicKeyBase,
        receipts: HashSet<Receipt>,
    ) -> Self {
        Self {
            id,
            key,
            receipts,
        }
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }

    pub fn key(&self) -> &PublicKeyBase {
        &self.key
    }

    pub fn receipts(&self) -> &HashSet<Receipt> {
        &self.receipts
    }
}

impl EnvelopeEncodable for DeleteSharesRequest {
    fn envelope(self) -> Envelope {
        let mut body = request_body(DELETE_SHARES_FUNCTION, self.key);
        for receipt in self.receipts {
            body = body.add_parameter(RECEIPT_PARAM, receipt);
        }
        request_envelope(self.id, body)
    }
}

impl From<DeleteSharesRequest> for Envelope {
    fn from(value: DeleteSharesRequest) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for DeleteSharesRequest {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, key, body) = parse_request(DELETE_SHARES_FUNCTION, envelope)?;
        let receipts = body.objects_for_parameter(RECEIPT_PARAM)
            .into_iter()
            .map(|e| e.try_into())
            .collect::<anyhow::Result<HashSet<Receipt>>>()?;
        Ok(Self::new(id, key, receipts))
    }
}

impl TryFrom<Envelope> for DeleteSharesRequest {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for DeleteSharesRequest {}

//
// Response
//


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteSharesResponse {
    id: ARID,
}

impl DeleteSharesResponse {
    pub fn new(
        id: ARID,
    ) -> Self {
        Self {id}
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }
}

impl EnvelopeEncodable for DeleteSharesResponse {
    fn envelope(self) -> Envelope {
        response_envelope(self.id, None)
    }
}

impl From<DeleteSharesResponse> for Envelope {
    fn from(value: DeleteSharesResponse) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for DeleteSharesResponse {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, _result) = parse_response(envelope)?;
        Ok(Self::new(id))
    }
}

impl TryFrom<Envelope> for DeleteSharesResponse {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for DeleteSharesResponse {}

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

    fn receipt_1() -> Receipt {
        Receipt::new(&user_id(), b"data_1")
    }

    fn receipt_2() -> Receipt {
        Receipt::new(&user_id(), b"data_2")
    }

    #[test]
    fn test_request() {
        let private_key = PrivateKeyBase::new();
        let key = private_key.public_keys();

        let receipts = vec![receipt_1(), receipt_2()].into_iter().collect();

        let request = DeleteSharesRequest::new(id(), key, receipts);
        let request_envelope = request.clone().envelope();
        assert_eq!(request_envelope.format(),
        indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"deleteShares"» [
                ❰"key"❱: PublicKeyBase
                ❰"receipt"❱: Bytes(32) [
                    'isA': "Receipt"
                ]
                ❰"receipt"❱: Bytes(32) [
                    'isA': "Receipt"
                ]
            ]
        ]
        "#}.trim()
        );
        let decoded = DeleteSharesRequest::try_from(request_envelope).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let response = DeleteSharesResponse::new(id());
        let response_envelope = response.clone().envelope();
        assert_eq!(response_envelope.format(),
        indoc! {r#"
        response(ARID(8712dfac)) [
            'result': 'OK'
        ]
        "#}.trim()
        );
        let decoded = DeleteSharesResponse::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);
    }
}
