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
    state::{Settings, Data},
    errors::DepositError,
    SETTINGS_SEED,
    DATA_SEED,
    id
};
pub struct Process;

impl Process {
    pub fn process(
        _program_id: &Pubkey,
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
            PixelBattleInstructrions::Clear => Self::clear(accounts),
            PixelBattleInstructrions::UpdateCost { cost } => todo!(),
            PixelBattleInstructrions::Draw { x, y, color } => todo!(),
        }?;
        Ok(())
    }

    fn clear(accounts: &[AccountInfo]) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let admin_info = next_account_info(accounts_iter)?;
        let settings_info = next_account_info(accounts_iter)?;
        let data_info = next_account_info(accounts_iter)?;

        if !admin_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if Settings::is_ok_settings_pubkey(settings_info.key) {
            return Err(DepositError::InvalidSettingsAccount.into())
        }

        if Data::is_ok_data_pubkey(data_info.key) {
            return Err(DepositError::InvalidDataAccount.into())
        }

        let settings = Settings::try_from_slice(&settings_info.data.borrow())?;

        let mut default_data = Data::new();
        default_data.create_empty_field(settings.width, settings.height);

        let _ = default_data.serialize(&mut &mut data_info.data.borrow_mut()[..]);
        msg!("Field Cleared");
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
        let data_info = next_account_info(accounts_iter)?;
        let rent_info = next_account_info(accounts_iter)?;
        let system_program_info = next_account_info(accounts_iter)?;

        let (settings_pubkey, settings_bump) = Settings::get_settings_pubkey_with_bump();
        let (data_pubkey, data_bump) = Settings::get_settings_pubkey_with_bump();

        if settings_pubkey != *settings_info.key {
            return Err(DepositError::InvalidSettingsAccount.into());
        }

        if !admin_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !settings_info.data_is_empty() {
            return Err(DepositError::AlreadyInit.into());
        }

        if data_pubkey != *data_info.key {
            return Err(DepositError::InvalidDataAccount.into());
        }

        let settings =  Settings {
            admin: admin_info.key.to_bytes(),
            cost,
            width,
            height
        };

        let settings_space = settings.try_to_vec()?.len();
        let settings_rent = &Rent::from_account_info(rent_info)?;
        let settings_lamports = settings_rent.minimum_balance(settings_space);
        let settings_signer_seeds: &[&[_]] = &[SETTINGS_SEED.as_bytes(), &[settings_bump]];

        invoke_signed(
            &system_instruction::create_account(
                admin_info.key,
                &settings_pubkey,
                settings_lamports,
                settings_space as u64,
                &id()
            ),
            &[admin_info.clone(), settings_info.clone(), system_program_info.clone()],
            &[&settings_signer_seeds]
        )?;

        let mut data = Data::new();
        data.create_empty_field(width, height);

        let data_space = data.try_to_vec()?.len();
        let data_rent = &Rent::from_account_info(rent_info)?;
        let data_lamports = data_rent.minimum_balance(data_space);
        let data_signer_seeds: &[&[_]] = &[DATA_SEED.as_bytes(), &[data_bump]];

        invoke_signed(
            &system_instruction::create_account(
                admin_info.key,
                &data_pubkey,
                data_lamports,
                data_space as u64,
                &id()
            ),
            &[admin_info.clone(), data_info.clone(), system_program_info.clone()],
            &[&data_signer_seeds]
        )?;

        let _ = data.serialize(&mut &mut data_info.data.borrow_mut()[..]);
        let _ = settings.serialize(&mut &mut settings_info.data.borrow_mut()[..]);

        msg!("Game init success");
        Ok(())
    }
}
