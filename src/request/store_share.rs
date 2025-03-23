use bc_envelope::prelude::*;
use anyhow::{Error, Result};
use gstp::prelude::*;

use crate::{STORE_SHARE_FUNCTION, DATA_PARAM, receipt::Receipt, util::{Abbrev, FlankedFunction}};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreShare(ByteString);

impl StoreShare {
    pub fn new(data: impl Into<ByteString>) -> Self {
        Self(data.into())
    }

    pub fn data(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<StoreShare> for Expression {
    fn from(value: StoreShare) -> Self {
        Expression::new(STORE_SHARE_FUNCTION)
            .with_parameter(DATA_PARAM, value.0)
    }
}

impl TryFrom<Expression> for StoreShare {
    type Error = Error;

    fn try_from(expression: Expression) -> Result<Self> {
        Ok(Self::new(expression.extract_object_for_parameter::<ByteString>(DATA_PARAM)?))
    }
}

impl std::fmt::Display for StoreShare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}",
            "storeShare".flanked_function(),
            ByteString::from(self.data()).abbrev(),
        ))
    }
}

//
// Response
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreShareResult(Receipt);

impl StoreShareResult {
    pub fn new(receipt: Receipt) -> Self {
        Self(receipt)
    }

    pub fn receipt(&self) -> &Receipt {
        &self.0
    }
}

impl From<StoreShareResult> for Envelope {
    fn from(value: StoreShareResult) -> Self {
        value.0.into_envelope()
    }
}

impl TryFrom<Envelope> for StoreShareResult {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        Ok(Self::new(Receipt::try_from(envelope)?))
    }
}

impl TryFrom<SealedResponse> for StoreShareResult {
    type Error = Error;

    fn try_from(response: SealedResponse) -> Result<Self> {
        response.result()?.clone().try_into()
    }
}

impl std::fmt::Display for StoreShareResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} OK receipt {}",
            "storeShare".flanked_function(),
            self.receipt().abbrev()
        ))
    }
}

#[cfg(test)]
mod tests {
    use bc_components::XID;
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_request() {
        bc_envelope::register_tags();

        let data = b"data";
        let request = StoreShare::new(data);
        let expression: Expression = request.clone().into();
        let request_envelope = expression.to_envelope();
        // println!("{}", request_envelope.format());
        assert_eq!(request_envelope.format(),
        indoc! {r#"
        «"storeShare"» [
            ❰"data"❱: Bytes(4)
        ]
        "#}.trim());
        let decoded_expression = Expression::try_from(request_envelope).unwrap();
        let decoded = StoreShare::try_from(decoded_expression).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        bc_envelope::register_tags();

        let user_id = XID::from_data_ref(hex_literal::hex!("8712dfac3d0ebfa910736b2a9ee39d4b68f64222a77bcc0074f3f5f1c9216d30")).unwrap();
        let data = b"data";
        let receipt = Receipt::new(user_id, data);
        let result = StoreShareResult::new(receipt);
        let result_envelope = result.to_envelope();
        // println!("{}", result_envelope.format());
        assert_eq!(result_envelope.format(),
        indoc! {r#"
        Bytes(32) [
            'isA': "Receipt"
        ]
        "#}.trim());
        let decoded =StoreShareResult::try_from(result_envelope).unwrap();
        assert_eq!(result, decoded);
    }
}
