script {
    use diem_framework::diem_governance;
    use diem_framework::transaction_fee;

    fun main(core_resources: &signer, burn_percentage: u8) {
        let framework_signer = diem_governance::get_signer_testnet_only(core_resources, @diem_framework);
        transaction_fee::initialize_fee_collection_and_distribution(&framework_signer, burn_percentage);
        // Since this only places resources on the core account, no reconfiguration is needed.
    }
}
