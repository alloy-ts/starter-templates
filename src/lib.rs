use napi::bindgen_prelude::*;
use napi_derive::napi;
use btls::symm::{encrypt, decrypt, Cipher};
use btls::hash::{hash, MessageDigest, hmac_sha1, hmac_sha256, hmac_sha512};
use btls::base64::{encode_block, decode_block};

fn get_cipher(name: &str) -> Result<Cipher> {
    match name.to_lowercase().replace('_', "-").as_str() {
        "aes-128-cbc" | "aes128-cbc" => Ok(Cipher::aes_128_cbc()),
        "aes-256-cbc" | "aes256-cbc" => Ok(Cipher::aes_256_cbc()),
        "aes-128-ctr" | "aes128-ctr" => Ok(Cipher::aes_128_ctr()),
        "aes-256-ctr" | "aes256-ctr" => Ok(Cipher::aes_256_ctr()),
        "aes-128-ecb" | "aes128-ecb" => Ok(Cipher::aes_128_ecb()),
        "aes-256-ecb" | "aes256-ecb" => Ok(Cipher::aes_256_ecb()),
        "des-cbc" | "descbc" => Ok(Cipher::des_cbc()),
        "des-ede3-cbc" | "3des-cbc" => Ok(Cipher::des_ede3_cbc()),
        _ => Err(Error::new(
            Status::InvalidArg,
            format!("Unsupported cipher: {}", name),
        )),
    }
}

fn get_digest(name: &str) -> Result<MessageDigest> {
    match name.to_lowercase().replace('_', "-").as_str() {
        "sha1" | "sha-1" => Ok(MessageDigest::sha1()),
        "sha224" | "sha-224" => Ok(MessageDigest::sha224()),
        "sha256" | "sha-256" => Ok(MessageDigest::sha256()),
        "sha384" | "sha-384" => Ok(MessageDigest::sha384()),
        "sha512" | "sha-512" => Ok(MessageDigest::sha512()),
        _ => Err(Error::new(
            Status::InvalidArg,
            format!("Unsupported digest: {}", name),
        )),
    }
}

#[napi]
pub fn init() {
    btls::init();
}

#[napi]
pub fn version() -> String {
    btls::version::version().to_string()
}

#[napi]
pub fn base64_encode(data: Uint8Array) -> String {
    encode_block(&data)
}

