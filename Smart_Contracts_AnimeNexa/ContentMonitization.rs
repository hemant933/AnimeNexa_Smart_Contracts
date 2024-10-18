use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

// Define the monetized content struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MonetizedContent {
    pub content_id: String,
    pub creator_user_id: String,
    pub price: u64,
    pub purchase_data: Vec<Purchase>,
}

// Define the purchase struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Purchase {
    pub purchaser_user_id: String,
    pub amount: u64,
    pub timestamp: u64,
}

// Enum to handle different instructions
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum UserInstruction {
    RegisterUser { user_id: String, username: String, profile_data: String },
    AuthenticateUser { user_id: String, public_key: Pubkey },
    CreatePost { post_id: String, user_id: String, content_data: String, timestamp: u64 },
    UploadMedia { media_id: String, post_id: String, user_id: String, media_url: String, media_type: String },
    SupportUser { support_id: String, sender_user_id: String, recipient_user_id: String, amount: u64, timestamp: u64 },
    MonetizeContent { content_id: String, creator_user_id: String, price: u64 },
    PurchaseContent { content_id: String, purchaser_user_id: String, amount: u64, timestamp: u64 },
}

// Smart contract entrypoint
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let creator_account = next_account_info(account_info_iter)?;
    let purchaser_account = next_account_info(account_info_iter)?;
    let content_account = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

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
            // Existing media upload logic...
        }
        UserInstruction::SupportUser { support_id, sender_user_id, recipient_user_id, amount, timestamp } => {
            // Existing support logic...
        }
        UserInstruction::MonetizeContent { content_id, creator_user_id, price } => {
            // Deserialize the creator's account data
            let creator = User::try_from_slice(&creator_account.data.borrow())?;

            // Validate that the user is the creator
            if creator.user_id != creator_user_id {
                msg!("Invalid creator user ID!");
                return Err(ProgramError::InvalidArgument);
            }

            // Initialize or update monetized content
            let mut content = MonetizedContent::try_from_slice(&content_account.data.borrow())?;
            content.content_id = content_id;
            content.creator_user_id = creator_user_id;
            content.price = price;

            // Serialize the content data back to the account
            content.serialize(&mut &mut content_account.data.borrow_mut()[..])?;

            msg!("Content monetized successfully: {:?}", content);
        }
        UserInstruction::PurchaseContent { content_id, purchaser_user_id, amount, timestamp } => {
            // Deserialize the content data to validate purchase
            let mut content = MonetizedContent::try_from_slice(&content_account.data.borrow())?;

            // Ensure the purchase amount matches the content's price
            if content.price != amount {
                msg!("Incorrect amount for purchasing content!");
                return Err(ProgramError::InvalidArgument);
            }

            // Transfer funds from purchaser to creator
            **purchaser_account.lamports.borrow_mut() -= amount;
            **creator_account.lamports.borrow_mut() += amount;

            // Record the purchase
            let purchase = Purchase {
                purchaser_user_id,
                amount,
                timestamp,
            };
            content.purchase_data.push(purchase);

            // Serialize the updated content data back to the account
            content.serialize(&mut &mut content_account.data.borrow_mut()[..])?;

            msg!("Content purchased successfully: {:?}", content);
        }
    }

    Ok(())
}
