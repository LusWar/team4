#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod erc20 {
    use ink_core::storage;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct Erc20 {
        total_supply: storage::Value<Balance>,
        balances: storage::HashMap<AccountId, Balance>,
        allowance: storage::HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    struct Transfered {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,

        value: Balance,
    }

    #[ink(event)]
    struct Approved {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,

        value: Balance,
    }

    impl Erc20 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        fn new(&mut self, init_supply: Balance) {
            let caller = self.env().caller();
            self.total_supply.set(init_supply);
            self.balances.insert(caller, init_supply);
            self.env().emit_event(Transfered {
                from: None, to: Some(caller), value: init_supply
            });
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        fn default(&mut self) {
            self.new(1000_000_000_000_000)
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            *self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_or_zero( &owner)
        }

        fn balance_of_or_zero(&self, who: &AccountId) -> Balance {
            *self.balances.get(who).unwrap_or(&0)
        }

        #[ink(message)]
        fn approval(&self, to: &AccountId) -> Balance {
            let from = self.env().caller();
            self.allowance_of_or_zero(&from, to)
        }

        fn allowance_of_or_zero(&self, from: &AccountId, to: &AccountId) -> Balance {
            *self.allowance.get(&(from.clone(), to.clone())).unwrap_or(&0)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller();
            let from_balance = self.balance_of_or_zero(&from);

            if from_balance < value {
                false
            } else {
                let to_balance = self.balance_of_or_zero(&to);
                self.balances.insert(from, from_balance - value);
                self.balances.insert(to, to_balance + value);

                self.env().emit_event(Transfered {
                    from: Some(from), to: Some(to), value
                });

                true
            }
        }

        #[ink(message)]
        fn approve(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller();
            let approval = self.allowance_of_or_zero(&from, &to);
            self.allowance.insert((from, to), approval + value);

            self.env().emit_event(Approved { from, to, value });

            true
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_core::env::AccountId;

        #[test]
        fn total_supply_works() {
            let value = 10000000;
            let erc20 = Erc20::new(value);
            assert_eq!(erc20.total_supply(), value);
        }

        #[test]
        fn transfer_works() {
            let mut erc20 = Erc20::new(100_000);
            let from = AccountId::from([0x01; 32]);
            assert_eq!(erc20.balance_of(from), 100_000);
            let to = AccountId::from([0x02; 32]);
            assert!(erc20.transfer(to, 50_000));
            assert_eq!(erc20.balance_of(from), 50_000);
            assert_eq!(erc20.balance_of(to), 50_000);
        }
    }
}
