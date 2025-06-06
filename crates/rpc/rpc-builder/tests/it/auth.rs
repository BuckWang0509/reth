//! Auth server tests

use crate::utils::launch_auth;
use alloy_primitives::U64;
use alloy_rpc_types_engine::{
    ExecutionPayloadInputV2, ExecutionPayloadV1, ForkchoiceState, PayloadId,
};
use jsonrpsee::core::client::{ClientT, SubscriptionClientT};
use reth_ethereum_engine_primitives::EthEngineTypes;
use reth_ethereum_primitives::{Block, TransactionSigned};
use reth_primitives_traits::block::Block as _;
use reth_rpc_api::clients::EngineApiClient;
use reth_rpc_layer::JwtSecret;

#[expect(unused_must_use)]
async fn test_basic_engine_calls<C>(client: &C)
where
    C: ClientT + SubscriptionClientT + Sync + EngineApiClient<EthEngineTypes>,
{
    let block = Block::default().seal_slow();
    EngineApiClient::new_payload_v1(
        client,
        ExecutionPayloadV1::from_block_unchecked(block.hash(), &block.clone().into_block()),
    )
    .await;
    EngineApiClient::new_payload_v2(
        client,
        ExecutionPayloadInputV2 {
            execution_payload: ExecutionPayloadV1::from_block_slow::<TransactionSigned, _>(
                &block.into_block(),
            ),
            withdrawals: None,
        },
    )
    .await;
    EngineApiClient::fork_choice_updated_v1(client, ForkchoiceState::default(), None).await;
    EngineApiClient::get_payload_v1(client, PayloadId::new([0, 0, 0, 0, 0, 0, 0, 0])).await;
    EngineApiClient::get_payload_v2(client, PayloadId::new([0, 0, 0, 0, 0, 0, 0, 0])).await;
    EngineApiClient::get_payload_bodies_by_hash_v1(client, vec![]).await;
    EngineApiClient::get_payload_bodies_by_range_v1(client, U64::ZERO, U64::from(1u64)).await;
    EngineApiClient::exchange_capabilities(client, vec![]).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_endpoints_http() {
    reth_tracing::init_test_tracing();
    let secret = JwtSecret::random();
    let handle = launch_auth(secret).await;
    let client = handle.http_client();
    test_basic_engine_calls(&client).await
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_endpoints_ws() {
    reth_tracing::init_test_tracing();
    let secret = JwtSecret::random();
    let handle = launch_auth(secret).await;
    let client = handle.ws_client().await;
    test_basic_engine_calls(&client).await
}
