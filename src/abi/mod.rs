use ethers::prelude::abigen;

abigen!(
    ROUTER,
    "src/abi/IUniswapV2Router.abi",
    event_derives(serde::Deserialize, serde::Serialize)
);
