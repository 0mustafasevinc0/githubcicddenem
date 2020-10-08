use codec::{Decode, Encode};
//118nci commit lock'un çalışan ilk commitidir.
/// A runtime module tamplate with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs
// use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult};
use frame_support::{
	decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
	traits::{Currency, ExistenceRequirement, ReservableCurrency, LockIdentifier, LockableCurrency, WithdrawReason, WithdrawReasons},
};
use sp_std::{prelude::*, vec::Vec};
// use system::ensure_signed;
use frame_system::{self as system, ensure_signed};

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Foo {
    id: u32,
    data: Vec<u8>,
}

use pallet_timestamp;
// use pallet_balances;

// pub trait Trait: pallet_timestamp::Trait {
// 	/// The identifier type for an authority.
// 	type AuthorityId: Member + Parameter + RuntimeAppPublic + Default;
// }

// struct WeightForSetDummy<T: pallet_balances::Trait>(BalanceOf<T>);

// impl<T: pallet_balances::Trait> WeighData<(&BalanceOf<T>,)> for WeightForSetDummy<T>
// {
// 	fn weigh_data(&self, target: (&BalanceOf<T>,)) -> Weight {
// 		let multiplier = self.0;
// 		(*target.0 * multiplier).saturated_into::<Weight>()
// 	}
// }

// impl<T: pallet_balances::Trait> ClassifyDispatch<(&BalanceOf<T>,)> for WeightForSetDummy<T> {
// 	fn classify_dispatch(&self, target: (&BalanceOf<T>,)) -> DispatchClass {
// 		if *target.0 > <BalanceOf<T>>::from(1000u32) {
// 			DispatchClass::Operational
// 		} else {
// 			DispatchClass::Normal
// 		}
// 	}
// }

// impl<T: pallet_balances::Trait> PaysFee<(&BalanceOf<T>,)> for WeightForSetDummy<T> {
// 	fn pays_fee(&self, _target: (&BalanceOf<T>,)) -> Pays {
// 		Pays::Yes
// 	}
// }

// /// A type alias for the balance type from this pallet's point of view.
type BalanceOf<T> = <<T as Trait>::Currencya as Currency<<T as system::Trait>::AccountId>>::Balance;
// type BalanceOf<T> = <T as pallet_balances::Trait>::Balance;
const EXAMPLE_ID: LockIdentifier = *b"example ";

/// The module's configuration trait.
pub trait Trait: system::Trait + pallet_timestamp::Trait {
    // TODO: Add other types and constants required configure this module.
	type Currencya: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	// type AuthorityId: Member + Parameter + RuntimeAppPublic + Default;

}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as TamplateModule {
        // Just a dummy storage item.
        // Here we are declaring a StorageValue, `Something` as a Option<u32>
        // `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
        Something get(fn something): Option<u32>;
        FooStore get(fn foo): Option<Foo>;

        Value get(fn get_value): bool;
        Product get(fn get_product): map hasher(blake2_128_concat) u32 => u32;
        Owner get(fn get_owner): map hasher(blake2_128_concat) u32 => T::AccountId;
        StartingPrice get(fn get_byte_struct_starting_price): map hasher(blake2_128_concat) u32 => BalanceOf<T>;
        ProductId get(fn get_byte_struct_product_id): u32 = 1;
        CreatedTime get(fn get_byte_struct_created_time): map hasher(blake2_128_concat) u32 => <T as pallet_timestamp::Trait>::Moment;
        Ended get(fn get_ended): map hasher(blake2_128_concat) u32 => bool;
        HighestPrice get(fn get_byte_struct_highest_price): map hasher(blake2_128_concat) u32 => BalanceOf<T>;
        HighestAccount get(fn get_byte_struct_highest_account): map hasher(blake2_128_concat) u32 => T::AccountId;
        PendingReturns get(fn get_byte_struct_pending_returns): map hasher(blake2_128_concat) (u32,T::AccountId) => BalanceOf<T>;
        EndTime get(fn get_byte_struct_end_time): map hasher(blake2_128_concat) u32 => <T as pallet_timestamp::Trait>::Moment;
    }
}

