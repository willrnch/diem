script {
    use diem_framework::diem_governance;
    use std::features;
    use diem_framework::diem_governance::reconfigure;

    fun main(core_resources: &signer) {
        let framework_signer = diem_governance::get_signer_testnet_only(core_resources, @diem_framework);
        diem_governance::initialize_partial_voting(&framework_signer);
        let feature = features::get_partial_governance_voting();
        features::change_feature_flags(&framework_signer, vector[feature], vector[]);
        reconfigure(&framework_signer);
    }
}
