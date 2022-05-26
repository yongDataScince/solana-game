use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum PixelBattleInstructrions {
    // 1. `[signer, writable]` game's admin counter
    // 2. `[writable]` settings account, PDA
    // 3. `[]` Rent sysvar
    // 4. `[]` system program 
    Init {
        width: u32,
        height: u32,
        cost: u64
    },

    // 1. `[signer, writable]` game's admin counter
    // 2. `[writable]` settings account, PDA
    // 3. `[]` system program 
    Clear {
        width: u32,
        height: u32,
        cost: u64
    },

    // 1. `[signer, writable]` game's admin counter
    // 2. `[writable]` settings account, PDA
    // 3. `[]` system program 
    UpdateCost { cost: u64 },

    // 1. `[signer]` player info
    // 2. `[writable]` game accout, PDA
    // 3. `[]` system program
    Draw { x: u32, y: u32, color: String },
}
