use subxt::{
    config::PolkadotExtrinsicParamsBuilder as Params,
    ext::sp_core::{sr25519, Pair},
    tx::{PairSigner, TxStatus},
    OnlineClient, PolkadotConfig as EntropyConfig,
};

#[subxt::subxt(runtime_metadata_path = "entropy_metadata.scale")]
pub mod entropy {}

#[tokio::main]
async fn main() {
    // menomnic for stash address
    let mnemonic = "//Alice//stash";
    // set to new endpoint
    let new_endpoint = "test";
    let url_of_chain = "ws://127.0.0.1:9944";
    let api = OnlineClient::<EntropyConfig>::from_url(url_of_chain)
        .await
        .unwrap();
    let pair = <sr25519::Pair as Pair>::from_string(mnemonic, None).unwrap();
    let signer = PairSigner::<EntropyConfig, sr25519::Pair>::new(pair);
    let change_endpoint_tx = entropy::tx()
        .staking_extension()
        .change_endpoint(new_endpoint.into());
    let latest_block = api.blocks().at_latest().await.unwrap();
    let tx_params = Params::new().mortal(latest_block.header(), 32u64).build();
    let mut tx = api
        .tx()
        .create_signed(&change_endpoint_tx, &signer, tx_params)
        .await
        .unwrap()
        .submit_and_watch()
        .await
        .unwrap();
    while let Some(status) = tx.next().await {
        match status.unwrap() {
            TxStatus::InBestBlock(tx_in_block) | TxStatus::InFinalizedBlock(tx_in_block) => {
                // now, we can attempt to work with the block, eg:
                tx_in_block.wait_for_success().await.unwrap();
            }
            TxStatus::Error { message }
            | TxStatus::Invalid { message }
            | TxStatus::Dropped { message } => {
                // Handle any errors:
                println!("Error submitting tx: {message}");
            }
            // Continue otherwise:
            _ => continue,
        }
    }
}