#[napi]
pub fn base64_decode(input: String) -> Result<Buffer> {
    decode_block(&input)
        .map(Buffer::from)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn digest(algorithm: String, data: Uint8Array) -> Result<Buffer> {
    let md = get_digest(&algorithm)?;
    let res = hash(md, &data)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(Buffer::from(res.as_ref()))
}

#[napi]
pub fn hmac(algorithm: String, key: Uint8Array, data: Uint8Array) -> Result<Buffer> {
    match algorithm.to_lowercase().replace('_', "-").as_str() {
        "sha1" | "sha-1" => {
            let res = hmac_sha1(&key, &data)
                .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
            Ok(Buffer::from(res.as_ref()))
        }
        "sha256" | "sha-256" => {
            let res = hmac_sha256(&key, &data)
                .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
            Ok(Buffer::from(res.as_ref()))
        }
        "sha512" | "sha-512" => {
            let res = hmac_sha512(&key, &data)
                .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
            Ok(Buffer::from(res.as_ref()))
        }
        _ => Err(Error::new(
            Status::InvalidArg,
            format!("Unsupported HMAC algorithm: {}", algorithm),
        )),
    }
}

#[napi]
pub fn encrypt_data(
    cipher_name: String,
    key: Uint8Array,
    iv: Option<Uint8Array>,
    data: Uint8Array,
) -> Result<Buffer> {
    let cipher = get_cipher(&cipher_name)?;
    let iv_ref = iv.as_ref().map(|i| i.as_ref());
    let encrypted = encrypt(cipher, &key, iv_ref, &data)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(Buffer::from(encrypted))
}

#[napi]
pub fn decrypt_data(
    cipher_name: String,
    key: Uint8Array,
    iv: Option<Uint8Array>,
    data: Uint8Array,
) -> Result<Buffer> {
    let cipher = get_cipher(&cipher_name)?;
    let iv_ref = iv.as_ref().map(|i| i.as_ref());
    let decrypted = decrypt(cipher, &key, iv_ref, &data)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(Buffer::from(decrypted))
}

#[napi]
pub struct BoringSshCodec {
    cipher_name: String,
    key: Vec<u8>,
    iv: Vec<u8>,
    sequence_number: u32,
    block_size: usize,
}

#[napi]
impl BoringSshCodec {
    #[napi(constructor)]
    pub fn new(
        cipher_name: String,
        key: Uint8Array,
        iv: Option<Uint8Array>,
    ) -> Result<Self> {
        let cipher = get_cipher(&cipher_name)?;
        let block_size = cipher.block_size();
        let key_vec = key.as_ref().to_vec();
        let iv_vec = iv.map(|i| i.as_ref().to_vec()).unwrap_or_default();

        Ok(Self {
            cipher_name,
            key: key_vec,
            iv: iv_vec,
            sequence_number: 0,
            block_size: if block_size == 0 { 8 } else { block_size },
        })
    }

    #[napi]
    pub fn encode(&mut self, payload: Uint8Array) -> Result<Buffer> {
        let cipher = get_cipher(&self.cipher_name)?;
        let iv_ref = if self.iv.is_empty() { None } else { Some(self.iv.as_slice()) };
        let enc = encrypt(cipher, &self.key, iv_ref, &payload)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        self.sequence_number = self.sequence_number.wrapping_add(1);
        Ok(Buffer::from(enc))
    }

    #[napi]
    pub fn decode(&mut self, payload: Uint8Array) -> Result<Buffer> {
        let cipher = get_cipher(&self.cipher_name)?;
        let iv_ref = if self.iv.is_empty() { None } else { Some(self.iv.as_slice()) };
        let dec = decrypt(cipher, &self.key, iv_ref, &payload)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        self.sequence_number = self.sequence_number.wrapping_add(1);
        Ok(Buffer::from(dec))
    }

    #[napi]
    pub fn encode_packet(&mut self, payload: Uint8Array) -> Result<Buffer> {
        let payload_len = payload.as_ref().len();
        let block_sz = self.block_size.max(8);

        // SSH Packet format:
        // 4 bytes: packet_length = 1 (pad_len byte) + payload_len + padding_len
        // 1 byte: padding_len (min 4, max 255, so (packet_length + 4) is a multiple of block_sz)
        let unpadded_len = 4 + 1 + payload_len; // 4 bytes len header + 1 byte pad len + payload
        let mut padding_len = block_sz - (unpadded_len % block_sz);
        if padding_len < 4 {
            padding_len += block_sz;
        }

        let packet_len = 1 + payload_len + padding_len;
        let total_frame_len = 4 + packet_len;

        let mut frame = Vec::with_capacity(total_frame_len);
        frame.extend_from_slice(&(packet_len as u32).to_be_bytes());
        frame.push(padding_len as u8);
        frame.extend_from_slice(payload.as_ref());
        // Simple deterministic padding for test consistency
        for i in 0..padding_len {
            frame.push((i % 256) as u8);
        }

        let cipher = get_cipher(&self.cipher_name)?;
        let iv_ref = if self.iv.is_empty() { None } else { Some(self.iv.as_slice()) };
        let enc = encrypt(cipher, &self.key, iv_ref, &frame)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        self.sequence_number = self.sequence_number.wrapping_add(1);
        Ok(Buffer::from(enc))
    }

    #[napi]
    pub fn decode_packet(&mut self, encrypted_packet: Uint8Array) -> Result<Buffer> {
        let cipher = get_cipher(&self.cipher_name)?;
        let iv_ref = if self.iv.is_empty() { None } else { Some(self.iv.as_slice()) };
        let dec = decrypt(cipher, &self.key, iv_ref, &encrypted_packet)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        if dec.len() < 5 {
            return Err(Error::new(
                Status::InvalidArg,
                "Packet too short to be a valid SSH frame",
            ));
        }

        let packet_len = u32::from_be_bytes([dec[0], dec[1], dec[2], dec[3]]) as usize;
        let pad_len = dec[4] as usize;

        if 4 + packet_len > dec.len() {
            return Err(Error::new(
                Status::InvalidArg,
                "Packet header indicates length exceeding buffer",
            ));
        }

        let payload_start = 5;
        let payload_end = 4 + packet_len - pad_len;

        if payload_end < payload_start || payload_end > dec.len() {
            return Err(Error::new(
                Status::InvalidArg,
                "Invalid SSH packet padding length",
            ));
        }

        self.sequence_number = self.sequence_number.wrapping_add(1);
        Ok(Buffer::from(&dec[payload_start..payload_end]))
    }

    #[napi]
    pub fn sequence_number(&self) -> u32 {
        self.sequence_number
    }

    #[napi]
    pub fn increment_sequence_number(&mut self) {
        self.sequence_number = self.sequence_number.wrapping_add(1);
    }
}

#[napi]
pub struct BtlsCodec {
    inner: BoringSshCodec,
}

#[napi]
impl BtlsCodec {
    #[napi(constructor)]
    pub fn new(
        cipher_name: String,
        key: Uint8Array,
        iv: Option<Uint8Array>,
    ) -> Result<Self> {
        let inner = BoringSshCodec::new(cipher_name, key, iv)?;
        Ok(Self { inner })
    }

    #[napi]
    pub fn encode(&mut self, payload: Uint8Array) -> Result<Buffer> {
        self.inner.encode(payload)
    }

    #[napi]
    pub fn decode(&mut self, payload: Uint8Array) -> Result<Buffer> {
        self.inner.decode(payload)
    }

    #[napi]
    pub fn encode_packet(&mut self, payload: Uint8Array) -> Result<Buffer> {
        self.inner.encode_packet(payload)
    }

    #[napi]
    pub fn decode_packet(&mut self, encrypted_packet: Uint8Array) -> Result<Buffer> {
        self.inner.decode_packet(encrypted_packet)
    }

    #[napi]
    pub fn sequence_number(&self) -> u32 {
        self.inner.sequence_number()
    }

    #[napi]
    pub fn increment_sequence_number(&mut self) {
        self.inner.increment_sequence_number()
    }
}
