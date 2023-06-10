fn main() {}

#[cfg(test)]
mod test {
    use ethers::abi::AbiDecode;
    use ethers::types::{Address, Bytes, U256};
    use ethers_gist::abi::SwapExactTokensForETHSupportingFeeOnTransferTokensCall;

    #[test]
    fn test_decode() {
        // We decode this transaction. https://etherscan.io/tx/0x8a8cc6e1ad1d452adb9d6dbce3a8d9718d5e28c5472098a726c2941391c5a31f
        // Expect whatever we see to be there
        let input_data = "0x791ac94700000000000000000000000000000000000000007125f332914bef5de8e4000000000000000000000000000000000000000000000000000002fb9784fe51ca3900000000000000000000000000000000000000000000000000000000000000a000000000000000000000000088fe39999348e33956feb18f84dfe0241a33481b0000000000000000000000000000000000000000000000000000000064844cbc00000000000000000000000000000000000000000000000000000000000000020000000000000000000000006ee9742d17b527e682248dca85952e4fe190061d000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
            .parse::<Bytes>().unwrap();
        /*
        We use abigen to help us. Since the function called on chain is swapExactTokensForETHSupportingFeeOnTransferTokens,
        We use the SwapExactTokensForETHSupportingFeeOnTransferTokensCall (note the capitalized first character and the suffix Call) which is generated by abigen! macro
        */
        let decoded =
            SwapExactTokensForETHSupportingFeeOnTransferTokensCall::decode(&input_data).unwrap();
        // Then whatever variables is available based on the abi provided in the IUniswapV2Router.abi can just be called
        let amount_in: U256 = decoded.amount_in;
        let amount_out_min: U256 = decoded.amount_out_min;
        let path: Vec<Address> = decoded.path;
        let to: Address = decoded.to;
        let deadline: U256 = decoded.deadline;

        // Check against the values that etherscan already decoded for us
        assert_eq!(U256::from(35017684833000000000000000000_i128), amount_in);
        assert_eq!(U256::from(214932004688480825_i128), amount_out_min);
        assert_eq!(
            vec![
                "0x6Ee9742d17B527e682248DCA85952e4Fe190061d"
                    .parse::<Address>()
                    .unwrap(),
                "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
                    .parse::<Address>()
                    .unwrap()
            ],
            path
        );
        assert_eq!(
            "0x88fE39999348e33956feb18f84dFe0241A33481b"
                .parse::<Address>()
                .unwrap(),
            to
        );
        assert_eq!(U256::from(1686391996), deadline);
    }
}