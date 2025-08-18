#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum KeyIdType {
    Pgp,
    X509,
    Pkcs7,
}

impl KeyIdType {
    pub fn from_name(name: &str) -> Option<KeyIdType> {
        match name {
            "PGP" => Some(KeyIdType::Pgp),
            "X509" => Some(KeyIdType::X509),
            "PKCS#7" => Some(KeyIdType::Pkcs7),
            _ => None,
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            KeyIdType::Pgp => "PGP",
            KeyIdType::X509 => "X509",
            KeyIdType::Pkcs7 => "PKCS#7",
        }
    }

    pub fn from_raw(raw: u8) -> Option<KeyIdType> {
        match raw {
            0 => Some(KeyIdType::Pgp),
            1 => Some(KeyIdType::X509),
            2 => Some(KeyIdType::Pkcs7),
            _ => None,
        }
    }

    pub fn to_raw(&self) -> u8 {
        match self {
            KeyIdType::Pgp => 0,
            KeyIdType::X509 => 1,
            KeyIdType::Pkcs7 => 2,
        }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PublicKeyAlgorithm {
    Dsa,
    Rsa,
}

impl PublicKeyAlgorithm {
    pub fn from_name(name: &str) -> Option<PublicKeyAlgorithm> {
        match name {
            "DSA" => Some(PublicKeyAlgorithm::Dsa),
            "RSA" => Some(PublicKeyAlgorithm::Rsa),
            _ => None,
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            PublicKeyAlgorithm::Dsa => "DSA",
            PublicKeyAlgorithm::Rsa => "RSA",
        }
    }

    pub fn from_raw(raw: u8) -> Option<PublicKeyAlgorithm> {
        match raw {
            0 => Some(PublicKeyAlgorithm::Dsa),
            1 => Some(PublicKeyAlgorithm::Rsa),
            _ => None,
        }
    }

    pub fn to_raw(&self) -> u8 {
        match self {
            PublicKeyAlgorithm::Dsa => 0,
            PublicKeyAlgorithm::Rsa => 1,
        }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HashAlgorithm {
    Md4,
    Md5,
    Sha1,
    RipeMd160,
    Sha256,
    Sha384,
    Sha512,
    Sha224,
    Sm3,
}

impl HashAlgorithm {
    pub fn from_name(name: &str) -> Option<HashAlgorithm> {
        match name {
            "MD4" => Some(HashAlgorithm::Md4),
            "MD5" => Some(HashAlgorithm::Md5),
            "SHA1" => Some(HashAlgorithm::Sha1),
            "RIPEMD160" => Some(HashAlgorithm::RipeMd160),
            "SHA256" => Some(HashAlgorithm::Sha256),
            "SHA384" => Some(HashAlgorithm::Sha384),
            "SHA512" => Some(HashAlgorithm::Sha512),
            "SHA224" => Some(HashAlgorithm::Sha224),
            "SM3" => Some(HashAlgorithm::Sm3),
            _ => None,
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            HashAlgorithm::Md4 => "MD4",
            HashAlgorithm::Md5 => "MD5",
            HashAlgorithm::Sha1 => "SHA1",
            HashAlgorithm::RipeMd160 => "RIPEMD160",
            HashAlgorithm::Sha256 => "SHA256",
            HashAlgorithm::Sha384 => "SHA384",
            HashAlgorithm::Sha512 => "SHA512",
            HashAlgorithm::Sha224 => "SHA224",
            HashAlgorithm::Sm3 => "SM3",
        }
    }

    pub fn from_raw(raw: u8) -> Option<HashAlgorithm> {
        match raw {
            0 => Some(HashAlgorithm::Md4),
            1 => Some(HashAlgorithm::Md5),
            2 => Some(HashAlgorithm::Sha1),
            3 => Some(HashAlgorithm::RipeMd160),
            4 => Some(HashAlgorithm::Sha256),
            5 => Some(HashAlgorithm::Sha384),
            6 => Some(HashAlgorithm::Sha512),
            7 => Some(HashAlgorithm::Sha224),
            8 => Some(HashAlgorithm::Sm3),
            _ => None,
        }
    }

    pub fn to_raw(&self) -> u8 {
        match self {
            HashAlgorithm::Md4 => 0,
            HashAlgorithm::Md5 => 1,
            HashAlgorithm::Sha1 => 2,
            HashAlgorithm::RipeMd160 => 3,
            HashAlgorithm::Sha256 => 4,
            HashAlgorithm::Sha384 => 5,
            HashAlgorithm::Sha512 => 6,
            HashAlgorithm::Sha224 => 7,
            HashAlgorithm::Sm3 => 8,
        }
    }
}
