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

// Define the post struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Post {
    pub post_id: String,
    pub user_id: String,
    pub content_data: String,  // JSON format: {"text": "...", "media_urls": [...]}
    pub timestamp: u64,
}

// Enum to handle different instructions
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum UserInstruction {
    RegisterUser { user_id: String, username: String, profile_data: String },
    AuthenticateUser { user_id: String, public_key: Pubkey },
    CreatePost { post_id: String, user_id: String, content_data: String, timestamp: u64 },
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
    let post_account = next_account_info(account_info_iter)?;

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
        UserInstruction::CreatePost { post_id, user_id, content_data, timestamp } => {
            let user = User::try_from_slice(&user_account.data.borrow())?;
            let mut post = Post::try_from_slice(&post_account.data.borrow())?;

            // Validate that the user exists
            if user.user_id != user_id {
                msg!("Invalid user ID!");
                return Err(ProgramError::InvalidArgument);
            }

            // Create the post
            post.post_id = post_id;
            post.user_id = user_id;
            post.content_data = content_data;
            post.timestamp = timestamp;

            // Serialize the post data back to the account
            post.serialize(&mut &mut post_account.data.borrow_mut()[..])?;

            msg!("Post created successfully: {:?}", post);
        }
    }

    Ok(())
}
