use bc_envelope::prelude::*;

use crate::{DELETE_ACCOUNT_FUNCTION, Error, Result, util::FlankedFunction};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteAccount();

impl DeleteAccount {
    pub fn new() -> Self {
        Self()
    }
}

impl Default for DeleteAccount {
    fn default() -> Self {
        Self::new()
    }
}

impl From<DeleteAccount> for Expression {
    fn from(_: DeleteAccount) -> Self {
        Expression::new(DELETE_ACCOUNT_FUNCTION)
    }
}

impl TryFrom<Expression> for DeleteAccount {
    type Error = Error;

    fn try_from(_: Expression) -> Result<Self> {
        Ok(Self::new())
    }
}

impl std::fmt::Display for DeleteAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", "deleteAccount".flanked_function()))
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_request() {
        bc_envelope::register_tags();

        let request = DeleteAccount::new();
        let expression: Expression = request.clone().into();
        let request_envelope = expression.to_envelope();

        // println!("{}", request_envelope.format());
        #[rustfmt::skip]
        assert_eq!(request_envelope.format(), indoc! {r#"
            «"deleteAccount"»
        "#}.trim());
        let decoded_expression =
            Expression::try_from(request_envelope).unwrap();
        let decoded = DeleteAccount::try_from(decoded_expression).unwrap();
        assert_eq!(request, decoded);
    }
}
