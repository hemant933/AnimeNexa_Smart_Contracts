use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define the user struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub public_key: Pubkey,
    pub profile_data: String,
}

// Smart contract entrypoint
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;

    if user_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut user = User::try_from_slice(&user_account.data.borrow())?;

    // Deserialize input data (assuming input data is in the same format as User struct)
    let input_user: User = User::try_from_slice(instruction_data)?;

    // Validate unique username
    if user.username != "" {
        msg!("Username already exists!");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Register the user
    user.user_id = input_user.user_id;
    user.username = input_user.username;
    user.public_key = input_user.public_key;
    user.profile_data = input_user.profile_data;

    // Serialize the updated user data back to the account
    user.serialize(&mut &mut user_account.data.borrow_mut()[..])?;

    msg!("User registered successfully: {:?}", user);

    Ok(())
}
