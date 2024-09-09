use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define the media struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Media {
    pub media_id: String,
    pub post_id: String,
    pub user_id: String,
    pub media_url: String,
    pub media_type: String,
}

// Enum to handle different instructions
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum UserInstruction {
    RegisterUser { user_id: String, username: String, profile_data: String },
    AuthenticateUser { user_id: String, public_key: Pubkey },
    CreatePost { post_id: String, user_id: String, content_data: String, timestamp: u64 },
    UploadMedia { media_id: String, post_id: String, user_id: String, media_url: String, media_type: String },
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
    let media_account = next_account_info(account_info_iter)?;

    if user_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = UserInstruction::try_from_slice(instruction_data)?;

    match instruction {
        UserInstruction::RegisterUser { user_id, username, profile_data } => {
            // Existing user registration logic...
        }
        UserInstruction::AuthenticateUser { user_id, public_key } => {
            // Existing user authentication logic...
        }
        UserInstruction::CreatePost { post_id, user_id, content_data, timestamp } => {
            // Existing post creation logic...
        }
        UserInstruction::UploadMedia { media_id, post_id, user_id, media_url, media_type } => {
            // Deserialize the user and post data to validate
            let user = User::try_from_slice(&user_account.data.borrow())?;

            // Validate that the user exists
            if user.user_id != user_id {
                msg!("Invalid user ID!");
                return Err(ProgramError::InvalidArgument);
            }

            // Link media to the post
            let mut media = Media::try_from_slice(&media_account.data.borrow())?;
            media.media_id = media_id;
            media.post_id = post_id;
            media.user_id = user_id;
            media.media_url = media_url;
            media.media_type = media_type;

            // Serialize the media data back to the account
            media.serialize(&mut &mut media_account.data.borrow_mut()[..])?;

            msg!("Media uploaded successfully: {:?}", media);
        }
    }

    Ok(())
}
