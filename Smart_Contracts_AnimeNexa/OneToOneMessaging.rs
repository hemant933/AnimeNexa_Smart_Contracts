use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::clock::Clock,
};

// Define the Message struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Message {
    pub message_id: String,
    pub sender_user_id: String,
    pub recipient_user_id: String,
    pub message_content: String, // Ideally, this should be encrypted
    pub timestamp: u64,
}

// Enum to handle different instructions
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum UserInstruction {
    // Other existing instructions...
    SendMessage {
        message_id: String,
        sender_user_id: String,
        recipient_user_id: String,
        message_content: String,
    },
}

// Smart contract entrypoint
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let sender_account = next_account_info(account_info_iter)?;
    let recipient_account = next_account_info(account_info_iter)?;
    let message_account = next_account_info(account_info_iter)?;

    let instruction = UserInstruction::try_from_slice(instruction_data)?;

    match instruction {
        UserInstruction::SendMessage {
            message_id,
            sender_user_id,
            recipient_user_id,
            message_content,
        } => {
            send_message(
                message_id,
                sender_user_id,
                recipient_user_id,
                message_content,
                sender_account,
                recipient_account,
                message_account,
            )?;
        }
        // Other cases...
    }

    Ok(())
}

// Implementing the SendMessage Logic
fn send_message(
    message_id: String,
    sender_user_id: String,
    recipient_user_id: String,
    message_content: String,
    sender_account: &AccountInfo,
    recipient_account: &AccountInfo,
    message_account: &AccountInfo,
) -> ProgramResult {
    // Initialize the Message struct with the current timestamp
    let clock = Clock::get().unwrap();
    let timestamp = clock.unix_timestamp as u64;

    let message = Message {
        message_id,
        sender_user_id,
        recipient_user_id,
        message_content, // Encrypt this content before sending
        timestamp,
    };

    // Serialize and store the Message data in the message account
    message.serialize(&mut &mut message_account.data.borrow_mut()[..])?;

    msg!("Message sent successfully: {:?}", message);

    Ok(())
}
