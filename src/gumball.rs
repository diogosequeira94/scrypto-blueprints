use scrypto::prelude::*;

#[blueprint]
mod gumball {
    struct Gumball {
        gumball_vault: Vault,
    }

    impl Gumball {
        pub fn new_gumball() -> Global<Gumball> {}
    }
}
