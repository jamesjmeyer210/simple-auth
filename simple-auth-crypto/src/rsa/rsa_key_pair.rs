use openssl::encrypt::Encrypter;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::{Padding, Rsa};
use openssl::x509::{X509, X509Builder, X509NameBuilder};

pub(crate) struct RsaKeyPair
{
    public: Rsa<Public>,
    private: Rsa<Private>
}

impl RsaKeyPair {
    const PK_SIZE: u32 = 4096;

    pub fn generate() -> Self {
        let private = Rsa::generate(Self::PK_SIZE).unwrap();
        let n = private.n().clone().to_owned().unwrap();
        let e = private.e().clone().to_owned().unwrap();

        let public = Rsa::from_public_components(n, e).unwrap();

        Self {
            public,
            private
        }
    }

    pub fn public(&self) -> PKey<Public> {
        PKey::from_rsa(self.public.clone()).unwrap()
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let private = self.private();
        let mut encrypter = Encrypter::new(&private).unwrap();
        encrypter.set_rsa_padding(Padding::PKCS1).unwrap();

        let buffer_len = encrypter.encrypt_len(data).unwrap();
        let mut encrypted = vec![0; buffer_len];

        let encrypted_len = encrypter.encrypt(data, &mut encrypted).unwrap();
        encrypted.truncate(encrypted_len);
        encrypted
    }

    pub fn to_x509(&self) -> X509 {
        let mut name_builder = X509NameBuilder::new().unwrap();
        name_builder.append_entry_by_text("C", "US").unwrap();
        name_builder.append_entry_by_text("O", "Simple Auth Org").unwrap();
        name_builder.append_entry_by_text("CN", "localhost").unwrap();
        let x509_name = name_builder.build();

        let mut builder = X509Builder::new().unwrap();
        builder.set_subject_name(&x509_name).unwrap();

        builder.set_pubkey(&self.public()).unwrap();
        builder.sign(&self.private(), MessageDigest::sha256()).unwrap();
        builder.build()
    }

    fn private(&self) -> PKey<Private> {
        PKey::from_rsa(self.private.clone()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    use super::RsaKeyPair;

    #[test]
    fn generate_returns_key_pair() {
        let pair = RsaKeyPair::generate();

        assert_eq!(RsaKeyPair::PK_SIZE/8, pair.private.size());

        let pk = pair.public();
        assert!(pk.rsa().is_ok());

        let encrypted = pair.encrypt(b"Secret message.");
        assert!(encrypted.len() > 1);
    }

    #[test]
    fn generate_cert_returns_x509() {
        let pair = RsaKeyPair::generate();
        let x509 = pair.to_x509();

        assert!(x509.public_key().is_ok());

        let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let t = t.as_millis();

        let txt = x509.to_text().unwrap();
        fs::write(format!("{}.txt", t), txt).unwrap();

        let pem = x509.to_pem().unwrap();
        fs::write(format!("{}.pem", t), pem).unwrap();
    }
}