use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::{ SETTINGS_SEED, id };

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Settings {
    pub cost: u64,
    pub admin: [u8; 32],
    pub width: u32,
    pub height: u32
}

impl Settings {
    pub fn get_settings_pubkey_with_bump() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SETTINGS_SEED.as_bytes()], &id())
    }

    pub fn get_pubkey() -> Pubkey {
        let (pubkey, _) = Self::get_settings_pubkey_with_bump();
        pubkey
    }

    pub fn is_ok_settings_pubkey(settings_pubkey: &Pubkey) -> bool {
        let (pubkey, _) = Self::get_settings_pubkey_with_bump();
        pubkey.to_bytes() == settings_pubkey.to_bytes()
    }
}
