use anchor_lang::prelude::*;
pub mod errors;
pub mod state;
pub mod config;
pub mod constants;

use state::*;

declare_id!("5r77gCrW8ifM3Qn8KUE4zLHFmbUNNPmL3ejnL1zBNYrD");

#[program]
pub mod testt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[cfg(test)]
mod test {
    use super::*;
    use litesvm::LiteSVM;
    use solana_program::{message::Message, pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

    #[test]
    fn testing() {
        let from_keypair = Keypair::new();
        let from = from_keypair.pubkey();
        let to = Pubkey::new_unique();

        let mut svm = LiteSVM::new();
        svm.airdrop(&from, 10_000).unwrap();

        let instruction = transfer(&from, &to, 64);
        let tx = Transaction::new(
            &[&from_keypair],
            Message::new(&[instruction], Some(&from)),
            svm.latest_blockhash(),
        );
        let tx_res = svm.send_transaction(tx).unwrap();
        println!("{:?}", tx_res);

        let from_account = svm.get_account(&from);
        let to_account = svm.get_account(&to);
        assert_eq!(from_account.unwrap().lamports, 4936);
        assert_eq!(to_account.unwrap().lamports, 64);
    }

    #[test]
    fn initialize() {}
}
