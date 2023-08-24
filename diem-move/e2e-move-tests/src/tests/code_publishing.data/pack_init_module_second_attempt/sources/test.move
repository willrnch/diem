module 0xcafe::test {
    use diem_framework::coin::{Self, Coin};
    use diem_framework::diem_coin::DiemCoin;

    struct State has key {
        important_value: u64,
        coins: Coin<DiemCoin>,
    }

    fun init_module(s: &signer) {
        move_to(s, State {
            important_value: get_value(),
            coins: coin::zero<DiemCoin>(),
        })
    }

    fun get_value(): u64 {
        2
    }
}
