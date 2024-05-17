//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct MintTo {
    /// The token authority account.
    pub token_account: solana_program::pubkey::Pubkey,
    /// The mint account PDA derived from the ticker and authority.
    pub mint: solana_program::pubkey::Pubkey,
    /// The authority for the mint.
    pub authority: solana_program::pubkey::Pubkey,
    /// The account paying for the storage fees.
    pub payer: Option<solana_program::pubkey::Pubkey>,
    /// The system program
    pub system_program: Option<solana_program::pubkey::Pubkey>,
}

impl MintTo {
    pub fn instruction(
        &self,
        args: MintToInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: MintToInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.mint, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.authority,
            true,
        ));
        if let Some(payer) = self.payer {
            accounts.push(solana_program::instruction::AccountMeta::new(payer, true));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::TOKEN_LITE_ID,
                false,
            ));
        }
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
        let mut data = MintToInstructionData::new().try_to_vec().unwrap();
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
pub struct MintToInstructionData {
    discriminator: u8,
}

impl MintToInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 5 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MintToInstructionArgs {
    pub amount: u32,
}

/// Instruction builder for `MintTo`.
///
/// ### Accounts:
///
///   0. `[writable]` token_account
///   1. `[writable]` mint
///   2. `[writable, signer]` authority
///   3. `[writable, signer, optional]` payer
///   4. `[optional]` system_program
#[derive(Default)]
pub struct MintToBuilder {
    token_account: Option<solana_program::pubkey::Pubkey>,
    mint: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    amount: Option<u32>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl MintToBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The token authority account.
    #[inline(always)]
    pub fn token_account(&mut self, token_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_account = Some(token_account);
        self
    }
    /// The mint account PDA derived from the ticker and authority.
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }
    /// The authority for the mint.
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    /// `[optional account]`
    /// The account paying for the storage fees.
    #[inline(always)]
    pub fn payer(&mut self, payer: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.payer = payer;
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
        let accounts = MintTo {
            token_account: self.token_account.expect("token_account is not set"),
            mint: self.mint.expect("mint is not set"),
            authority: self.authority.expect("authority is not set"),
            payer: self.payer,
            system_program: self.system_program,
        };
        let args = MintToInstructionArgs {
            amount: self.amount.clone().expect("amount is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `mint_to` CPI accounts.
pub struct MintToCpiAccounts<'a, 'b> {
    /// The token authority account.
    pub token_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint account PDA derived from the ticker and authority.
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority for the mint.
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees.
    pub payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `mint_to` CPI instruction.
pub struct MintToCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The token authority account.
    pub token_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint account PDA derived from the ticker and authority.
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority for the mint.
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees.
    pub payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The arguments for the instruction.
    pub __args: MintToInstructionArgs,
}

impl<'a, 'b> MintToCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: MintToCpiAccounts<'a, 'b>,
        args: MintToInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            token_account: accounts.token_account,
            mint: accounts.mint,
            authority: accounts.authority,
            payer: accounts.payer,
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
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.authority.key,
            true,
        ));
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
        let mut data = MintToInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::TOKEN_LITE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.token_account.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.authority.clone());
        if let Some(payer) = self.payer {
            account_infos.push(payer.clone());
        }
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

/// Instruction builder for `MintTo` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` token_account
///   1. `[writable]` mint
///   2. `[writable, signer]` authority
///   3. `[writable, signer, optional]` payer
///   4. `[optional]` system_program
pub struct MintToCpiBuilder<'a, 'b> {
    instruction: Box<MintToCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> MintToCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(MintToCpiBuilderInstruction {
            __program: program,
            token_account: None,
            mint: None,
            authority: None,
            payer: None,
            system_program: None,
            amount: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The token authority account.
    #[inline(always)]
    pub fn token_account(
        &mut self,
        token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_account = Some(token_account);
        self
    }
    /// The mint account PDA derived from the ticker and authority.
    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }
    /// The authority for the mint.
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
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
        let args = MintToInstructionArgs {
            amount: self.instruction.amount.clone().expect("amount is not set"),
        };
        let instruction = MintToCpi {
            __program: self.instruction.__program,

            token_account: self
                .instruction
                .token_account
                .expect("token_account is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            payer: self.instruction.payer,

            system_program: self.instruction.system_program,
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct MintToCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    amount: Option<u32>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
