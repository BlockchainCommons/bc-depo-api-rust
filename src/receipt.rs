use bc_components::{ARID, Digest};
use bc_envelope::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Receipt(Digest);

pub const RECEIPT_TYPE: &str = "Receipt";

impl Receipt {
    pub fn new(user_id: &ARID, data: impl AsRef<[u8]>) -> Self {
        Self(Digest::from_image_parts(&[user_id.data(), data.as_ref()]))
    }
}

impl std::ops::Deref for Receipt {
    type Target = Digest;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for Receipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Receipt({})", hex::encode(&self.0))
    }
}

impl EnvelopeEncodable for Receipt {
    fn envelope(self) -> Envelope {
        Envelope::new(CBOR::byte_string(self.0))
            .add_type(RECEIPT_TYPE)
    }
}

impl From<Receipt> for Envelope {
    fn from(receipt: Receipt) -> Self {
        receipt.envelope()
    }
}

impl EnvelopeDecodable for Receipt {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        envelope.clone().check_type_envelope(RECEIPT_TYPE)?;
        let cbor: CBOR = envelope.extract_subject()?;
        let bytes = cbor.expect_byte_string()?;
        let digest = Digest::from_data_ref(&bytes)?;
        Ok(Self(digest))
    }
}

impl TryFrom<Envelope> for Receipt {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        Self::from_envelope(envelope)
    }
}

impl EnvelopeCodable for Receipt { }

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use indoc::indoc;

    #[test]
    fn test_receipt() {
        let user_id = ARID::from_data_ref(hex!("3eadf5bf7a4da69f824be029d2d0ece06fcb3aca7dd85d402b661f7b48f18294")).unwrap();
        let receipt = Receipt::new(&user_id, b"data");
        assert_eq!(format!("{:?}", receipt), "Receipt(12bd077763220d3223f6cd74f4d51103f29c7ba70b68765cd8ee13c84ee50152)");

        let envelope = receipt.clone().envelope();
        assert_eq!(format!("{}", envelope.ur_string()), "ur:envelope/lftpsohdcxbgryatktiacpbteycnynsnjywktlbyaxwznskgosbdiskohhtpwybwspglvwadgmoyadtpsoiogmihiaihinjojyamdwplrf");
        assert_eq!(envelope.format(),
        indoc!{r#"
        Bytes(32) [
            'isA': "Receipt"
        ]
        "#}.trim());

        let receipt_2 = Receipt::from_envelope(envelope.clone()).unwrap();
        assert_eq!(receipt, receipt_2);
    }
}
