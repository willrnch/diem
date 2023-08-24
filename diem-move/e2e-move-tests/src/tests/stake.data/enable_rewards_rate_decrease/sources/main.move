script {
    use diem_framework::diem_governance;
    use diem_framework::staking_config;
    use diem_framework::timestamp;
    use diem_std::fixed_point64;
    use std::features;
    use diem_framework::diem_governance::reconfigure;

    fun main(core_resources: &signer) {
        let framework_signer = diem_governance::get_signer_testnet_only(core_resources, @diem_framework);
        staking_config::initialize_rewards(
            &framework_signer,
            fixed_point64::create_from_rational(1, 100),
            fixed_point64::create_from_rational(3, 1000),
            365 * 24 * 60 * 60,
            timestamp::now_seconds(),
            fixed_point64::create_from_rational(50, 100),
        );
        let feature = features::get_periodical_reward_rate_decrease_feature();
        features::change_feature_flags(&framework_signer, vector[feature], vector[]);
        reconfigure(&framework_signer);
    }
}
