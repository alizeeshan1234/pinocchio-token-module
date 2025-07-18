// use mollusk_svm::sysvar;
use pinocchio::{
    account_info::AccountInfo, instruction::{AccountMeta, Instruction}, program, sysvars::rent::Rent, ProgramResult, program_error::ProgramError
};

use pinocchio_token::ID;

pub const TOKEN_PROGRAM_ID: [u8; 32] = [
    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172,
    28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
];

pub const TOKEN2022_PROGRAM_ID: [u8; 32] = [
    10, 112, 131, 147, 84, 51, 84, 60, 164, 244, 75, 206, 19, 222, 193, 205,
    72, 94, 156, 56, 196, 228, 137, 19, 29, 150, 42, 168, 58, 88, 217, 19,
];

pub const SYSTEM_PROGRAM_ID: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub use crate::helper::create_account_instruction_data;

pub use crate::token_type::TokenProgramType;

use solana_program::{pubkey::Pubkey, sysvar::rent};

/*
Need to be implemented:
Adding seeds paramter.

create_token_account_cpi_with_seeds(
    token_program_type: TokenProgramType,
    token_account: &AccountInfo,
    mint: &AccountInfo,
    owner: &AccountInfo,
    payer: &AccountInfo,
    system_program: &AccountInfo,
    rent_sysvar: &AccountInfo,
    seeds: &[u8]
)
*/

pub fn create_token_account_cpi(
    token_program_type: TokenProgramType,
    token_account: &AccountInfo,
    mint: &AccountInfo,
    owner: &AccountInfo,
    payer: &AccountInfo,
    system_program: &AccountInfo,
    rent_sysvar: &AccountInfo,
) -> ProgramResult {

    let selected_token_program_id = match token_program_type {
        TokenProgramType::PToken => Pubkey::new_from_array(TOKEN_PROGRAM_ID),
        TokenProgramType::Token2022 => Pubkey::new_from_array(TOKEN2022_PROGRAM_ID),
    };

    if *system_program.key() != SYSTEM_PROGRAM_ID {
        return Err(ProgramError::InvalidAccountOwner)
    }

    let space = 165u64;
    let rent = Rent::from_account_info(rent_sysvar).unwrap();
    let lamports = rent.minimum_balance(space as usize);

    // STEP 1: CPI to System Program to create generic account

    let create_account_data = create_account_instruction_data(lamports, space, &selected_token_program_id.to_bytes());

    let create_account_meta = [
        AccountMeta::new(payer.key(), true, true),
        AccountMeta::new(token_account.key(), true, false),
    ];

    let create_instruction = Instruction {
        program_id: &SYSTEM_PROGRAM_ID,
        data: &create_account_data,
        accounts: &create_account_meta,
    };

    program::invoke(
        &create_instruction, 
        &[&payer, &token_account, &system_program]
    )?;

    pinocchio::pubkey::log(b"Reached Here!______________dummy");

    // STEP 2: CPI to Token Program to initialize as token account

    let token_program_selected = &selected_token_program_id.to_bytes();
    let initialize_token_account_meta = [
        AccountMeta::writable(token_account.key()),
        AccountMeta::readonly(mint.key()),
        AccountMeta::readonly(owner.key()),
        AccountMeta::readonly(rent_sysvar.key()),
        AccountMeta::readonly(token_program_selected),
        AccountMeta::readonly(system_program.key()),
    ];

    let initialize_data = vec![1u8]; // InitializeAccount instruction discriminator (the instruction to be called)

    let initialize_instruction = Instruction {
        program_id: &selected_token_program_id.to_bytes(),
        data: &initialize_data, // InitializeAccount instruction discriminator
        accounts: &initialize_token_account_meta,
    };

    program::invoke(
        &initialize_instruction,
        &[&token_account, &mint, &owner, &rent_sysvar, &system_program]
    )?;

    Ok(())
}

#[cfg(test)]
pub mod testing {
    use super::*;

    use {
        borsh::BorshSerialize,
        mollusk_svm::Mollusk,
        solana_sdk::{
            instruction::{AccountMeta, Instruction},
            signature::Keypair,
            signer::Signer,
        },
    };

    use crate::{TokenInstruction, TokenProgramType};

    use solana_sdk::{pubkey, pubkey::Pubkey, account::Account};

    const ID: Pubkey = pubkey!("FHUW81Au2k38MLkgsZ7af8FZDiDa1s8t2Hzn6bKstrpC");

    #[test]
    pub fn test_create_token_account_cpi_token2022() {
        let program_id = ID;

        let token_account = Keypair::new();
        let mint = Keypair::new();
        let owner = Keypair::new();
        let payer = Keypair::new();
        let system_program = Pubkey::new_from_array(SYSTEM_PROGRAM_ID);
        let rent_sysvar = solana_program::sysvar::rent::id();

        let token2022_program = Pubkey::new_from_array(TOKEN2022_PROGRAM_ID);

        let instruction_data = TokenInstruction::InitializeTokenAccount { 
            token_program_type: TokenProgramType::Token2022
        }.try_to_vec().unwrap();

        let instruction = Instruction {
            program_id: program_id.into(),
            data: instruction_data,
            accounts: vec![
                AccountMeta::new(token_account.pubkey(), true),
                AccountMeta::new(mint.pubkey(), false),
                AccountMeta::new(owner.pubkey(), false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(system_program, false),
                AccountMeta::new(rent_sysvar, false),
            ],
        };

        let accounts = vec![
            (token2022_program, Account::default()),
            (token_account.pubkey(), Account::default()),
            (mint.pubkey(), Account::default()),
            (owner.pubkey(), Account::default()),
            (payer.pubkey(), Account::default()),
            (system_program, Account::default()),
            (rent_sysvar, Account::default()),
        ];

        let mollusk = Mollusk::new(&program_id.into(), "target/deploy/pinocchio_spl_cpi");

        let result = mollusk.process_instruction(&instruction, &accounts);

        assert!(result.raw_result.is_ok(), "Instruction failed: {:?}", result.raw_result);

    }
}

// #[cfg(test)]
// mod tests {
//     use mollusk_svm::Mollusk;
//     use solana_sdk::{pubkey, pubkey::Pubkey, account::Account};

//     use super::*;

//     const ID: Pubkey = pubkey!("FHUW81Au2k38MLkgsZ7af8FZDiDa1s8t2Hzn6bKstrpC");
//     const USER: Pubkey = Pubkey::new_from_array([0x01; 32]);
//     #[test]
//     fn test_create_token_account_cpi() {
//         //mollusk instance
//         let mollusk = Mollusk::new(&ID, "../../target/deploy/pinocchio_spl_cpi");

//         //Pubkeys
//         let token_account = Account::new(1 * 1_000_000_000, 0, &TOKEN_PROGRAM_ID.into());
//         let mint =  Account::new(1 * 1_000_000_000, 0, &TOKEN_PROGRAM_ID.into());
//         let owner =  Account::new(1 * 1_000_000_000, 0, &TOKEN_PROGRAM_ID.into());
//         let payer =  Account::new(1 * 1_000_000_000, 0, &SYSTEM_PROGRAM_ID.into());
//         let rent_sysvar = solana_program::sysvar::rent::id();

//         //Build the accounts

//         //Get the accounts meta

//         //Data

//         //Build IX

//         //Get Tx Accounts

//         //process and validate instruction
//     }
// }