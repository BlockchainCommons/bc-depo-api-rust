use bc_components::ARID;
use bc_envelope::prelude::*;
use bytes::Bytes;
use anyhow::{Error, Result};

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
        let bytes: Bytes = envelope.extract_subject()?;
        let digest = Digest::from_data_ref(&bytes)?;
        Ok(Self(digest))
    }
}

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

        let envelope = receipt.clone().to_envelope();
        assert_eq!(format!("{}", envelope.ur_string()), "ur:envelope/lftpsohdcxbgryatktiacpbteycnynsnjywktlbyaxwznskgosbdiskohhtpwybwspglvwadgmoyadtpsoiogmihiaihinjojyamdwplrf");
        assert_eq!(envelope.format(),
        indoc!{r#"
        Bytes(32) [
            'isA': "Receipt"
        ]
        "#}.trim());

        let receipt_2: Receipt = envelope.try_into().unwrap();
        assert_eq!(receipt, receipt_2);
    }
}
