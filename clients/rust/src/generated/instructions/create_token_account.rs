//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CreateTokenAccount {
    /// The account paying for the storage fees.
    pub payer: solana_program::pubkey::Pubkey,
    /// The namespace for the token account.
    pub namespace: solana_program::pubkey::Pubkey,
    /// The pubkey of the user associated with the token account
    pub user: solana_program::pubkey::Pubkey,
    /// The token namespace account.
    pub token_account: solana_program::pubkey::Pubkey,
    /// The system program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl CreateTokenAccount {
    pub fn instruction(
        &self,
        args: CreateTokenAccountInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateTokenAccountInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.namespace,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.user, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateTokenAccountInstructionData::new()
            .try_to_vec()
            .unwrap();
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
struct CreateTokenAccountInstructionData {
    discriminator: u8,
}

impl CreateTokenAccountInstructionData {
    fn new() -> Self {
        Self { discriminator: 1 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTokenAccountInstructionArgs {
    pub capacity: u8,
}

/// Instruction builder for `CreateTokenAccount`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` payer
///   1. `[]` namespace
///   2. `[]` user
///   3. `[writable]` token_account
///   4. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Default)]
pub struct CreateTokenAccountBuilder {
    payer: Option<solana_program::pubkey::Pubkey>,
    namespace: Option<solana_program::pubkey::Pubkey>,
    user: Option<solana_program::pubkey::Pubkey>,
    token_account: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    capacity: Option<u8>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateTokenAccountBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The account paying for the storage fees.
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// The namespace for the token account.
    #[inline(always)]
    pub fn namespace(&mut self, namespace: solana_program::pubkey::Pubkey) -> &mut Self {
        self.namespace = Some(namespace);
        self
    }
    /// The pubkey of the user associated with the token account
    #[inline(always)]
    pub fn user(&mut self, user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user = Some(user);
        self
    }
    /// The token namespace account.
    #[inline(always)]
    pub fn token_account(&mut self, token_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_account = Some(token_account);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// The system program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn capacity(&mut self, capacity: u8) -> &mut Self {
        self.capacity = Some(capacity);
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
        let accounts = CreateTokenAccount {
            payer: self.payer.expect("payer is not set"),
            namespace: self.namespace.expect("namespace is not set"),
            user: self.user.expect("user is not set"),
            token_account: self.token_account.expect("token_account is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = CreateTokenAccountInstructionArgs {
            capacity: self.capacity.clone().expect("capacity is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_token_account` CPI accounts.
pub struct CreateTokenAccountCpiAccounts<'a, 'b> {
    /// The account paying for the storage fees.
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The namespace for the token account.
    pub namespace: &'b solana_program::account_info::AccountInfo<'a>,
    /// The pubkey of the user associated with the token account
    pub user: &'b solana_program::account_info::AccountInfo<'a>,
    /// The token namespace account.
    pub token_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_token_account` CPI instruction.
pub struct CreateTokenAccountCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees.
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The namespace for the token account.
    pub namespace: &'b solana_program::account_info::AccountInfo<'a>,
    /// The pubkey of the user associated with the token account
    pub user: &'b solana_program::account_info::AccountInfo<'a>,
    /// The token namespace account.
    pub token_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateTokenAccountInstructionArgs,
}

impl<'a, 'b> CreateTokenAccountCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateTokenAccountCpiAccounts<'a, 'b>,
        args: CreateTokenAccountInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            payer: accounts.payer,
            namespace: accounts.namespace,
            user: accounts.user,
            token_account: accounts.token_account,
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
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.namespace.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.user.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CreateTokenAccountInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::TOKEN_LITE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.namespace.clone());
        account_infos.push(self.user.clone());
        account_infos.push(self.token_account.clone());
        account_infos.push(self.system_program.clone());
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

/// Instruction builder for `CreateTokenAccount` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` payer
///   1. `[]` namespace
///   2. `[]` user
///   3. `[writable]` token_account
///   4. `[]` system_program
pub struct CreateTokenAccountCpiBuilder<'a, 'b> {
    instruction: Box<CreateTokenAccountCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateTokenAccountCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateTokenAccountCpiBuilderInstruction {
            __program: program,
            payer: None,
            namespace: None,
            user: None,
            token_account: None,
            system_program: None,
            capacity: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The account paying for the storage fees.
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// The namespace for the token account.
    #[inline(always)]
    pub fn namespace(
        &mut self,
        namespace: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.namespace = Some(namespace);
        self
    }
    /// The pubkey of the user associated with the token account
    #[inline(always)]
    pub fn user(&mut self, user: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.user = Some(user);
        self
    }
    /// The token namespace account.
    #[inline(always)]
    pub fn token_account(
        &mut self,
        token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_account = Some(token_account);
        self
    }
    /// The system program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn capacity(&mut self, capacity: u8) -> &mut Self {
        self.instruction.capacity = Some(capacity);
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
        let args = CreateTokenAccountInstructionArgs {
            capacity: self
                .instruction
                .capacity
                .clone()
                .expect("capacity is not set"),
        };
        let instruction = CreateTokenAccountCpi {
            __program: self.instruction.__program,

            payer: self.instruction.payer.expect("payer is not set"),

            namespace: self.instruction.namespace.expect("namespace is not set"),

            user: self.instruction.user.expect("user is not set"),

            token_account: self
                .instruction
                .token_account
                .expect("token_account is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct CreateTokenAccountCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    namespace: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    capacity: Option<u8>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
