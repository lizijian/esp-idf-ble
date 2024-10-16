use std::ops::BitOr;

#[derive(Default)]
#[repr(u8)]
pub enum IOCapabilities {
    #[default]
    DisplayOnly = 0,
    DisplayYesNo = 1,
    KeyboardOnly = 2,
    NoInputNoOutput = 3,
    Keyboard = 4,
}

#[derive(Default)]
#[repr(u8)]
pub enum AuthenticationRequest {
    #[default]
    NoBonding = esp_idf_sys::ESP_LE_AUTH_NO_BOND as u8,
    Bonding = esp_idf_sys::ESP_LE_AUTH_BOND as u8,
    Mitm = esp_idf_sys::ESP_LE_AUTH_REQ_MITM as u8,
    MitmBonding = esp_idf_sys::ESP_LE_AUTH_REQ_BOND_MITM as u8,
    SecureOnly = esp_idf_sys::ESP_LE_AUTH_REQ_SC_ONLY as u8,
    SecureBonding = esp_idf_sys::ESP_LE_AUTH_REQ_SC_BOND as u8,
    SecureMitm = esp_idf_sys::ESP_LE_AUTH_REQ_SC_MITM as u8,
    SecureMitmBonding = esp_idf_sys::ESP_LE_AUTH_REQ_SC_MITM_BOND as u8,
}

#[repr(u8)]
pub enum KeyMask {
    EncryptionKey = 0b0000_0001,
    IdentityResolvingKey = 0b0000_0010,
    ConnectionSignatureResolvingKey = 0b0000_0100,
    LinkKey = 0b0000_1000,
    Inner0011 = 0b0000_0011,
    Inner0101 = 0b0000_0101,
    Inner1001 = 0b0000_1001,
    Inner1010 = 0b0000_1010,
    Inner1100 = 0b0000_1100,
    Inner1101 = 0b0000_1101,
    Inner1011 = 0b0000_1011,
    Inner1111 = 0b0000_1111,
}

impl BitOr for KeyMask {
    type Output = KeyMask;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u8 | rhs as u8).into()
    }
}

impl From<u8> for KeyMask {
    fn from(from: u8) -> Self {
        match from {
            0b0000_0001 => KeyMask::EncryptionKey,
            0b0000_0010 => KeyMask::IdentityResolvingKey,
            0b0000_0100 => KeyMask::ConnectionSignatureResolvingKey,
            0b0000_1000 => KeyMask::LinkKey,
            0b0000_0011 => KeyMask::Inner0011,
            0b0000_0101 => KeyMask::Inner0101,
            0b0000_1001 => KeyMask::Inner1001,
            0b0000_1010 => KeyMask::Inner1010,
            0b0000_1100 => KeyMask::Inner1100,
            0b0000_1101 => KeyMask::Inner1101,
            0b0000_1011 => KeyMask::Inner1011,
            0b0000_1111 => KeyMask::Inner1111,
            _ => unimplemented!("This does not correspond to a valid KeyMask")
        }
    }
}

#[repr(u32)]
pub enum BleEncryption {
    Encryption = 0x01,
    EncryptionNoMitm = 0x02,
    EncryptionMitm = 0x03
}

#[derive(Default)]
pub struct SecurityConfig {
    pub auth_req_mode: AuthenticationRequest,
    pub io_capabilities: IOCapabilities,
    pub initiator_key: Option<KeyMask>,
    pub responder_key: Option<KeyMask>,
    pub max_key_size: Option<u8>,
    pub min_key_size: Option<u8>,
    pub static_passkey: Option<u32>,
    pub only_accept_specified_auth: bool,
    pub enable_oob: bool,
    // app_key_size: u8,
}