// The module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event() = default;

        // Just a dummy entry point.
        // function that can be called by the external world as an extrinsics call
        // takes a parameter of the type `AccountId`, stores it and emits an event
        #[weight = 0]
        pub fn do_something(origin, something: u32) -> DispatchResult {
            // TODO: You only need this if you want to check it was signed.
            let who = ensure_signed(origin)?;

            // TODO: Code to execute when something calls this.
            // For example: the following line stores the passed in u32 in the storage
            Something::put(something);

            // here we are raising the Something event
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            Ok(())
        }

        /// Stores a custom struct in the runtime storage.
        /// Used for querying from smart contract.
        #[weight = 0]
        pub fn store_foo(origin, data: Vec<u8>, id: u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let foo = Foo {
                id,
                data,
              };

            FooStore::put(foo);
            Ok(())
        }

        #[weight = 0]
        pub fn add_product(origin, starting_price: BalanceOf<T>, millisecond: <T as pallet_timestamp::Trait>::Moment) -> DispatchResult {
            // TODO: You only need this if you want to check it was signed.
            let _who = ensure_signed(origin)?;

            let product_id=Self::get_byte_struct_product_id();
            let new_product_id = product_id.checked_add(1).expect("Entire supply fits in u64; qed");
            ProductId::put(&new_product_id);
            <Product>::insert(&new_product_id, &new_product_id);

            <Owner<T>>::insert(&new_product_id, &_who);
            <StartingPrice<T>>::insert(&new_product_id, &starting_price);
            
            let curr_time=<pallet_timestamp::Module<T>>::now();
            
            <CreatedTime<T>>::insert(&new_product_id, &curr_time);
            
            <HighestAccount<T>>::insert(&new_product_id, &_who);

            <PendingReturns<T>>::insert((&new_product_id,&_who), &starting_price);

            let end_time=curr_time+millisecond;
            <EndTime<T>>::insert(&new_product_id, &end_time);
            
            Self::deposit_event(RawEvent::Created(_who,starting_price,curr_time,end_time));
            Ok(())
        }

        #[weight = 0]
        pub fn is_ended(origin, id:u32) -> DispatchResult {
            // TODO: You only need this if you want to check it was signed.
            let _who = ensure_signed(origin)?;
            let end_time=<EndTime<T>>::get(&id);
            let curr_time=<pallet_timestamp::Module<T>>::now();

            let asd: i32=12;
            
            let last_time: <T as pallet_timestamp::Trait>::Moment=end_time-curr_time;
            if end_time>curr_time{
                Self::deposit_event(RawEvent::Is_ended(id,false,end_time,curr_time,last_time));
                Ok(())
            }else{
                Self::deposit_event(RawEvent::Is_ended(id,true,end_time,curr_time,last_time));
                Ok(())
            }
        }

        #[weight = 0]
        pub fn get_time_left(origin, id:u32) -> DispatchResult {
            // TODO: You only need this if you want to check it was signed.
            let _who = ensure_signed(origin)?;
            let end_time=<EndTime<T>>::get(&id);
            let curr_time=<pallet_timestamp::Module<T>>::now();

            let last_time=end_time-curr_time;
            if end_time>curr_time{
                Self::deposit_event(RawEvent::Is_ended(id,false,end_time,curr_time,last_time));
                Ok(())
            }else{
                Self::deposit_event(RawEvent::Is_ended(id,true,end_time,curr_time,last_time));
                Ok(())
            }
        }
        #[weight = 0]
        pub fn bid(origin, id:u32, amount:BalanceOf<T>) -> DispatchResult {
            // TODO: You only need this if you want to check it was signed.
            let _who = ensure_signed(origin)?;

            
            let _freebalance= T::Currencya::free_balance(&_who);
            if amount<_freebalance{
                let end_time=<EndTime<T>>::get(&id);
                let curr_time=<pallet_timestamp::Module<T>>::now();
    
                let starting_price=<StartingPrice<T>>::get(&id);
                let highest_price=<HighestPrice<T>>::get(&id);
    
                let sender_returs_money=<PendingReturns<T>>::get((&id,&_who));
                // const Lock_Id: LockIdentifier = *b"example";
                T::Currencya::set_lock(EXAMPLE_ID, &_who, amount, WithdrawReasons::except(WithdrawReason::Reserve));
                
                if end_time <= curr_time {
                    <PendingReturns<T>>::insert((&id,&_who), amount+sender_returs_money);
                    <Ended>::insert(&id, true);
                    let highest_bidder=<HighestAccount<T>>::get(&id);
                    Self::deposit_event(RawEvent::NoMoreBidding(true,highest_bidder,highest_price));
                    Ok(())
                    // return false 
                }
                else if amount <= starting_price {
                    // baslangıc fiyatının altında teklif girilmisse geri alabilir
                    <PendingReturns<T>>::insert((&id,&_who), amount+sender_returs_money);
                    Self::deposit_event(RawEvent::FailedBidLowerThanStartingPrice(_who,amount,starting_price));
                    Ok(())
                    //return false 
                }
                else if amount <= highest_price{
                    <PendingReturns<T>>::insert((&id,&_who), amount+sender_returs_money);
                    let highest_bidder=<HighestAccount<T>>::get(&id);
                    Self::deposit_event(RawEvent::FailedBidLowerThanHighestBid(_who,amount,highest_bidder,highest_price));
                    Ok(())
                    //return false 
                }else{
                    let previous_highest_bidder=<HighestAccount<T>>::get(&id);
                    let last_pending=<PendingReturns<T>>::get((&id,&previous_highest_bidder));
                    <PendingReturns<T>>::insert((&id,&previous_highest_bidder), last_pending+highest_price);
                    <HighestAccount<T>>::insert(&id, &_who);
                    <HighestPrice<T>>::insert(&id, &amount);
                    Self::deposit_event(RawEvent::NewHighestBid(previous_highest_bidder,highest_price,_who,amount));
                    Ok(())
                    // return true
                }
            }else{
                //error event emit edilcek
                Self::deposit_event(RawEvent::Insufficient_balance(_who,_freebalance,amount));
                Ok(())
            }
            
        }

        #[weight = 0]
        pub fn withdrawal(origin, id:u32, amount:BalanceOf<T>) -> DispatchResult {
            // TODO: You only need this if you want to check it was signed.
            let _who = ensure_signed(origin)?;
            let zero= T::Currencya::minimum_balance();
            if amount!=zero{
                <PendingReturns<T>>::insert((&id,&_who), zero);
                T::Currencya::remove_lock(EXAMPLE_ID, &_who);
                Self::deposit_event(RawEvent::Withdrawal(_who,amount));
            }
            Ok(())
        }
        
        #[weight = 0]
        pub fn end(origin, id:u32) -> DispatchResult {
            // TODO: You only need this if you want to check it was signed.
            let _who = ensure_signed(origin)?;

            let owner=<Owner<T>>::get(&id);
            let owner_curr_pending=<PendingReturns<T>>::get((&id,&owner));
            let highest_price=<HighestPrice<T>>::get(&id);
            let end_time=<EndTime<T>>::get(&id);
            let curr_time=<pallet_timestamp::Module<T>>::now();
            let new_owner=<HighestAccount<T>>::get(&id);

            if  (_who == new_owner || _who == owner) && end_time <= curr_time {
                <Ended>::insert(&id, true);
                <Owner<T>>::insert(&id, &new_owner);
                // <PendingReturns<T>>::insert((&id,&owner), owner_curr_pending+highest_price);
                T::Currencya::remove_lock(EXAMPLE_ID, &owner);
                T::Currencya::remove_lock(EXAMPLE_ID, &new_owner);
                T::Currencya::transfer(&new_owner,&owner,highest_price,ExistenceRequirement::KeepAlive);
                Self::deposit_event(RawEvent::Ended(new_owner,highest_price));
            }
            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Balance = BalanceOf<T>,
        Moment = <T as pallet_timestamp::Trait>::Moment
    {
    	// pub enum Event<T> where B = <T as pallet_balances::Trait>::Balance {

        // Just a dummy event.
        // Event `Something` is declared with a parameter of the type `u32` and `AccountId`
        // To emit this event, we call the deposit funtion, from our runtime funtions
        SomethingStored(u32, AccountId),

        // A smart contract was called from the runtime.
        ContractCalled(AccountId, bool),

        // A smart contract storage was queried.
        ContractQueried(AccountId, bool),



        Created(AccountId,Balance,Moment,Moment),
        NewHighestBid(AccountId,Balance,AccountId,Balance),
        FailedBidLowerThanStartingPrice(AccountId,Balance,Balance),
        FailedBidLowerThanHighestBid(AccountId,Balance,AccountId,Balance),
        NoMoreBidding(bool,AccountId,Balance),
        Withdrawal(AccountId,Balance),
        Ended(AccountId,Balance),
        Is_ended(u32,bool,Moment,Moment,Moment),
        Insufficient_balance(AccountId,Balance,Balance),
    }
);

