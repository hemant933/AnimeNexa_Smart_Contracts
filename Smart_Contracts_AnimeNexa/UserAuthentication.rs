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

// Enum to differentiate between registration and authentication instructions
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum UserInstruction {
    RegisterUser { user_id: String, username: String, profile_data: String },
    AuthenticateUser { user_id: String, public_key: Pubkey },
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

    let instruction = UserInstruction::try_from_slice(instruction_data)?;

    match instruction {
        UserInstruction::RegisterUser { user_id, username, profile_data } => {
            let mut user = User::try_from_slice(&user_account.data.borrow())?;

            // Validate unique username
            if user.username != "" {
                msg!("Username already exists!");
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            // Register the user
            user.user_id = user_id;
            user.username = username;
            user.public_key = *user_account.key; // Use the account's public key
            user.profile_data = profile_data;

            // Serialize the updated user data back to the account
            user.serialize(&mut &mut user_account.data.borrow_mut()[..])?;

            msg!("User registered successfully: {:?}", user);
        }
        UserInstruction::AuthenticateUser { user_id, public_key } => {
            let user = User::try_from_slice(&user_account.data.borrow())?;

            // Check if the user ID matches
            if user.user_id != user_id {
                msg!("User ID does not match!");
                return Err(ProgramError::InvalidArgument);
            }

            // Authenticate by verifying the public key
            if user.public_key != public_key {
                msg!("Public key does not match!");
                return Err(ProgramError::InvalidArgument);
            }

            msg!("User authenticated successfully: {:?}", user);
        }
    }

    Ok(())
}
