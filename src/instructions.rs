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
    // 3. `[writable]` data account, PDA
    // 4. `[]` Rent sysvar
    // 5. `[]` system program 
    Init {
        width: u32,
        height: u32,
        cost: u64
    },

    // 1. `[signer, writable]` game's admin counter
    // 2. `[]` settings info, PDA
    // 2. `[writable]` data account, PDA
    // 3. `[]` system program 
    Clear,

    // 1. `[signer, writable]` game's admin counter
    // 2. `[writable]` settings account, PDA
    // 3. `[writable]` store SOLs account
    // 4. `[]` system program 
    WithDraw { cost: u64, to: [u8; 32] },

    // 1. `[signer]` player info
    // 2. `[]` settings account, PDA
    // 3. `[writable]` game accout, PDA
    // 4. `[writable]` store SOLs account
    // 5. `[]` system program
    Draw { x: usize, y: usize, color: String },
}
