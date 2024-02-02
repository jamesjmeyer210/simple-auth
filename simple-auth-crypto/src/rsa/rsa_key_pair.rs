use openssl::encrypt::Encrypter;
use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::{Padding, Rsa};

struct RsaKeyPair
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

    fn private(&self) -> PKey<Private> {
        PKey::from_rsa(self.private.clone()).unwrap()
    }
}

#[cfg(test)]
mod test {
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
}