use anchor_lang::prelude::*;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use ::borsh::{BorshSerialize, BorshDeserialize};

#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Message {
    pub poster: Pubkey,
    pub text: [u8; 280],
}

impl Default for Message {
    fn default() -> Self {
        Self {
            poster: Pubkey::default(),
            text: [0u8; 280],
        }
    }
}


entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let message_account = next_account_info(accounts_iter)?;

    if message_account.owner != program_id {
        msg!("Message account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let poster = *message_account.key;
    let mut message_data = Message {
        poster,
        text: [0; 280],
    };

    message_data.text.copy_from_slice(&input[0..280]);

    // Use Borsh to pack the data
    let serialized_data = message_data.try_to_vec().unwrap();
    message_account.data.borrow_mut()[..serialized_data.len()].copy_from_slice(&serialized_data);

    Ok(())
}
