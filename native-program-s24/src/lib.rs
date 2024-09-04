use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    system_instruction,
    program::invoke,
    clock::Clock,
    sysvar::Sysvar,
};


#[derive(Clone, Debug, Default, PartialEq)]
struct AccountState {
    balance: u64,
    last_withdrawal_time: i64,
}


entrypoint!(process_instruction);


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
 
    if accounts.len() < 2 {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData.into());
    }

    let account_info_iter = &mut accounts.iter();
    let account = next_account_info(account_info_iter)?;
    let payer = next_account_info(account_info_iter)?;


    match instruction_data[0] {
        0 => initialize_account(account, payer),
        1 => deposit(account, payer, &instruction_data[1..]),
        2 => withdraw(account, payer),
        _ => Err(solana_program::program_error::ProgramError::InvalidInstructionData.into()),
    }
}

fn initialize_account(account: &AccountInfo, payer: &AccountInfo) -> ProgramResult {
    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(std::mem::size_of::<AccountState>());

    if **account.lamports.borrow() < required_lamports {
        msg!("Insufficient balance for account creation");
        return Err(solana_program::program_error::ProgramError::InsufficientFunds.into());
    }

    let mut account_state = AccountState::default();
    account_state.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}

fn deposit(account: &AccountInfo, payer: &AccountInfo, amount_data: &[u8]) -> ProgramResult {
    let amount = u64::from_le_bytes(amount_data.try_into().unwrap());

    let ix = system_instruction::transfer(payer.key, account.key, amount);
    invoke(&ix, &[payer.clone(), account.clone()])?;

    let mut account_state = AccountState::unpack(&account.data.borrow())?;
    account_state.balance += amount;
    AccountState::pack(account_state, &mut account.data.borrow_mut())?;

    Ok(())
}

fn withdraw(account: &AccountInfo, receiver: &AccountInfo) -> ProgramResult {
    let mut account_state = AccountState::unpack(&account.data.borrow())?;
    let current_time = Clock::get()?.unix_timestamp;

    if current_time - account_state.last_withdrawal_time < 24 * 60 * 60 {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData.into());
    }

    let withdraw_amount = account_state.balance / 10; 
    **account.lamports.borrow_mut() -= withdraw_amount;
    **receiver.lamports.borrow_mut() += withdraw_amount;

    account_state.balance -= withdraw_amount;
    account_state.last_withdrawal_time = current_time;
    AccountState::pack(account_state, &mut account.data.borrow_mut())?;

    Ok(())
}
