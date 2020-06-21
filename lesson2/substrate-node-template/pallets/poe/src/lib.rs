#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod mock;


#[cfg(test)]
mod tests;

//钱包的配置trait
pub trait Trait: system::Trait {
    // 总体事件类型
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as MyModule {
        Something get(fn something): Option<u32>;
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        SomethingStored(u32, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        NoneValue,
        StorageOverflow,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn depoist_event() = default;

        #[weight = 10_000]
        pub fn do_something(origin, something u32) -> dispatch::DispatchResult {
            let who = ensure_signed(origin);
            Something::put(something);
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            Ok(())
        }

        #[weight = 10_000]
        pub fn cause_error(origin) -> dispatch::DispatchResult {

            let _who = ensure_signed(origin)?;
            match Something::get() {
                None => Err(Error::<T>::NoneValue)?,
                Some(old) => {
                    let new  = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    Something::put(new);
                    Ok(())
                },
            }
        }
    }
}