#[test_only]
module marketplace::test_utils {
    use std::signer;
    use std::string;
    use std::vector;

    use diem_framework::account;
    use diem_framework::diem_coin::{Self, DiemCoin};
    use diem_framework::coin;
    use diem_framework::object::{Self, Object};
    use diem_framework::timestamp;

    use diem_token::token as tokenv1;
    use diem_token_objects::token::Token;
    use diem_token_objects::diem_token;

    use marketplace::fee_schedule::{Self, FeeSchedule};

    public inline fun setup(
        diem_framework: &signer,
        marketplace: &signer,
        seller: &signer,
        purchaser: &signer,
    ): (address, address, address) {
        timestamp::set_time_has_started_for_testing(diem_framework);
        let (burn_cap, mint_cap) = diem_coin::initialize_for_test(diem_framework);

        let marketplace_addr = signer::address_of(marketplace);
        account::create_account_for_test(marketplace_addr);
        coin::register<DiemCoin>(marketplace);

        let seller_addr = signer::address_of(seller);
        account::create_account_for_test(seller_addr);
        coin::register<DiemCoin>(seller);

        let purchaser_addr = signer::address_of(purchaser);
        account::create_account_for_test(purchaser_addr);
        coin::register<DiemCoin>(purchaser);

        let coins = coin::mint(10000, &mint_cap);
        coin::deposit(seller_addr, coins);
        let coins = coin::mint(10000, &mint_cap);
        coin::deposit(purchaser_addr, coins);

        coin::destroy_burn_cap(burn_cap);
        coin::destroy_mint_cap(mint_cap);

        (marketplace_addr, seller_addr, purchaser_addr)
    }

    public fun fee_schedule(seller: &signer): Object<FeeSchedule> {
        fee_schedule::init(
            seller,
            signer::address_of(seller),
            2,
            1,
            100,
            1,
        )
    }

    public inline fun increment_timestamp(seconds: u64) {
        timestamp::update_global_time_for_test(timestamp::now_microseconds() + (seconds * 1000000));
    }

    public fun mint_tokenv2(seller: &signer): Object<Token> {
        let seller_addr = signer::address_of(seller);
        let collection_name = string::utf8(b"collection_name");
        let token_creation_num = account::get_guid_next_creation_num(seller_addr);

        diem_token::create_collection(
            seller,
            string::utf8(b"collection description"),
            1,
            collection_name,
            string::utf8(b"collection uri"),
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            1,
            100,
        );

        diem_token::mint(
            seller,
            collection_name,
            string::utf8(b"description"),
            string::utf8(b"token_name"),
            string::utf8(b"uri"),
            vector::empty(),
            vector::empty(),
            vector::empty(),
        );

        let obj_addr = object::create_guid_object_address(seller_addr, token_creation_num);
        object::address_to_object(obj_addr)
    }

    public fun mint_tokenv1(seller: &signer): tokenv1::TokenId {
        let collection_name = string::utf8(b"collection_name");
        let token_name = string::utf8(b"token_name");

        tokenv1::create_collection(
            seller,
            collection_name,
            string::utf8(b"Collection: Hello, World"),
            string::utf8(b"https://diem.dev"),
            1,
            vector[true, true, true],
        );

        tokenv1::create_token_script(
            seller,
            collection_name,
            token_name,
            string::utf8(b"Hello, Token"),
            1,
            1,
            string::utf8(b"https://diem.dev"),
            signer::address_of(seller),
            100,
            1,
            vector[true, true, true, true, true],
            vector::empty(),
            vector::empty(),
            vector::empty(),
        );

        tokenv1::create_token_id_raw(
            signer::address_of(seller),
            collection_name,
            token_name,
            0,
        )
    }
}
