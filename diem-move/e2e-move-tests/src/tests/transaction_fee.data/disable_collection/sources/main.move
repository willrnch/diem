script {
    use diem_framework::diem_governance;
    use std::features;

    fun main(core_resources: &signer) {
        let framework_signer = diem_governance::get_signer_testnet_only(core_resources, @diem_framework);
        let feature = features::get_collect_and_distribute_gas_fees_feature();

        // Trigger reconfiguration first, to also sync all the fees to validators.
        diem_governance::reconfigure(&framework_signer);

        // Then, disable the feature.
        features::change_feature_flags(&framework_signer, vector[], vector[feature]);
    }
}
