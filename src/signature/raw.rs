use crate::error::{Error, Result};
use bytemuck::{Pod, Zeroable};
use bytes::Bytes;
use std::borrow::Cow;

const SIG_MAGIC: &[u8] = b"~Module signature appended~\n";

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct RawKernelObjectSignatureHeader {
    algorithm: u8,
    hash: u8,
    id_type: u8,
    signer_length: u8,
    key_id_length: u8,
    _pad: [u8; 3],
    signature_length_be: u32,
}

impl RawKernelObjectSignatureHeader {
    pub fn signature_length(&self) -> u32 {
        if cfg!(target_endian = "little") {
            // when little endian, it is assumed the value is little endian.
            // the converting the value to big endian will result in little endian.
            self.signature_length_be.to_be()
        } else {
            self.signature_length_be
        }
    }
}

#[derive(Clone, Debug)]
pub struct RawKernelObjectSignature {
    pub header: RawKernelObjectSignatureHeader,
    pub signature: Vec<u8>,
    pub key_id: Vec<u8>,
    pub signer: Vec<u8>,
}

impl RawKernelObjectSignature {
    pub fn key_id_str(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.key_id)
    }

    pub fn signer_str(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.signer)
    }
}

impl RawKernelObjectSignature {
    pub fn load(bytes: &Bytes) -> Result<Option<RawKernelObjectSignature>> {
        if bytes.len() < SIG_MAGIC.len() {
            return Ok(None);
        }

        let offset = bytes.len() - SIG_MAGIC.len();
        let magic = &bytes[offset..];
        if magic != SIG_MAGIC {
            return Ok(None);
        }
        let before_magic = &bytes[..offset];
        if before_magic.len() < size_of::<RawKernelObjectSignatureHeader>() {
            return Ok(None);
        }
        let header_offset = before_magic.len() - size_of::<RawKernelObjectSignatureHeader>();
        let header = &before_magic[header_offset..];
        let header = bytemuck::try_pod_read_unaligned::<RawKernelObjectSignatureHeader>(header)
            .map_err(Error::DataDecodeError)?;
        let signature_length = header.signature_length() as usize;
        let total_length =
            signature_length + header.signer_length as usize + header.key_id_length as usize;
        if signature_length == 0 || (total_length > before_magic.len()) {
            return Ok(Some(RawKernelObjectSignature {
                header,
                signature: Vec::new(),
                key_id: Vec::new(),
                signer: Vec::new(),
            }));
        }
        let signature_offset = header_offset - signature_length;
        let signature = &before_magic[signature_offset..signature_offset + signature_length];
        let key_id_offset = signature_offset - header.key_id_length as usize;
        let key_id = &bytes[key_id_offset..key_id_offset + header.key_id_length as usize];
        let signer_offset = key_id_offset - header.signer_length as usize;
        let signer = &bytes[signer_offset..signer_offset + header.signer_length as usize];
        Ok(Some(RawKernelObjectSignature {
            header,
            signature: signature.to_vec(),
            key_id: key_id.to_vec(),
            signer: signer.to_vec(),
        }))
    }
}
