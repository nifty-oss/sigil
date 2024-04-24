//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Transfer {
    /// The account paying for the storage fees.
    pub payer: Option<solana_program::pubkey::Pubkey>,
    /// The pubkey of the user associated with the token account
    pub user: solana_program::pubkey::Pubkey,
    /// The recipient account.
    pub recipient: solana_program::pubkey::Pubkey,
    /// The mint account for the token to be transferred
    pub mint: solana_program::pubkey::Pubkey,
    /// The token authority account.
    pub user_token_account: solana_program::pubkey::Pubkey,
    /// The token authority account.
    pub recipient_token_account: solana_program::pubkey::Pubkey,
    /// The system program
    pub system_program: Option<solana_program::pubkey::Pubkey>,
}

impl Transfer {
    pub fn instruction(
        &self,
        args: TransferInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: TransferInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        if let Some(payer) = self.payer {
            accounts.push(solana_program::instruction::AccountMeta::new(payer, true));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::TOKEN_LITE_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.user, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.recipient,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.mint, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.recipient_token_account,
            false,
        ));
        if let Some(system_program) = self.system_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                system_program,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::TOKEN_LITE_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let mut data = TransferInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::TOKEN_LITE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TransferInstructionData {
    discriminator: u8,
}

impl TransferInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 5 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransferInstructionArgs {
    pub amount: u32,
}

/// Instruction builder for `Transfer`.
///
/// ### Accounts:
///
///   0. `[writable, signer, optional]` payer
///   1. `[signer]` user
///   2. `[]` recipient
///   3. `[]` mint
///   4. `[writable]` user_token_account
///   5. `[writable]` recipient_token_account
///   6. `[optional]` system_program
#[derive(Default)]
pub struct TransferBuilder {
    payer: Option<solana_program::pubkey::Pubkey>,
    user: Option<solana_program::pubkey::Pubkey>,
    recipient: Option<solana_program::pubkey::Pubkey>,
    mint: Option<solana_program::pubkey::Pubkey>,
    user_token_account: Option<solana_program::pubkey::Pubkey>,
    recipient_token_account: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    amount: Option<u32>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl TransferBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// `[optional account]`
    /// The account paying for the storage fees.
    #[inline(always)]
    pub fn payer(&mut self, payer: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.payer = payer;
        self
    }
    /// The pubkey of the user associated with the token account
    #[inline(always)]
    pub fn user(&mut self, user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user = Some(user);
        self
    }
    /// The recipient account.
    #[inline(always)]
    pub fn recipient(&mut self, recipient: solana_program::pubkey::Pubkey) -> &mut Self {
        self.recipient = Some(recipient);
        self
    }
    /// The mint account for the token to be transferred
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }
    /// The token authority account.
    #[inline(always)]
    pub fn user_token_account(
        &mut self,
        user_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.user_token_account = Some(user_token_account);
        self
    }
    /// The token authority account.
    #[inline(always)]
    pub fn recipient_token_account(
        &mut self,
        recipient_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.recipient_token_account = Some(recipient_token_account);
        self
    }
    /// `[optional account]`
    /// The system program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.system_program = system_program;
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u32) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Transfer {
            payer: self.payer,
            user: self.user.expect("user is not set"),
            recipient: self.recipient.expect("recipient is not set"),
            mint: self.mint.expect("mint is not set"),
            user_token_account: self
                .user_token_account
                .expect("user_token_account is not set"),
            recipient_token_account: self
                .recipient_token_account
                .expect("recipient_token_account is not set"),
            system_program: self.system_program,
        };
        let args = TransferInstructionArgs {
            amount: self.amount.clone().expect("amount is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `transfer` CPI accounts.
pub struct TransferCpiAccounts<'a, 'b> {
    /// The account paying for the storage fees.
    pub payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The pubkey of the user associated with the token account
    pub user: &'b solana_program::account_info::AccountInfo<'a>,
    /// The recipient account.
    pub recipient: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint account for the token to be transferred
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The token authority account.
    pub user_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The token authority account.
    pub recipient_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The system program
    pub system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `transfer` CPI instruction.
pub struct TransferCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees.
    pub payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The pubkey of the user associated with the token account
    pub user: &'b solana_program::account_info::AccountInfo<'a>,
    /// The recipient account.
    pub recipient: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint account for the token to be transferred
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The token authority account.
    pub user_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The token authority account.
    pub recipient_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The system program
    pub system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The arguments for the instruction.
    pub __args: TransferInstructionArgs,
}

impl<'a, 'b> TransferCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: TransferCpiAccounts<'a, 'b>,
        args: TransferInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            payer: accounts.payer,
            user: accounts.user,
            recipient: accounts.recipient,
            mint: accounts.mint,
            user_token_account: accounts.user_token_account,
            recipient_token_account: accounts.recipient_token_account,
            system_program: accounts.system_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        if let Some(payer) = self.payer {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *payer.key, true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::TOKEN_LITE_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.user.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.recipient.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.recipient_token_account.key,
            false,
        ));
        if let Some(system_program) = self.system_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *system_program.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::TOKEN_LITE_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = TransferInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::TOKEN_LITE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        if let Some(payer) = self.payer {
            account_infos.push(payer.clone());
        }
        account_infos.push(self.user.clone());
        account_infos.push(self.recipient.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.user_token_account.clone());
        account_infos.push(self.recipient_token_account.clone());
        if let Some(system_program) = self.system_program {
            account_infos.push(system_program.clone());
        }
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Transfer` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer, optional]` payer
///   1. `[signer]` user
///   2. `[]` recipient
///   3. `[]` mint
///   4. `[writable]` user_token_account
///   5. `[writable]` recipient_token_account
///   6. `[optional]` system_program
pub struct TransferCpiBuilder<'a, 'b> {
    instruction: Box<TransferCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> TransferCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(TransferCpiBuilderInstruction {
            __program: program,
            payer: None,
            user: None,
            recipient: None,
            mint: None,
            user_token_account: None,
            recipient_token_account: None,
            system_program: None,
            amount: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// `[optional account]`
    /// The account paying for the storage fees.
    #[inline(always)]
    pub fn payer(
        &mut self,
        payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.payer = payer;
        self
    }
    /// The pubkey of the user associated with the token account
    #[inline(always)]
    pub fn user(&mut self, user: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.user = Some(user);
        self
    }
    /// The recipient account.
    #[inline(always)]
    pub fn recipient(
        &mut self,
        recipient: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.recipient = Some(recipient);
        self
    }
    /// The mint account for the token to be transferred
    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }
    /// The token authority account.
    #[inline(always)]
    pub fn user_token_account(
        &mut self,
        user_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_token_account = Some(user_token_account);
        self
    }
    /// The token authority account.
    #[inline(always)]
    pub fn recipient_token_account(
        &mut self,
        recipient_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.recipient_token_account = Some(recipient_token_account);
        self
    }
    /// `[optional account]`
    /// The system program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.system_program = system_program;
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u32) -> &mut Self {
        self.instruction.amount = Some(amount);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = TransferInstructionArgs {
            amount: self.instruction.amount.clone().expect("amount is not set"),
        };
        let instruction = TransferCpi {
            __program: self.instruction.__program,

            payer: self.instruction.payer,

            user: self.instruction.user.expect("user is not set"),

            recipient: self.instruction.recipient.expect("recipient is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

            user_token_account: self
                .instruction
                .user_token_account
                .expect("user_token_account is not set"),

            recipient_token_account: self
                .instruction
                .recipient_token_account
                .expect("recipient_token_account is not set"),

            system_program: self.instruction.system_program,
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct TransferCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    recipient: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    recipient_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    amount: Option<u32>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
