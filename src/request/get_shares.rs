use std::collections::{HashMap, HashSet};

use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;
use bytes::Bytes;

use crate::{receipt::Receipt, GET_SHARES_FUNCTION, RECEIPT_PARAM};

use super::{parse_request, parse_response, request_body, request_envelope, response_envelope};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSharesRequest {
    id: ARID,
    key: PublicKeyBase,
    receipts: HashSet<Receipt>,
}

impl GetSharesRequest {
    pub fn new<'a>(
        key: impl AsRef<PublicKeyBase>,
        receipts: impl IntoIterator<Item = &'a Receipt>,
    ) -> Self {
        Self::new_opt(
            ARID::new(),
            key.as_ref().clone(),
            receipts.into_iter().cloned().collect(),
        )
    }

    pub fn new_opt(id: ARID, key: PublicKeyBase, receipts: HashSet<Receipt>) -> Self {
        Self { id, key, receipts }
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

impl EnvelopeEncodable for GetSharesRequest {
    fn envelope(self) -> Envelope {
        let mut body = request_body(GET_SHARES_FUNCTION, self.key);
        for receipt in self.receipts {
            body = body.add_parameter(RECEIPT_PARAM, receipt);
        }
        request_envelope(self.id, body)
    }
}

impl From<GetSharesRequest> for Envelope {
    fn from(value: GetSharesRequest) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for GetSharesRequest {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, key, body) = parse_request(GET_SHARES_FUNCTION, envelope)?;
        let receipts = body
            .objects_for_parameter(RECEIPT_PARAM)
            .into_iter()
            .map(|e| e.try_into())
            .collect::<anyhow::Result<HashSet<Receipt>>>()?;
        Ok(Self::new_opt(id, key, receipts))
    }
}

impl TryFrom<Envelope> for GetSharesRequest {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for GetSharesRequest {}

//
// Response
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSharesResponse {
    id: ARID,
    receipt_to_data: HashMap<Receipt, Bytes>,
}

impl GetSharesResponse {
    pub fn new(id: ARID, receipt_to_data: HashMap<Receipt, Bytes>) -> Self {
        Self {
            id,
            receipt_to_data,
        }
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }

    pub fn receipt_to_data(&self) -> &HashMap<Receipt, Bytes> {
        &self.receipt_to_data
    }

    pub fn data_for_receipt(&self, receipt: &Receipt) -> Option<&Bytes> {
        self.receipt_to_data.get(receipt)
    }
}

impl EnvelopeEncodable for GetSharesResponse {
    fn envelope(self) -> Envelope {
        let mut result = known_values::OK_VALUE.envelope();
        for (receipt, data) in self.receipt_to_data {
            result = result.add_assertion(receipt, data);
        }
        response_envelope(self.id, Some(result))
    }
}

impl From<GetSharesResponse> for Envelope {
    fn from(value: GetSharesResponse) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for GetSharesResponse {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, result) = parse_response(envelope)?;
        let mut receipt_to_data = HashMap::new();
        for assertion in result.assertions() {
            let receipt: Receipt = assertion.expect_predicate()?.try_into()?;
            let data: Bytes = assertion.expect_object()?.try_into()?;
            receipt_to_data.insert(receipt, data);
        }
        Ok(Self::new(id, receipt_to_data))
    }
}

impl TryFrom<Envelope> for GetSharesResponse {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for GetSharesResponse {}

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

    fn user_id() -> ARID {
        ARID::from_data_ref(hex_literal::hex!(
            "8712dfac3d0ebfa910736b2a9ee39d4b68f64222a77bcc0074f3f5f1c9216d30"
        ))
        .unwrap()
    }

    fn data_1() -> Bytes {
        Bytes::from_static(b"data_1")
    }

    fn receipt_1() -> Receipt {
        Receipt::new(&user_id(), data_1())
    }

    fn data_2() -> Bytes {
        Bytes::from_static(b"data_2")
    }

    fn receipt_2() -> Receipt {
        Receipt::new(&user_id(), data_2())
    }

    #[test]
    fn test_request() {
        let private_key = PrivateKeyBase::new();
        let key = private_key.public_keys();

        let receipts = vec![receipt_1(), receipt_2()].into_iter().collect();

        let request = GetSharesRequest::new_opt(id(), key, receipts);
        let request_envelope = request.clone().envelope();
        assert_eq!(
            request_envelope.format(),
            indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"getShares"» [
                ❰"key"❱: PublicKeyBase
                ❰"receipt"❱: Bytes(32) [
                    'isA': "Receipt"
                ]
                ❰"receipt"❱: Bytes(32) [
                    'isA': "Receipt"
                ]
            ]
        ]
        "#}
            .trim()
        );
        let decoded = GetSharesRequest::try_from(request_envelope).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let receipts_to_data = vec![(receipt_1(), data_1()), (receipt_2(), data_2())]
            .into_iter()
            .collect();
        let response = GetSharesResponse::new(id(), receipts_to_data);
        let response_envelope = response.clone().envelope();
        assert_eq!(
            response_envelope.format(),
            indoc! {r#"
        response(ARID(8712dfac)) [
            'result': 'OK' [
                Bytes(32) [
                    'isA': "Receipt"
                ]
                : Bytes(6)
                Bytes(32) [
                    'isA': "Receipt"
                ]
                : Bytes(6)
            ]
        ]
        "#}
            .trim()
        );
        let decoded = GetSharesResponse::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);
    }
}
