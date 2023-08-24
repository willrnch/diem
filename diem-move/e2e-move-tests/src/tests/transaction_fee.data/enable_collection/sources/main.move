script {
    use diem_framework::diem_governance;
    use std::features;

    fun main(core_resources: &signer) {
        let framework_signer = diem_governance::get_signer_testnet_only(core_resources, @diem_framework);
        let feature = features::get_collect_and_distribute_gas_fees_feature();
        features::change_feature_flags(&framework_signer, vector[feature], vector[]);

        // Make sure to trigger a reconfiguration!
        diem_governance::reconfigure(&framework_signer);
    }
}
