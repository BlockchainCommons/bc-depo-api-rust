use std::collections::HashSet;

use bc_envelope::prelude::*;

use crate::{
    DELETE_SHARES_FUNCTION, Error, RECEIPT_PARAM, RECEIPT_PARAM_NAME, Result,
    receipt::Receipt,
    util::{Abbrev, FlankedFunction},
};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteShares(HashSet<Receipt>);

impl DeleteShares {
    pub fn new<I, T>(iterable: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Clone + Into<Receipt>,
    {
        Self(
            iterable
                .into_iter()
                .map(|item| item.clone().into())
                .collect(),
        )
    }

    pub fn receipts(&self) -> &HashSet<Receipt> {
        &self.0
    }
}

impl From<DeleteShares> for Expression {
    fn from(value: DeleteShares) -> Self {
        let mut expression = Expression::new(DELETE_SHARES_FUNCTION);
        for receipt in value.0.into_iter() {
            expression = expression.with_parameter(RECEIPT_PARAM, receipt);
        }
        expression
    }
}

impl TryFrom<Expression> for DeleteShares {
    type Error = Error;

    fn try_from(expression: Expression) -> Result<Self> {
        let receipts = expression
            .objects_for_parameter(RECEIPT_PARAM)
            .into_iter()
            .map(|parameter| {
                parameter.try_into().map_err(|e| Error::InvalidParameter {
                    parameter: RECEIPT_PARAM_NAME.to_string(),
                    message: format!("failed to convert to Receipt: {}", e),
                })
            })
            .collect::<Result<HashSet<Receipt>>>()?;
        Ok(Self::new(receipts))
    }
}

impl std::fmt::Display for DeleteShares {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {}",
            "deleteShares".flanked_function(),
            self.receipts().abbrev()
        ))
    }
}

#[cfg(test)]
mod tests {
    use bc_components::XID;
    use indoc::indoc;

    use super::*;

    fn user_id() -> XID {
        XID::from_data_ref(hex_literal::hex!(
            "8712dfac3d0ebfa910736b2a9ee39d4b68f64222a77bcc0074f3f5f1c9216d30"
        ))
        .unwrap()
    }

    fn receipt_1() -> Receipt {
        Receipt::new(user_id(), b"data_1")
    }

    fn receipt_2() -> Receipt {
        Receipt::new(user_id(), b"data_2")
    }

    #[test]
    fn test_request() {
        bc_envelope::register_tags();

        let receipts = vec![receipt_1(), receipt_2()];

        let request = DeleteShares::new(receipts);
        let expression: Expression = request.clone().into();
        let request_envelope = expression.to_envelope();
        // println!("{}", request_envelope.format());
        #[rustfmt::skip]
        assert_eq!(request_envelope.format(), indoc! {r#"
            «"deleteShares"» [
                ❰"receipt"❱: Bytes(32) [
                    'isA': "Receipt"
                ]
                ❰"receipt"❱: Bytes(32) [
                    'isA': "Receipt"
                ]
            ]
        "#}.trim());
        let decoded_expression =
            Expression::try_from(request_envelope).unwrap();
        let decoded = DeleteShares::try_from(decoded_expression).unwrap();
        assert_eq!(request, decoded);
    }
}
