mod bindings;
use subxt::dynamic::{At, Value};
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    use bindings::api::runtime_types::pallet_minimal_template::pallet::TupleWrapper;

    let query_tuple = bindings::api::storage()
        .template()
        .tuple_storage_map(1u32, 2u32);

    let query_wrapper = bindings::api::storage()
        .template()
        .tuple_wrapper_storage_map(TupleWrapper((1u32, 2u32)));


    println!(
        "tuple: {:?}",
        api.storage().at_latest().await?.fetch(&query_tuple).await?
    );

    println!(
        "tuple wrapper: {:?}",
        api.storage().at_latest().await?.fetch(&query_wrapper).await?
    );
    Ok(())
}
