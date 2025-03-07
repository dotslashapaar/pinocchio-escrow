use instructions::EscrowInstructions;
use pinocchio::{account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

mod instructions;
mod state;
mod tests;


use instructions::*;
use state::*;

entrypoint!(process_instruction);

const ID: Pubkey = five8_const::decode_32_const("22222222222222222222222222222222222222222222");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult{
    assert_eq!(program_id, &crate::ID);

    let (discriminator, data) = data.split_first().ok_or(ProgramError::InvalidAccountData)?;

    match EscrowInstructions::try_from(discriminator)? {
        EscrowInstructions::Make => process_make_instruction(accounts, data)?,
        EscrowInstructions::Take => process_take_instruction(accounts, data)?,
        EscrowInstructions::Refund => process_refund_instruction(accounts, data)?,
    }


    Ok(())
}
