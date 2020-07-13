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
        total_supply:storage::Value<Balance>,
        balances:storage::HashMap<AccountId, Balance>,
        allowance: storage::HashMap<(AccountId,AccountId), Balance>,
        //value: storage::Value<bool>,
    }

    #[ink(event)]
    struct Transfer{
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value:Balance,
    }
    #[ink(event)]
    struct Approve{
        #[ink(topic)]
        approver: Option<AccountId>,
        #[ink(topic)]
        user: Option<AccountId>,
        amount:Balance,
    }

    impl Erc20 {
        #[ink(constructor)]
        fn mint(&mut self, initial_supply: Balance) {
            let caller = self.env().caller();
            self.total_supply.set(initial_supply);
            self.balances.insert(caller, initial_supply);
            self.env().emit_event(Transfer{
                from: None,
                to: Some(caller),
                value:initial_supply,
            });
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance{
            *self.total_supply
        }
        //查询方法
        #[ink(message)]
        fn balance_of(&self, owner:AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]
        fn transfer(&mut self, to:AccountId, value:Balance) -> bool {
            let from = self.env().caller();
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false
            }
            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(from, from_balance - value);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer{
                from: Some(from),
                to: Some(to),
                value,
            });
            true
        }
        fn balance_of_or_zero(&self, owner:&AccountId)-> Balance {
            *self.balances.get(owner).unwrap_or(&0)
        }

        #[ink(message)]
        fn approve(&mut self, user:AccountId, amount:Balance) -> bool {
            let approver = self.env().caller();
            let approver_balance = self.balance_of_or_zero(&approver);
            if approver_balance < amount {
                return false
            }
            let user_balance = self.balance_of_or_zero(&user);
            self.allowance.insert((approver, user), approver_balance - amount);
            self.allowance.insert((user, approver), user_balance + amount);
            self.env().emit_event(Approve{
                approver: Some(approver),
                user: Some(user),
                amount,
            });
            true

        }

        #[ink(message)]
        fn approval(&self, to:AccountId) -> Balance {
            let from = self.env().caller();
            *self.allowance.get(&(from, to)).unwrap_or(&0)
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[test]
        fn mint_works() {
            let mut erc20 = Erc20::mint(666);
            assert_eq!(erc20.total_supply(), 666);
        }

        #[test]
        fn approve_works() {
            let mut erc20 = Erc20::mint(100);
            assert!(erc20.transfer(AccountId::from([0x1; 32]), 10));
            assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 110);
            assert!(erc20.approve(AccountId::from([0x2; 32]), 50));
            assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 110);
            assert_eq!(erc20.approval(AccountId::from([0x2; 32])), 60);
            assert_eq!(erc20.approval(AccountId::from([0x0; 32])), 0);
            assert_eq!(erc20.approval(AccountId::from([0x1; 32])), 0);
        }
    }
}
