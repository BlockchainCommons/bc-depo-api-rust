use std::collections::HashSet;

use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;
use anyhow::{Error, Result};

use crate::{receipt::Receipt, DELETE_SHARES_FUNCTION, RECEIPT_PARAM, util::{Abbrev, FlankedFunction}};

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

impl From<DeleteSharesRequest> for Envelope {
    fn from(value: DeleteSharesRequest) -> Self {
        let mut body = Envelope::new_function(DELETE_SHARES_FUNCTION);
        let id = value.id().clone();
        for receipt in value.receipts.into_iter() {
            body = body.add_parameter(RECEIPT_PARAM, receipt);
        }
        body.into_transaction_request(id, &value.key)
    }
}

impl TryFrom<Envelope> for DeleteSharesRequest {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        let (id, key, body, _) = envelope.parse_transaction_request(Some(&DELETE_SHARES_FUNCTION))?;
        Self::from_body(id, key, body)
    }
}

impl std::fmt::Display for DeleteSharesRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {} {} key {}",
            self.id().abbrev(),
            "deleteShares".flanked_function(),
            self.receipts().abbrev(),
            self.key().abbrev()
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

    fn receipt_1() -> Receipt {
        Receipt::new(&user_id(), b"data_1")
    }

    fn receipt_2() -> Receipt {
        Receipt::new(&user_id(), b"data_2")
    }

    #[test]
    fn test_request() {
        let private_key = PrivateKeyBase::new();
        let key = private_key.public_key();

        let receipts = vec![receipt_1(), receipt_2()].into_iter().collect();

        let request = DeleteSharesRequest::from_fields(id(), key, receipts);
        let request_envelope = request.to_envelope();
        assert_eq!(
            request_envelope.format(),
            indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"deleteShares"» [
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
}
