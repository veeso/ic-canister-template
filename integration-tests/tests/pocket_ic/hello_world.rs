use candid::Encode;
use did::State;
use integration_tests::actor::admin;
use integration_tests::{PocketIcTestEnv, TestEnv as _};

#[tokio::test]
async fn test_should_set_and_get_state() {
    let env = PocketIcTestEnv::init().await;

    let canister = env.hello_world();
    let new_state = State {
        name: "test".to_string(),
        value: 1,
    };

    let res = env
        .update::<()>(canister, admin(), "set_state", Encode!(&new_state).unwrap())
        .await;
    assert!(res.is_ok(), "Failed to set state: {:?}", res);
    let res = env
        .query::<State>(canister, admin(), "get_state", Encode!().unwrap())
        .await
        .expect("Failed to get state");

    assert_eq!(res, new_state, "Failed to get state: {:?}", res);
}
