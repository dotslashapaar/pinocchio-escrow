use pinocchio::{self, account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::find_program_address, ProgramResult};
use pinocchio_token::{instructions::{CloseAccount, Transfer}, state::TokenAccount};

use crate::state::{escrow, Escrow};

pub fn process_take_instruction(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult{
    let [
        taker, maker, mint_x, mint_y, taker_ata_x, taker_ata_y, maker_ata_y, vault, escrow, _token_program, _system_program
    ] = accounts else{
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    let escrow_account = Escrow::from_account_info_unchecked(escrow);
    assert_eq!(escrow_account.mint_x, *mint_x.key());
    assert_eq!(escrow_account.mint_y, *mint_y.key());

    let vault_account = TokenAccount::from_account_info(vault)?;

    let seed = [(b"escrow"), maker.key().as_slice(), &[escrow_account.bump]];
    let seeds = &seed[..];
    let escrow_pda = find_program_address(seeds, &crate::ID).0;
    assert_eq!(*escrow.key(), escrow_pda);

    Transfer{
        from: taker_ata_y,
        to: maker_ata_y,
        authority: taker,
        amount: escrow_account.amount,
    }.invoke()?;

    let bump = [escrow_account.bump.to_le()];
    let seed = [Seed::from(b"escrow"), Seed::from(maker.key()), Seed::from(&bump)];
    let seeds = Signer::from(&seed);

    Transfer{
        from: vault,
        to: taker_ata_x,
        authority: escrow,
        amount: vault_account.amount(),
    }.invoke_signed(&[seeds.clone()])?;

    CloseAccount{
        account: vault,
        destination: maker,
        authority: escrow,
    }.invoke_signed(&[seeds])?;

    unsafe{
        *maker.borrow_mut_lamports_unchecked() += *escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() = 0;
    }


    Ok(())
}