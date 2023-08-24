spec diem_framework::transaction_fee {
    spec module {
        use diem_framework::chain_status;
        pragma verify = true;
        pragma aborts_if_is_strict;

        invariant [suspendable] chain_status::is_operating() ==> exists<DiemCoinCapabilities>(@diem_framework);
    }

    spec CollectedFeesPerBlock {
        invariant burn_percentage <= 100;
    }

    spec initialize_fee_collection_and_distribution(diem_framework: &signer, burn_percentage: u8) {
        use std::signer;
        use diem_framework::stake::ValidatorFees;
        use diem_framework::aggregator_factory;
        use diem_framework::system_addresses;

        aborts_if exists<CollectedFeesPerBlock>(@diem_framework);
        aborts_if burn_percentage > 100;

        let diem_addr = signer::address_of(diem_framework);
        aborts_if !system_addresses::is_diem_framework_address(diem_addr);
        aborts_if exists<ValidatorFees>(diem_addr);

        include system_addresses::AbortsIfNotDiemFramework {account: diem_framework};
        include aggregator_factory::CreateAggregatorInternalAbortsIf;

        ensures exists<ValidatorFees>(diem_addr);
    }

    spec upgrade_burn_percentage(diem_framework: &signer, new_burn_percentage: u8) {
        use std::signer;
        use diem_framework::coin::CoinInfo;
        use diem_framework::diem_coin::DiemCoin;
        // Percentage validation
        aborts_if new_burn_percentage > 100;
        // Signer validation
        let diem_addr = signer::address_of(diem_framework);
        aborts_if !system_addresses::is_diem_framework_address(diem_addr);
        // Requirements of `process_collected_fees`
        requires exists<DiemCoinCapabilities>(@diem_framework);
        requires exists<stake::ValidatorFees>(@diem_framework);
        requires exists<CoinInfo<DiemCoin>>(@diem_framework);
        include RequiresCollectedFeesPerValueLeqBlockDiemSupply;
        // The effect of upgrading the burn percentage
        ensures exists<CollectedFeesPerBlock>(@diem_framework) ==>
            global<CollectedFeesPerBlock>(@diem_framework).burn_percentage == new_burn_percentage;
    }

    spec register_proposer_for_fee_collection(proposer_addr: address) {
        aborts_if false;
        ensures is_fees_collection_enabled() ==>
            option::spec_borrow(global<CollectedFeesPerBlock>(@diem_framework).proposer) == proposer_addr;
    }

    spec burn_coin_fraction(coin: &mut Coin<DiemCoin>, burn_percentage: u8) {
        use diem_framework::optional_aggregator;
        use diem_framework::aggregator;
        use diem_framework::coin::CoinInfo;
        use diem_framework::diem_coin::DiemCoin;
        requires burn_percentage <= 100;
        requires exists<DiemCoinCapabilities>(@diem_framework);
        requires exists<CoinInfo<DiemCoin>>(@diem_framework);
        let amount_to_burn = (burn_percentage * coin::value(coin)) / 100;
        let maybe_supply = coin::get_coin_supply_opt<DiemCoin>();
        aborts_if amount_to_burn > 0 && option::is_some(maybe_supply) && optional_aggregator::is_parallelizable(option::borrow(maybe_supply))
            && aggregator::spec_aggregator_get_val(option::borrow(option::borrow(maybe_supply).aggregator)) <
            amount_to_burn;
        aborts_if option::is_some(maybe_supply) && !optional_aggregator::is_parallelizable(option::borrow(maybe_supply))
            && option::borrow(option::borrow(maybe_supply).integer).value <
            amount_to_burn;
        include (amount_to_burn > 0) ==> coin::AbortsIfNotExistCoinInfo<DiemCoin>;
    }

    spec fun collectedFeesAggregator(): AggregatableCoin<DiemCoin> {
        global<CollectedFeesPerBlock>(@diem_framework).amount
    }

    spec schema RequiresCollectedFeesPerValueLeqBlockDiemSupply {
        use diem_framework::optional_aggregator;
        use diem_framework::aggregator;
        let maybe_supply = coin::get_coin_supply_opt<DiemCoin>();
        requires
            (is_fees_collection_enabled() && option::is_some(maybe_supply)) ==>
                (aggregator::spec_aggregator_get_val(global<CollectedFeesPerBlock>(@diem_framework).amount.value) <=
                    optional_aggregator::optional_aggregator_value(option::spec_borrow(coin::get_coin_supply_opt<DiemCoin>())));
    }

    spec process_collected_fees() {
        use diem_framework::coin::CoinInfo;
        use diem_framework::diem_coin::DiemCoin;
        requires exists<DiemCoinCapabilities>(@diem_framework);
        requires exists<stake::ValidatorFees>(@diem_framework);
        requires exists<CoinInfo<DiemCoin>>(@diem_framework);
        include RequiresCollectedFeesPerValueLeqBlockDiemSupply;
    }

    /// `DiemCoinCapabilities` should be exists.
    spec burn_fee(account: address, fee: u64) {
        use diem_std::type_info;
        use diem_framework::optional_aggregator;
        use diem_framework::coin::{CoinInfo, CoinStore};


        aborts_if !exists<DiemCoinCapabilities>(@diem_framework);

        // This function essentially calls `coin::burn_coin`, monophormized for `DiemCoin`.
        let account_addr = account;
        let amount = fee;

        let diem_addr = type_info::type_of<DiemCoin>().account_address;
        let coin_store = global<CoinStore<DiemCoin>>(account_addr);
        let post post_coin_store = global<CoinStore<DiemCoin>>(account_addr);

        modifies global<CoinInfo<DiemCoin>>(diem_addr);
        modifies global<CoinStore<DiemCoin>>(account_addr);

        aborts_if amount != 0 && !(exists<CoinInfo<DiemCoin>>(diem_addr)
            && exists<CoinStore<DiemCoin>>(account_addr));
        aborts_if coin_store.coin.value < amount;

        let maybe_supply = global<CoinInfo<DiemCoin>>(diem_addr).supply;
        let supply = option::spec_borrow(maybe_supply);
        let value = optional_aggregator::optional_aggregator_value(supply);

        let post post_maybe_supply = global<CoinInfo<DiemCoin>>(diem_addr).supply;
        let post post_supply = option::spec_borrow(post_maybe_supply);
        let post post_value = optional_aggregator::optional_aggregator_value(post_supply);

        aborts_if option::spec_is_some(maybe_supply) && value < amount;

        ensures post_coin_store.coin.value == coin_store.coin.value - amount;
        ensures if (option::spec_is_some(maybe_supply)) {
            post_value == value - amount
        } else {
            option::spec_is_none(post_maybe_supply)
        };
    }

    spec collect_fee(account: address, fee: u64) {
        use diem_framework::aggregator;
        let collected_fees = global<CollectedFeesPerBlock>(@diem_framework).amount;
        let aggr = collected_fees.value;
        aborts_if !exists<CollectedFeesPerBlock>(@diem_framework);
        aborts_if fee > 0 && !exists<coin::CoinStore<DiemCoin>>(account);
        aborts_if fee > 0 && global<coin::CoinStore<DiemCoin>>(account).coin.value < fee;
        aborts_if fee > 0 && aggregator::spec_aggregator_get_val(aggr)
            + fee > aggregator::spec_get_limit(aggr);
        aborts_if fee > 0 && aggregator::spec_aggregator_get_val(aggr)
            + fee > MAX_U128;
    }

    /// Ensure caller is admin.
    /// Aborts if `DiemCoinCapabilities` already exists.
    spec store_diem_coin_burn_cap(diem_framework: &signer, burn_cap: BurnCapability<DiemCoin>) {
        use std::signer;
        let addr = signer::address_of(diem_framework);
        aborts_if !system_addresses::is_diem_framework_address(addr);
        aborts_if exists<DiemCoinCapabilities>(addr);
        ensures exists<DiemCoinCapabilities>(addr);
    }
}
