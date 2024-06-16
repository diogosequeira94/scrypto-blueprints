use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub struct Status {
    pub price: Decimal,
    pub amount: Decimal,
}

#[blueprint]
mod gumball {

    struct GumballMachine {
        gumballs: Vault,
        gumball_xrd_vault: Vault,
        price: Decimal,
    }

    impl GumballMachine {
        pub fn instantiate_gumball_machine(price: Decimal) -> Global<GumballMachine> {
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
                         // Mint initial supply may return a type that isn't directly a "Bucket"
            Self {
                gumballs: Vault::with_bucket(bucket_of_gumballs),
                gumball_xrd_vault: Vault::new(XRD),
                price: price,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn get_price(&self) -> Decimal {
            self.price
        }

        pub fn get_status(&self) -> Status {
            Status {
                price: self.price,
                amount: self.gumballs.amount(),
            }
        }

        pub fn buy_gumball(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            // Check if the payment contains at least the required price
            if payment.amount() < self.price {
                panic!(
                    "Insufficient funds: Required {}, but received {}",
                    self.price,
                    payment.amount()
                );
            }

            // take our price in XRD out of the payment
            // if the caller has sent too few, or sent something other than XRD, they'll get a runtime error
            let our_share = payment.take(self.price);
            self.gumball_xrd_vault.put(our_share);

            // we could have simplified the above into a single line, like so:
            // self.collected_xrd.put(payment.take(self.price));

            // return a tuple containing a gumball, plus whatever change is left on the input payment (if any)
            // if we're out of gumballs to give, we'll see a runtime error when we try to grab one
            (self.gumballs.take(1), payment)
        }
    }
}
