use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{
            instruction::Instruction,
            system_program,
        },
        InstructionData,
        ToAccountMetas,
    },
    solana_keypair::read_keypair_file,
    solana_signer::Signer,
};
use solana_client::rpc_client::RpcClient;
use solana_message::{Message, VersionedMessage};
use solana_transaction::versioned::VersionedTransaction;
fn main() {
    let program_id = counter_pda::id();

    let payer = read_keypair_file(
        std::env::var("HOME").unwrap() + "/.config/solana/id.json",
    )
    .expect("No se pudo cargar el keypair");

    let counter = Pubkey::find_program_address(
        &[
            counter_pda::constants::COUNTER_SEED,
            payer.pubkey().as_ref(),
        ],
        &program_id,
    )
    .0;

    println!("Program ID: {}", program_id);
    println!("Authority: {}", payer.pubkey());
    println!("PDA: {}", counter);

    let instruction = Instruction::new_with_bytes(
        program_id,
        &counter_pda::instruction::Initialize {}.data(),
        counter_pda::accounts::Initialize {
            authority: payer.pubkey(),
            counter,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let rpc = RpcClient::new(
        "https://api.devnet.solana.com".to_string()
    );

    let blockhash = rpc
        .get_latest_blockhash()
        .expect("No se pudo obtener el blockhash");

    println!("Initialize instruction creada");
        let message = Message::new_with_blockhash(
        &[instruction],
        Some(&payer.pubkey()),
        &blockhash,
    );

    let transaction = VersionedTransaction::try_new(
        VersionedMessage::Legacy(message),
        &[&payer],
    )
    .expect("No se pudo firmar la transacción");
    let signature = rpc
        .send_and_confirm_transaction(&transaction)
        .expect("La transacción falló");

    println!("Transaction: {}", signature);
}
