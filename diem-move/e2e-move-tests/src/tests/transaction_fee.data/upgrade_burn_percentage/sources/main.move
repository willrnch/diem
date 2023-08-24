script {
    use diem_framework::diem_governance;
    use diem_framework::transaction_fee;

    fun main(core_resources: &signer, burn_percentage: u8) {
        let framework_signer = diem_governance::get_signer_testnet_only(core_resources, @diem_framework);
        transaction_fee::upgrade_burn_percentage(&framework_signer, burn_percentage);

        // Make sure to trigger a reconfiguration!
        diem_governance::reconfigure(&framework_signer);
    }
}
