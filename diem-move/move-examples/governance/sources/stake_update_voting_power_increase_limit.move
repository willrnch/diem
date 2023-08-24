script {
    use diem_framework::diem_governance;
    use diem_framework::staking_config;

    fun main(proposal_id: u64) {
        let framework_signer = diem_governance::resolve(proposal_id, @diem_framework);
        // Update voting power increase limit to 10%.
        staking_config::update_voting_power_increase_limit(&framework_signer, 10);
    }
}
