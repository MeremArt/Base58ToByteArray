
pub mod Turbin3_prereq;

// Import the structs and methods we need to test
#[cfg(test)]
mod tests {
    use super::*; // Brings the main module into scope
    use solana_sdk::signature::{read_keypair_file, Signer};
    use solana_sdk::system_program;
    use solana_client::rpc_client::RpcClient;
    use crate::programs::Turbin3_prereq::{WbaPrereqProgram, UpdateArgs};
    

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn example_function() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

        let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq",
            signer.pubkey().to_bytes().as_ref()
        ]);

        let args: UpdateArgs = UpdateArgs {
            github: b"MeremArt".to_vec(),
        };

        let blockhash = rpc_client.get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = WbaPrereqProgram::update(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash
        );

        let signature = rpc_client.send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}

