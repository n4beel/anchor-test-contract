/*
 * ============================================================================
 * ANCHOR TEST CONTRACT - COMPREHENSIVE COMMENT TESTING
 * ============================================================================
 *
 * This contract is designed to test various aspects of static analysis tools
 * particularly focusing on Source Lines of Code (SLOC) counting functionality.
 *
 * The contract includes:
 * - Single-line comments (//)
 * - Multi-line comments (/* */)
 * - Inline comments
 * - Documentation comments (///)
 * - Various code patterns for comprehensive testing
 *
 * Author: Static Analysis Testing Suite
 * Version: 1.0.0
 * ============================================================================
 */

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer}; // Import token utilities

// Program ID declaration - this is the unique identifier for our program
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// Main program module containing all instructions and business logic
/// This annotation generates the program entry point automatically
#[program]
pub mod anchor_test_contract {
    use super::*; // Import parent scope items

    /*
     * INITIALIZATION INSTRUCTION
     * ==========================
     * This function initializes a new user account with default values.
     * It demonstrates basic account creation patterns in Anchor.
     */

    /// Initialize a new user account with provided parameters
    ///
    /// # Arguments
    /// * `ctx` - The context containing accounts and program information
    /// * `name` - The user's display name (max 32 characters)
    /// * `age` - The user's age (must be positive)
    ///
    /// # Returns
    /// * `Result<()>` - Success or error result
    pub fn initialize_user(
        ctx: Context<InitializeUser>, // Account context
        name: String,                 // User name parameter
        age: u8,                      // User age parameter
    ) -> Result<()> {
        // Get mutable reference to user account
        let user_account = &mut ctx.accounts.user;

        /*
         * Input validation section
         * Ensure the provided parameters meet our requirements
         */
        require!(name.len() <= 32, CustomError::NameTooLong); // Name length check
        require!(age > 0, CustomError::InvalidAge); // Age validation

        // Set account fields with provided values
        user_account.authority = ctx.accounts.authority.key(); // Set authority
        user_account.name = name; // Set name
        user_account.age = age; // Set age
        user_account.balance = 0; // Initialize balance to zero
        user_account.is_active = true; // Mark account as active
        user_account.created_at = Clock::get()?.unix_timestamp; // Set creation timestamp

        // Log successful initialization
        msg!("User account initialized: {}", user_account.name);

        Ok(()) // Return success
    }

    /// Update user information with new values
    /// Only the account authority can perform this operation
    pub fn update_user(
        ctx: Context<UpdateUser>,
        new_name: Option<String>, // Optional new name
        new_age: Option<u8>,      // Optional new age
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user; // Get mutable user account reference

        // Update name if provided
        if let Some(name) = new_name {
            require!(name.len() <= 32, CustomError::NameTooLong); // Validate name length
            user_account.name = name; // Update name
        }

        // Update age if provided
        if let Some(age) = new_age {
            require!(age > 0, CustomError::InvalidAge); // Validate age
            user_account.age = age; // Update age
        }

        msg!("User account updated successfully"); // Log update
        Ok(()) // Return success
    }

    /*
     * TOKEN TRANSFER INSTRUCTION
     * ==========================
     * This instruction handles token transfers between accounts.
     * It includes comprehensive validation and error handling.
     */

    /// Transfer tokens between user accounts
    ///
    /// # Security Considerations
    /// - Validates transfer amount is positive
    /// - Checks sender has sufficient balance
    /// - Ensures both accounts are active
    /// - Updates balances atomically
    pub fn transfer_tokens(
        ctx: Context<TransferTokens>,
        amount: u64, // Transfer amount in smallest token units
    ) -> Result<()> {
        // Extract account references for readability
        let sender = &mut ctx.accounts.sender; // Sender account
        let receiver = &mut ctx.accounts.receiver; // Receiver account

        /*
         * Pre-transfer validation checks
         * These ensure the transfer can be completed safely
         */
        require!(amount > 0, CustomError::InvalidAmount); // Amount must be positive
        require!(sender.balance >= amount, CustomError::InsufficientFunds); // Sufficient balance
        require!(sender.is_active, CustomError::AccountInactive); // Sender must be active
        require!(receiver.is_active, CustomError::AccountInactive); // Receiver must be active

        // Perform atomic balance updates
        sender.balance = sender
            .balance
            .checked_sub(amount) // Safe subtraction to prevent underflow
            .ok_or(CustomError::MathOverflow)?; // Handle overflow error

        receiver.balance = receiver
            .balance
            .checked_add(amount) // Safe addition to prevent overflow
            .ok_or(CustomError::MathOverflow)?; // Handle overflow error

        // Emit transfer event for off-chain monitoring
        emit!(TokenTransferEvent {
            from: sender.authority,                  // Sender's authority
            to: receiver.authority,                  // Receiver's authority
            amount,                                  // Transfer amount
            timestamp: Clock::get()?.unix_timestamp, // Current timestamp
        });

        msg!("Transferred {} tokens successfully", amount); // Log successful transfer
        Ok(()) // Return success
    }

    /// Deactivate a user account
    /// This is a security measure to disable compromised accounts
    pub fn deactivate_user(ctx: Context<DeactivateUser>) -> Result<()> {
        let user_account = &mut ctx.accounts.user; // Get user account

        require!(user_account.is_active, CustomError::AccountAlreadyInactive); // Must be active

        user_account.is_active = false; // Mark as inactive

        msg!("User account deactivated: {}", user_account.authority); // Log deactivation
        Ok(()) // Success
    }
}

