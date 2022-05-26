use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    msg,
    pubkey::Pubkey,
    account_info::{ next_account_info, AccountInfo },
    entrypoint::{ ProgramResult },
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
    program::invoke_signed, system_instruction
};
use crate::{
    instructions::PixelBattleInstructrions,
    state::Settings,
    errors::DepositError,
    SETTINGS_SEED,
    id
};
pub struct Process;

impl Process {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8]
    ) -> ProgramResult {
        let instruction = PixelBattleInstructrions::try_from_slice(data)?;

        match instruction {
            PixelBattleInstructrions::Init {
                width,
                height,
                cost 
            } => Self::init_game(accounts, width, height, cost),
            PixelBattleInstructrions::Clear { width, height, cost } => Self::update_cost(),
            PixelBattleInstructrions::UpdateCost { cost } => todo!(),
            PixelBattleInstructrions::Draw { x, y, color } => todo!(),
        }?;
        Ok(())
    }

    fn clear() -> ProgramResult {
        Ok(())
    }

    fn update_cost() -> ProgramResult {
        Ok(())
    }

    fn init_game(
        accounts: &[AccountInfo],
        width: u32,
        height: u32,
        cost: u64
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let admin_info = next_account_info(accounts_iter)?;
        let settings_info = next_account_info(accounts_iter)?;
        let rent_info = next_account_info(accounts_iter)?;
        let system_program_info = next_account_info(accounts_iter)?;

        let (settings_pubkey, settings_bump) = Settings::get_settings_pubkey_with_bump();
        if settings_pubkey != *settings_info.key {
            return Err(DepositError::InvalidSettingsAccount.into());
        }

        if !admin_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !settings_info.data_is_empty() {
            return Err(DepositError::AlreadyInit.into());
        }

        let settings =  Settings {
            admin: admin_info.key.to_bytes(),
            cost,
            width,
            height
        };

        let space = settings.try_to_vec()?.len();
        let rent = &Rent::from_account_info(rent_info)?;
        let lamports = rent.minimum_balance(space);
        let signer_seeds: &[&[_]] = &[SETTINGS_SEED.as_bytes(), &[settings_bump]];

        invoke_signed(
            &system_instruction::create_account(
                admin_info.key,
                &settings_pubkey,
                lamports,
                space as u64,
                &id()
            ),
            &[admin_info.clone(), settings_info.clone(), system_program_info.clone()],
            &[&signer_seeds]
        )?;

        let _ = settings.serialize(&mut &mut settings_info.data.borrow_mut()[..]);
        msg!("Game init success");
        Ok(())
    }
}
