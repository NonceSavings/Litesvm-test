use anchor_lang::{prelude, solana_program::native_token::sol_to_lamports};
use anchor_spl::token_interface::spl_token_2022::{
    extension::StateWithExtensions, solana_program::program_pack::Pack,
    state::Account as SplAccount,
};
use anchor_spl::{
    associated_token::spl_associated_token_account,
    token_2022::spl_token_2022::{self},
};
use litesvm::types::{FailedTransactionMetadata, TransactionMetadata};
use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    transaction::{Transaction, TransactionError},
    message::Message,
};
use testt;

pub struct NonceUtils {
    svm_engine: LiteSVM,
    pub payer: Pubkey,
}

pub const ACTORS: [&str; 3] = ["admin", "emeke", "ekenem"];

impl NonceUtils {
    pub fn new() -> Self {
        let svm_engine = LiteSVM::new()
            .with_sigverify(false)
            .with_blockhash_check(false)
            .with_transaction_history(0)
            .with_spl_programs();

        let mut env = Self {
            svm_engine,
            payer: Pubkey::default(),
        };

        env
    }

    pub fn airdrop(&mut self, pubkey: &Pubkey) {
        self.svm_engine
            .airdrop(pubkey, sol_to_lamports(100.0))
            .unwrap();
    }

    pub fn add_token_mill_program(&mut self) {
        self.svm_engine.add_program_from_file(testt::id(), "../../../target/deploy").unwrap();
    }

    #[allow(clippy::result_large_err)]
    pub fn execute_actions(
        &mut self,
        actions: &[&dyn InstructionGenerator],
    ) -> Result<TransactionMetadata, FailedTransactionMetadata> {
        let instructions = actions
            .iter()
            .map(|action| action.instruction())
            .collect::<Vec<_>>();

        self.execute(&instructions)
    }

    #[allow(clippy::result_large_err)]
    pub fn execute(
        &mut self,
        instructions: &[Instruction],
    ) -> Result<TransactionMetadata, FailedTransactionMetadata> {
        let tx = Transaction::new_unsigned(Message::new(instructions, Some(&self.payer)));

        self.svm_engine.send_transaction(tx)
    }
}

pub fn make_address(string: &str) -> Pubkey {
    let mut array: [u8; 32] = [0; 32];

    for (index, byte) in string.bytes().enumerate() {
        array[index] = byte;
    }

    Pubkey::new_from_array(array)
}

pub trait InstructionGenerator {
    fn accounts(&self) -> Vec<AccountMeta>;
    fn instruction(&self) -> Instruction;
}

trait AccountMetaVecExt {
    fn append_payer(&mut self, payer: Pubkey) -> &mut Self;
    fn append_system_program(&mut self) -> &mut Self;
    fn append_token_program(&mut self) -> &mut Self;
    fn append_associated_token_program(&mut self) -> &mut Self;
}

impl AccountMetaVecExt for Vec<AccountMeta> {
    fn append_payer(&mut self, payer: Pubkey) -> &mut Self {
        self.push(AccountMeta::new_readonly(payer, false));

        self
    }

    fn append_system_program(&mut self) -> &mut Self {
        self.push(AccountMeta::new_readonly(
            solana_sdk::system_program::ID,
            false,
        ));
        self
    }

    fn append_token_program(&mut self) -> &mut Self {
        self.push(AccountMeta::new_readonly(anchor_spl::token::ID, false));

        self
    }

    fn append_associated_token_program(&mut self) -> &mut Self {
        self.push(AccountMeta::new_readonly(
            spl_associated_token_account::id(),
            false,
        ));

        self
    }
}
