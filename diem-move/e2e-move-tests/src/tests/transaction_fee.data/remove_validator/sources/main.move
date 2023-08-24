script {
    use diem_framework::diem_governance;
    use diem_framework::stake;

    fun main(core_resources: &signer, addr: address) {
        let framework_signer = diem_governance::get_signer_testnet_only(core_resources, @diem_framework);
        stake::remove_validators(&framework_signer, &vector[addr]);

        // Make sure to trigger a reconfiguration!
        diem_governance::reconfigure(&framework_signer);
    }
}
