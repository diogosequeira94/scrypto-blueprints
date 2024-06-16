use scrypto::prelude::*;

#[blueprint]
mod gumball {
    struct GumballMachine {
        gumballs: Vault,
        gumball_xrd_vault: Vault,
        price: Decimal,
    }

    impl GumballMachine {
        pub fn instantiate_gumball_machine(price: Decimal) -> ComponentAddress {
            let bucket_of_gumballs: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE) // Individual gumballs, cannot be a % of a gumball.
                .metadata(metadata!(
                    init {
                        "name" => "Gumball", locked; // All the metadata is locked, cannot be changed after initialization
                        "symbol" => "GUM", locked;
                        "description" => "A delicious gumball", locked;
                    }
                ))
                .mint_initial_supply(100)
                .into(); // Into converts this Resource builder into a bucket;
                         // this because mint initial supply may return a type that isn't directly a "Bucket"

            Self {
                gumballs: Vault::with_bucket(bucket_of_gumballs),
                gumball_xrd_vault: Vault::new(XRD),
                price: price,
            }

            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
            
        }
    }
}
