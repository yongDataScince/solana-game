use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::{ SETTINGS_SEED, DATA_SEED, id };

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
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Cell {
    pub writer: [u8; 32],
    pub color: String
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            writer: [0; 32], 
            color: "#FFFFFF".to_string()
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Data {
    pub field: Vec<Vec<Cell>>
}

impl Data {
    pub fn new() -> Self {
        Data { field: Vec::new() }
    }
    pub fn create_empty_field(&mut self, width: u32, height: u32) {
        for y in 0..height {
            for x in 0..width {
                self.field[y as usize][x as usize] = Cell::default();
            }
        }
    }
}

impl Data {
    pub fn get_data_pubkey_with_bump() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[DATA_SEED.as_bytes()], &id())
    }

    pub fn get_pubkey() -> Pubkey {
        let (pubkey, _) = Self::get_data_pubkey_with_bump();
        pubkey
    }

    pub fn is_ok_data_pubkey(data_pubkey: &Pubkey) -> bool {
        let (pubkey, _) = Self::get_data_pubkey_with_bump();
        pubkey.to_bytes() == data_pubkey.to_bytes()
    }
}