/// tests for this module
#[cfg(test)]
mod tests {
    use super::*;

    use frame_support::{assert_ok, impl_outer_origin, parameter_types, weights::Weight};
    use sp_core::H256;
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
        Perbill,
    };

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaximumBlockWeight: Weight = 1024;
        pub const MaximumBlockLength: u32 = 2 * 1024;
        pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    }
    impl system::Trait for Test {
        type Origin = Origin;
        type Call = ();
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type BlockHashCount = BlockHashCount;
        type MaximumBlockWeight = MaximumBlockWeight;
        type MaximumBlockLength = MaximumBlockLength;
        type AvailableBlockRatio = AvailableBlockRatio;
        type Version = ();
        type ModuleToIndex = ();
    }
    impl Trait for Test {
        type Event = ();
    }
    type TamplateModule = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> sp_io::TestExternalities {
        system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap()
            .into()
    }

    #[test]
    fn it_works_for_default_value() {
        new_test_ext().execute_with(|| {
            // Just a dummy test for the dummy funtion `do_something`
            // calling the `do_something` function with a value 42
            assert_ok!(TamplateModule::do_something(Origin::signed(1), 42));
            // asserting that the stored value is equal to what we stored
            assert_eq!(TamplateModule::something(), Some(42));
        });
    }
}