/*
 * ============================================================================
 * ACCOUNT STRUCTURES AND VALIDATION CONTEXTS
 * ============================================================================
 *
 * This section defines the account structures and their validation rules.
 * Each context specifies which accounts are required and their constraints.
 */

/// User account data structure
/// Contains all user-related information stored on-chain
#[account]
pub struct UserAccount {
    pub authority: Pubkey, // Account owner/authority (32 bytes)
    pub name: String,      // User display name (variable length, max 32)
    pub age: u8,           // User age (1 byte)
    pub balance: u64,      // Token balance (8 bytes)
    pub is_active: bool,   // Account status flag (1 byte)
    pub created_at: i64,   // Account creation timestamp (8 bytes)
}

// Calculate account size for rent calculation
impl UserAccount {
    /// Calculate the space required for this account
    /// Used for rent calculation during account creation
    pub const LEN: usize = 8 +  // Anchor discriminator
        32 +                     // authority: Pubkey
        4 + 32 +                // name: String (length + max content)
        1 +                     // age: u8
        8 +                     // balance: u64
        1 +                     // is_active: bool
        8; // created_at: i64
}

/// Context for initializing a new user account
#[derive(Accounts)]
pub struct InitializeUser<'info> {
    /// The user account being created
    #[account(
        init,                                    // Initialize new account
        payer = authority,                       // Authority pays for account creation
        space = UserAccount::LEN,               // Required space for account
        seeds = [b"user", authority.key().as_ref()], // Deterministic address generation
        bump                                     // Find valid bump seed
    )]
    pub user: Account<'info, UserAccount>, // The user account

    /// The authority/owner of the account
    #[account(mut)] // Must be mutable to pay rent
    pub authority: Signer<'info>, // Must sign the transaction

    /// System program for account creation
    pub system_program: Program<'info, System>, // Required for account initialization
}

/// Context for updating user information
#[derive(Accounts)]
pub struct UpdateUser<'info> {
    /// The user account being updated
    #[account(
        mut,                                     // Account data will be modified
        has_one = authority,                     // Verify authority ownership
        seeds = [b"user", authority.key().as_ref()], // Verify PDA derivation
        bump                                     // Verify bump seed
    )]
    pub user: Account<'info, UserAccount>, // The user account

    /// The authority/owner of the account
    pub authority: Signer<'info>, // Must sign to authorize changes
}

/// Context for token transfers between users
#[derive(Accounts)]
pub struct TransferTokens<'info> {
    /// Sender's user account
    #[account(
        mut,                                     // Will be modified (balance decrease)
        has_one = authority,                     // Verify ownership
        seeds = [b"user", authority.key().as_ref()], // Verify PDA
        bump
    )]
    pub sender: Account<'info, UserAccount>, // Sender account

    /// Receiver's user account
    #[account(
        mut,                                     // Will be modified (balance increase)
        seeds = [b"user", receiver.authority.as_ref()], // Verify receiver PDA
        bump
    )]
    pub receiver: Account<'info, UserAccount>, // Receiver account

    /// Transaction authority (must be sender)
    pub authority: Signer<'info>, // Must sign transaction
}

/// Context for deactivating a user account
#[derive(Accounts)]
pub struct DeactivateUser<'info> {
    /// The user account being deactivated
    #[account(
        mut,                                     // Account status will change
        has_one = authority,                     // Verify ownership
        seeds = [b"user", authority.key().as_ref()], // Verify PDA
        bump
    )]
    pub user: Account<'info, UserAccount>, // The user account

    /// Account authority
    pub authority: Signer<'info>, // Must sign for deactivation
}

/*
 * ============================================================================
 * EVENTS AND ERROR DEFINITIONS
 * ============================================================================
 *
 * Events provide off-chain monitoring capabilities
 * Errors define custom error codes for better debugging
 */

/// Event emitted when tokens are transferred
#[event]
pub struct TokenTransferEvent {
    pub from: Pubkey,   // Sender's public key
    pub to: Pubkey,     // Receiver's public key
    pub amount: u64,    // Transfer amount
    pub timestamp: i64, // When transfer occurred
}

/// Custom error codes for better error handling
#[error_code]
pub enum CustomError {
    #[msg("Name is too long. Maximum 32 characters allowed.")]
    NameTooLong, // Error code: 6000

    #[msg("Invalid age provided. Age must be greater than 0.")]
    InvalidAge, // Error code: 6001

    #[msg("Invalid transfer amount. Amount must be greater than 0.")]
    InvalidAmount, // Error code: 6002

    #[msg("Insufficient funds for this operation.")]
    InsufficientFunds, // Error code: 6003

    #[msg("Account is not active.")]
    AccountInactive, // Error code: 6004

    #[msg("Account is already inactive.")]
    AccountAlreadyInactive, // Error code: 6005

    #[msg("Mathematical operation resulted in overflow.")]
    MathOverflow, // Error code: 6006
}

/*
 * ============================================================================
 * UTILITY FUNCTIONS AND HELPERS
 * ============================================================================
 *
 * Additional helper functions for common operations
 */

/// Utility function to validate user account state
/// Returns true if account is valid for operations
pub fn is_account_valid(account: &UserAccount) -> bool {
    account.is_active &&           // Must be active
    !account.name.is_empty() &&    // Must have a name
    account.age > 0 // Must have valid age
}

/// Calculate transaction fee based on amount
/// Simple fee calculation for demonstration
pub fn calculate_fee(amount: u64) -> u64 {
    let fee_rate = 100; // 1% fee (100 basis points)
    amount / fee_rate // Simple division for fee calculation
}

// End of file - Total lines include extensive comments for SLOC testing
