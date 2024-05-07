use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;
use bytes::Bytes;
use anyhow::{Error, Result};

use crate::{STORE_SHARE_FUNCTION, DATA_PARAM, receipt::Receipt, util::{Abbrev, FlankedFunction}};

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
    pub fn from_fields(id: ARID, key: PublicKeyBase, data: Bytes) -> Self {
        Self {
            id,
            key,
            data,
        }
    }

    pub fn new(key: impl AsRef<PublicKeyBase>, data: impl AsRef<[u8]>) -> Self {
        Self::from_fields(ARID::new(), key.as_ref().clone(), Bytes::copy_from_slice(data.as_ref()))
    }

    pub fn from_body(id: ARID, key: PublicKeyBase, body: Envelope) -> Result<Self> {
        let data: Bytes = body.extract_object_for_parameter(DATA_PARAM)?;
        Ok(Self::from_fields(id, key, data))
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

impl From<StoreShareRequest> for Envelope {
    fn from(value: StoreShareRequest) -> Self {
        let id = value.id().clone();
        Envelope::new_function(STORE_SHARE_FUNCTION)
            .add_parameter(DATA_PARAM, value.data)
            .into_transaction_request(id, &value.key)
    }
}

impl TryFrom<&Envelope> for StoreShareRequest {
    type Error = Error;

    fn try_from(envelope: &Envelope) -> Result<Self> {
        let (id, key, body, _) = envelope.parse_transaction_request(Some(&STORE_SHARE_FUNCTION))?;
        Self::from_body(id, key, body)
    }
}

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

impl From<StoreShareResponse> for Envelope {
    fn from(value: StoreShareResponse) -> Self {
        value.receipt.to_envelope().into_success_response(value.id)
    }
}

impl TryFrom<Envelope> for StoreShareResponse {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        let (result, id) = envelope.parse_success_response(None)?;
        Ok(Self::new(id, result.try_into()?))
    }
}

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
        let key = private_key.public_key();

        let request = StoreShareRequest::from_fields(id(), key, Bytes::from_static(b"data"));
        let request_envelope = request.to_envelope();
        assert_eq!(request_envelope.format(),
        indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"storeShare"» [
                ❰"data"❱: Bytes(4)
            ]
            'senderPublicKey': PublicKeyBase
        ]
        "#}.trim()
        );
        let decoded = StoreShareRequest::try_from(&request_envelope).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let response = StoreShareResponse::new(id(), receipt_1());
        let response_envelope = response.to_envelope();
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
