use std::collections::{HashMap, HashSet};

use bc_envelope::prelude::*;
use anyhow::{Error, Result};

use crate::{receipt::Receipt, GET_SHARES_FUNCTION, RECEIPT_PARAM, util::{Abbrev, FlankedFunction}};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetShares (HashSet<Receipt>);

impl GetShares {
    pub fn new<I, T>(iterable: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Clone + Into<Receipt>,
    {
        Self(iterable.into_iter().map(|item| item.clone().into()).collect())
    }

    pub fn new_all_shares() -> Self {
        Self(HashSet::new())
    }

    pub fn receipts(&self) -> &HashSet<Receipt> {
        &self.0
    }
}

impl From<GetShares> for Expression {
    fn from(value: GetShares) -> Self {
        let mut expression = Expression::new(GET_SHARES_FUNCTION);
        for receipt in value.0.into_iter() {
            expression = expression.with_parameter(RECEIPT_PARAM, receipt);
        }
        expression
    }
}

impl TryFrom<Expression> for GetShares {
    type Error = Error;

    fn try_from(expression: Expression) -> Result<Self> {
        let receipts = expression
            .objects_for_parameter(RECEIPT_PARAM)
            .into_iter()
            .map(|parameter| parameter.try_into())
            .collect::<Result<HashSet<Receipt>>>()?;
        Ok(Self(receipts))
    }
}

impl std::fmt::Display for GetShares {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}",
            "getShares".flanked_function(),
            self.receipts().abbrev(),
        ))
    }
}

//
// Response
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSharesResult(HashMap<Receipt, ByteString>);

impl GetSharesResult {
    pub fn new(receipt_to_data: HashMap<Receipt, ByteString>) -> Self {
        Self(receipt_to_data)
    }

    pub fn receipt_to_data(&self) -> &HashMap<Receipt, ByteString> {
        &self.0
    }

    pub fn data_for_receipt(&self, receipt: &Receipt) -> Option<&ByteString> {
        self.0.get(receipt)
    }
}

impl From<GetSharesResult> for Envelope {
    fn from(value: GetSharesResult) -> Self {
        let mut result = known_values::OK_VALUE.to_envelope();
        for (receipt, data) in value.0 {
            result = result.add_assertion(receipt, data);
        }
        result
    }
}

impl TryFrom<Envelope> for GetSharesResult {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        let mut receipt_to_data = HashMap::new();
        for assertion in envelope.assertions() {
            let receipt = Receipt::try_from(assertion.try_predicate()?)?;
            let object = assertion.try_object()?;
            let data = ByteString::try_from(object)?;
            receipt_to_data.insert(receipt, data);
        }
        Ok(Self::new(receipt_to_data))
    }
}

impl TryFrom<SealedResponse> for GetSharesResult {
    type Error = Error;

    fn try_from(response: SealedResponse) -> Result<Self> {
        response.result()?.clone().try_into()
    }
}

impl std::fmt::Display for GetSharesResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let abbrevable: HashMap<Receipt, ByteString> = self.receipt_to_data()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        f.write_fmt(format_args!("{} OK {}",
            "getShares".flanked_function(),
            abbrevable.abbrev()
        ))
    }
}

#[cfg(test)]
mod tests {
    use bc_components::ARID;
    use indoc::indoc;

    use super::*;

    fn user_id() -> ARID {
        ARID::from_data_ref(hex_literal::hex!(
            "8712dfac3d0ebfa910736b2a9ee39d4b68f64222a77bcc0074f3f5f1c9216d30"
        ))
        .unwrap()
    }

    fn data_1() -> ByteString {
        b"data_1".to_vec().into()
    }

    fn receipt_1() -> Receipt {
        Receipt::new(&user_id(), data_1())
    }

    fn data_2() -> ByteString {
        b"data_2".to_vec().into()
    }

    fn receipt_2() -> Receipt {
        Receipt::new(&user_id(), data_2())
    }

    #[test]
    fn test_request() {
        let receipts = vec![receipt_1(), receipt_2()];

        let request = GetShares::new(receipts);
        let expression: Expression = request.clone().into();
        let request_envelope = expression.to_envelope();
        // println!("{}", request_envelope.format());
        assert_eq!(request_envelope.format(), indoc! {r#"
        «"getShares"» [
            ❰"receipt"❱: Bytes(32) [
                'isA': "Receipt"
            ]
            ❰"receipt"❱: Bytes(32) [
                'isA': "Receipt"
            ]
        ]
        "#}.trim());
        let decoded_expression = Expression::try_from(request_envelope).unwrap();
        let decoded = GetShares::try_from(decoded_expression).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let receipts_to_data = vec![(receipt_1(), data_1()), (receipt_2(), data_2())]
            .into_iter()
            .collect();
        let response = GetSharesResult::new(receipts_to_data);
        let response_envelope = response.to_envelope();
        // println!("{}", response_envelope.format());
        assert_eq!(response_envelope.format(), indoc! {r#"
        'OK' [
            Bytes(32) [
                'isA': "Receipt"
            ]
            : Bytes(6)
            Bytes(32) [
                'isA': "Receipt"
            ]
            : Bytes(6)
        ]
        "#}.trim());
        let decoded = GetSharesResult::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);
    }
}
