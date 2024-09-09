use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define the Manga Access struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MangaAccess {
    pub manga_id: String,
    pub user_id: String,
    pub access_data: String, // Could be a timestamp, URL, or access token
}

// Enum to handle different instructions
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum UserInstruction {
    // Other existing instructions...
    AccessManga { manga_id: String, user_id: String, access_data: String },
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
    let manga_access_account = next_account_info(account_info_iter)?;

    let instruction = UserInstruction::try_from_slice(instruction_data)?;

    match instruction {
        UserInstruction::AccessManga { manga_id, user_id, access_data } => {
            access_manga(manga_id, user_id, access_data, user_account, manga_access_account)?;
        }
        // Other cases...
    }

    Ok(())
}

// Implementing the Manga Access Logic
fn access_manga(
    manga_id: String,
    user_id: String,
    access_data: String,
    user_account: &AccountInfo,
    manga_access_account: &AccountInfo,
) -> ProgramResult {
    // Deserialize the user's account data to check if the user exists
    let user = User::try_from_slice(&user_account.data.borrow())?;

    // Validate that the User ID matches
    if user.user_id != user_id {
        msg!("User ID does not match!");
        return Err(ProgramError::InvalidArgument);
    }

    // Initialize or update the Manga Access data
    let mut manga_access = MangaAccess::try_from_slice(&manga_access_account.data.borrow())?;
    manga_access.manga_id = manga_id;
    manga_access.user_id = user_id;
    manga_access.access_data = access_data;

    // Serialize the Manga Access data back to the account
    manga_access.serialize(&mut &mut manga_access_account.data.borrow_mut()[..])?;

    msg!("Manga access granted successfully: {:?}", manga_access);

    Ok(())
}
