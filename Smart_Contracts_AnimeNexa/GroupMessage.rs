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

// Define the Group struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Group {
    pub group_id: String,
    pub members: Vec<String>, // List of User IDs
}

// Define the Group Message struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GroupMessage {
    pub group_id: String,
    pub user_id: String,
    pub message_content: String, // Ideally, this should be encrypted
    pub timestamp: u64,
}

// Enum to handle different instructions
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum GroupInstruction {
    CreateGroup { group_id: String, members: Vec<String> },
    SendGroupMessage { group_id: String, user_id: String, message_content: String },
    // Other group-related instructions...
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
    let group_account = next_account_info(account_info_iter)?;
    let message_account = next_account_info(account_info_iter)?;

    let instruction = GroupInstruction::try_from_slice(instruction_data)?;

    match instruction {
        GroupInstruction::CreateGroup { group_id, members } => {
            create_group(group_id, members, group_account)?;
        }
        GroupInstruction::SendGroupMessage { group_id, user_id, message_content } => {
            send_group_message(group_id, user_id, message_content, user_account, group_account, message_account)?;
        }
        // Other cases...
    }

    Ok(())
}

// Implementing the Create Group Logic
fn create_group(group_id: String, members: Vec<String>, group_account: &AccountInfo) -> ProgramResult {
    let group = Group { group_id, members };

    // Serialize and store the Group data in the group account
    group.serialize(&mut &mut group_account.data.borrow_mut()[..])?;

    msg!("Group created successfully: {:?}", group);

    Ok(())
}

// Implementing the Send Group Message Logic
fn send_group_message(
    group_id: String,
    user_id: String,
    message_content: String,
    user_account: &AccountInfo,
    group_account: &AccountInfo,
    message_account: &AccountInfo,
) -> ProgramResult {
    // Deserialize the group account to check if the user is a member
    let group = Group::try_from_slice(&group_account.data.borrow())?;
    
    if !group.members.contains(&user_id) {
        msg!("User is not a member of the group!");
        return Err(ProgramError::InvalidArgument);
    }

    // Initialize the Group Message struct with the current timestamp
    let clock = Clock::get().unwrap();
    let timestamp = clock.unix_timestamp as u64;

    let message = GroupMessage {
        group_id,
        user_id,
        message_content, // Encrypt this content before sending
        timestamp,
    };

    // Serialize and store the Group Message data in the message account
    message.serialize(&mut &mut message_account.data.borrow_mut()[..])?;

    msg!("Group message sent successfully: {:?}", message);

    Ok(())
}
