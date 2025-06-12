use anyhow::{Error, Result};
use bc_components::XID;
use bc_envelope::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Receipt(Digest);

pub const RECEIPT_TYPE: &str = "Receipt";

impl Receipt {
    pub fn new(user_id: XID, data: impl AsRef<[u8]>) -> Self {
        Self(Digest::from_image_parts(&[user_id.data(), data.as_ref()]))
    }
}

impl std::ops::Deref for Receipt {
    type Target = Digest;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl std::fmt::Debug for Receipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Receipt({})", hex::encode(&self.0))
    }
}

impl From<Receipt> for Envelope {
    fn from(receipt: Receipt) -> Self {
        Envelope::new(CBOR::to_byte_string(receipt.0.clone()))
            .add_type(RECEIPT_TYPE)
    }
}

impl TryFrom<Envelope> for Receipt {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        envelope.check_type_envelope(RECEIPT_TYPE)?;
        let bytes: ByteString = envelope.extract_subject()?;
        let digest = Digest::from_data_ref(bytes.data())?;
        Ok(Self(digest))
    }
}

impl From<&Receipt> for Receipt {
    fn from(receipt: &Receipt) -> Self { receipt.clone() }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_receipt() {
        let user_id = XID::from_data_ref(hex!(
            "3eadf5bf7a4da69f824be029d2d0ece06fcb3aca7dd85d402b661f7b48f18294"
        ))
        .unwrap();
        let receipt = Receipt::new(user_id, b"data");
        assert_eq!(
            format!("{:?}", receipt),
            "Receipt(12bd077763220d3223f6cd74f4d51103f29c7ba70b68765cd8ee13c84ee50152)"
        );

        let envelope = receipt.clone().to_envelope();
        assert_eq!(
            format!("{}", envelope.ur_string()),
            "ur:envelope/lftpsohdcxbgryatktiacpbteycnynsnjywktlbyaxwznskgosbdiskohhtpwybwspglvwadgmoyadtpsoiogmihiaihinjojyamdwplrf"
        );
        #[rustfmt::skip]
        assert_eq!(envelope.format(), indoc!{r#"
            Bytes(32) [
                'isA': "Receipt"
            ]
        "#}.trim());

        let receipt_2 = Receipt::try_from(envelope).unwrap();
        assert_eq!(receipt, receipt_2);
    }
}
