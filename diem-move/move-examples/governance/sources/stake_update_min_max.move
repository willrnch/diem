script {
    use diem_framework::diem_governance;
    use diem_framework::coin;
    use diem_framework::diem_coin::DiemCoin;
    use diem_framework::staking_config;

    fun main(proposal_id: u64) {
        let framework_signer = diem_governance::resolve(proposal_id, @diem_framework);
        let one_diem_coin_with_decimals = 10 ** (coin::decimals<DiemCoin>() as u64);
        // Change min to 1000 and max to 1M Diem coins.
        let new_min_stake = 1000 * one_diem_coin_with_decimals;
        let new_max_stake = 1000000 * one_diem_coin_with_decimals;
        staking_config::update_required_stake(&framework_signer, new_min_stake, new_max_stake);
    }
}
