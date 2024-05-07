use std::collections::{HashMap, HashSet};

use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;
use bytes::Bytes;
use anyhow::{Error, Result};

use crate::{receipt::Receipt, GET_SHARES_FUNCTION, RECEIPT_PARAM, util::{Abbrev, FlankedFunction}};

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
    pub fn from_fields(id: ARID, key: PublicKeyBase, receipts: HashSet<Receipt>) -> Self {
        Self { id, key, receipts }
    }

    pub fn new<'a>(
        key: impl AsRef<PublicKeyBase>,
        receipts: impl IntoIterator<Item = &'a Receipt>,
    ) -> Self {
        Self::from_fields(
            ARID::new(),
            key.as_ref().clone(),
            receipts.into_iter().cloned().collect(),
        )
    }

    pub fn from_body(id: ARID, key: PublicKeyBase, body: Envelope) -> Result<Self> {
        let receipts = body
            .objects_for_parameter(RECEIPT_PARAM)
            .into_iter()
            .map(|e| e.try_into())
            .collect::<Result<HashSet<Receipt>>>()?;
        Ok(Self::from_fields(id, key, receipts))
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

impl From<GetSharesRequest> for Envelope {
    fn from(value: GetSharesRequest) -> Self {
        let mut body = Envelope::new_function(GET_SHARES_FUNCTION);
        let id = value.id().clone();
        for receipt in value.receipts.into_iter() {
            body = body.add_parameter(RECEIPT_PARAM, receipt);
        }
        body.into_transaction_request(id, &value.key)
    }
}

impl TryFrom<Envelope> for GetSharesRequest {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        let (id, key, body, _) = envelope.parse_transaction_request(Some(&GET_SHARES_FUNCTION))?;
        Self::from_body(id, key, body)
    }
}

impl std::fmt::Display for GetSharesRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {} {} key {}",
            self.id().abbrev(),
            "getShares".flanked_function(),
            self.receipts().abbrev(),
            self.key().abbrev()
        ))
    }
}

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

    pub fn data_for_receipt(&self, receipt: &Receipt) -> Option<Bytes> {
        self.receipt_to_data.get(receipt).cloned()
    }
}

impl From<GetSharesResponse> for Envelope {
    fn from(value: GetSharesResponse) -> Self {
        let mut result = known_values::OK_VALUE.to_envelope();
        for (receipt, data) in value.receipt_to_data {
            result = result.add_assertion(receipt, data);
        }
        result.into_success_response(value.id)
    }
}

impl TryFrom<Envelope> for GetSharesResponse {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        let (result, id) = envelope.parse_success_response(None)?;
        let mut receipt_to_data = HashMap::new();
        for assertion in result.assertions() {
            let receipt: Receipt = assertion.try_predicate()?.try_into()?;
            // let receipt: Receipt = (&assertion.try_predicate()?).try_into()?;
            let data: Bytes = assertion.try_object()?.try_into()?;
            receipt_to_data.insert(receipt, data);
        }
        Ok(Self::new(id, receipt_to_data))
    }
}

impl std::fmt::Display for GetSharesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {} OK {}",
            self.id().abbrev(),
            "getShares".flanked_function(),
            self.receipt_to_data().abbrev()
        ))
    }
}

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
        let key = private_key.public_key();

        let receipts = vec![receipt_1(), receipt_2()].into_iter().collect();

        let request = GetSharesRequest::from_fields(id(), key, receipts);
        let request_envelope = request.to_envelope();
        assert_eq!(
            request_envelope.format(),
            indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"getShares"» [
                ❰"receipt"❱: Bytes(32) [
                    'isA': "Receipt"
                ]
                ❰"receipt"❱: Bytes(32) [
                    'isA': "Receipt"
                ]
            ]
            'senderPublicKey': PublicKeyBase
        ]
        "#}
            .trim()
        );
        let decoded = request_envelope.try_into().unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let receipts_to_data = vec![(receipt_1(), data_1()), (receipt_2(), data_2())]
            .into_iter()
            .collect();
        let response = GetSharesResponse::new(id(), receipts_to_data);
        let response_envelope = response.to_envelope();
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
