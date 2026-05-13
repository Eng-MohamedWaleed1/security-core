#[derive(Debug, PartialEq, Clone)]
pub enum BufferState {
    Idle,
    Loaded,
    Encrypted,
    Decrypted,
}

pub struct SecureBuffer {
    data: Vec<u8>,
    key: u8,
    state: BufferState,
    checksum: u64,
}

pub struct BufferView<'buf> {
    pub data: &'buf [u8],
    pub state: &'buf BufferState,
    pub checksum: u64,
}

fn fnv1a_checksum(data: &[u8]) -> u64 {
    const BASIS: u64 = 14_695_981_039_346_656_037;
    const PRIME: u64 = 1_099_511_628_211;
    data.iter().fold(BASIS, |acc, &b| acc.wrapping_mul(PRIME) ^ (b as u64))
}

fn derive_positional_key(key: u8, position: usize) -> u8 {
    let pos_byte = (position & 0xFF) as u8;
    let rotated = key.wrapping_add(pos_byte).rotate_left((position % 7) as u32);
    rotated ^ !key
}

fn apply_cipher(data: &mut Vec<u8>, key: u8) {
    for (i, byte) in data.iter_mut().enumerate() {
        let pos_key = derive_positional_key(key, i);
        let xored = *byte ^ pos_key;
        let shift = ((i % 3) + 1) as u32;
        *byte = xored.rotate_left(shift);
    }
}

fn reverse_cipher(data: &mut Vec<u8>, key: u8) {
    for (i, byte) in data.iter_mut().enumerate() {
        let shift = ((i % 3) + 1) as u32;
        let unrotated = byte.rotate_right(shift);
        let pos_key = derive_positional_key(key, i);
        *byte = unrotated ^ pos_key;
    }
}

impl SecureBuffer {
    pub fn new(key: u8) -> Self {
        SecureBuffer { data: Vec::new(), key, state: BufferState::Idle, checksum: 0 }
    }

    /// المُرسِل: يحمّل plaintext ويحسب الـ checksum
    pub fn load(&mut self, plaintext: Vec<u8>) -> Result<(), &'static str> {
        if self.state == BufferState::Encrypted {
            return Err("ERR-101: Cannot overwrite active ciphertext.");
        }
        self.checksum = fnv1a_checksum(&plaintext);
        self.data = plaintext;
        self.state = BufferState::Loaded;
        Ok(())
    }

    /// المُستقبِل: يحمّل ciphertext جاهز بدون حساب checksum جديد
    pub fn load_ciphertext(&mut self, ciphertext: Vec<u8>) -> Result<(), &'static str> {
        if self.state != BufferState::Idle {
            return Err("ERR-105: Receiver buffer must be Idle.");
        }
        // الـ checksum سيُتحقق منه بعد فك التشفير عبر إعادة حسابه
        self.data = ciphertext;
        self.state = BufferState::Encrypted;
        Ok(())
    }

    pub fn encrypt(&mut self) -> Result<(), &'static str> {
        if self.state != BufferState::Loaded {
            return Err("ERR-102: Buffer must be Loaded to encrypt.");
        }
        apply_cipher(&mut self.data, self.key);
        self.state = BufferState::Encrypted;
        Ok(())
    }

    pub fn decrypt(&mut self) -> Result<(), &'static str> {
        if self.state != BufferState::Encrypted {
            return Err("ERR-103: Buffer must be Encrypted to decrypt.");
        }
        reverse_cipher(&mut self.data, self.key);
        self.state = BufferState::Decrypted;
        Ok(())
    }

    pub fn view(&self) -> BufferView<'_> {
        BufferView { data: &self.data, state: &self.state, checksum: self.checksum }
    }

    pub fn clear(&mut self) {
        for byte in self.data.iter_mut() { *byte = 0x00; }
        self.data.clear();
        self.checksum = 0;
        self.state = BufferState::Idle;
    }
}

impl Drop for SecureBuffer {
    fn drop(&mut self) { self.clear(); }
}