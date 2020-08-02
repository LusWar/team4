#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod erc20 {
    use ink_core::storage;

    #[ink(storage)]
    struct Erc20 {
        total_supply: storage::Value<Balance>,
        balances: storage::HashMap<AccountId, Balance>,
        allowance: storage::HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    impl Erc20 {

        #[ink(constructor)]
        fn new(&mut self, initial_supply: Balance) {
            let caller = self.env().caller();
            self.total_supply.set(initial_supply);
            self.balances.insert(caller, initial_supply);
            self.env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply
            });
        }

        #[ink(message)]
        fn total_supply(&mut self) -> Balance {
            *self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            *self.balances.get(owner).unwrap_or(&0)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller();
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false;
            }
            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(from, from_balance - value);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            true
        }

        #[ink(message)]
        fn approve(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller();
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false;
            }
            self.allowance.insert((from, to), value);
            true
        }

        #[ink(message)]
        fn approval(&self, to: AccountId) -> Balance {
            let owner = self.env().caller();
            *self.allowance.get(&(owner, to)).unwrap_or(&0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn new_works() {
            let mut erc20 = Erc20::new(666);
            assert_eq!(erc20.total_supply(), 666);
        }

        #[test]
        fn approve_works() {
            let mut erc20 = Erc20::new(888);
            let account = AccountId::from([0x0; 32]);
            assert_eq!(erc20.approve(account, 889), false);
            assert_eq!(erc20.approve(account, 666), true);
            assert_eq!(erc20.approval(account), 666);
        }
    }
}
