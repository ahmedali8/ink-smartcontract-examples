#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod incrementer {
    use ink::storage::Mapping;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Incrementer {
        /// Stores a single `i32` value on the storage.
        value: i32,
        /// Stores the mapping key from `AccountId` to `i32`.
        my_map: Mapping<AccountId, i32>,
    }

    impl Incrementer {
        /// Constructor that initializes the `i32` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            let mut my_map = Mapping::default();
            let caller = Self::env().caller();
            my_map.insert(&caller, &0);

            Self {
                value: init_value,
                my_map,
            }
        }

        /// Constructor that initializes the `i32` value to `0`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one increments the value of the stored `i32` by `i32`.
        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
        }

        /// A message that can be called on instantiated contracts.
        /// This one increments the value of the stored caller's `i32` by `i32`
        /// in `my_map`.
        #[ink(message)]
        pub fn inc_mine(&mut self, by: i32) {
            let caller: AccountId = self.env().caller();
            let my_value = self.get_mine();
            self.my_map.insert(caller, &(my_value + by));
        }

        /// A message that can be called on instantiated contracts.
        /// This one removes the value of the stored caller's `i32`.
        #[ink(message)]
        pub fn remove_mine(&self) {
            let caller: AccountId = self.env().caller();
            self.my_map.remove(&caller)
        }

        /// Simply returns the `my_map` value of the caller's `AccountId`.
        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            let caller: AccountId = self.env().caller();
            self.my_map.get(&caller).unwrap_or_default()
        }

        /// Simply returns the current value of our `i32`.
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let incrementer: Incrementer = Incrementer::default();
            assert_eq!(incrementer.get(), 0);
        }

        #[ink::test]
        fn my_map_works() {
            let incrementer: Incrementer = Incrementer::new(11);
            assert_eq!(incrementer.get(), 11);
            assert_eq!(incrementer.get_mine(), 0);
        }

        #[ink::test]
        fn inc_mine_works() {
            let mut contract: Incrementer = Incrementer::new(11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 10);
        }

        #[ink::test]
        fn remove_mine_works() {
            let mut contract: Incrementer = Incrementer::new(11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.remove_mine();
            assert_eq!(contract.get_mine(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut incrementer: Incrementer = Incrementer::new(42);
            assert_eq!(incrementer.get(), 42);
            incrementer.inc(5);
            assert_eq!(incrementer.get(), 47);
            incrementer.inc(-50);
            assert_eq!(incrementer.get(), -3);
        }
    }
}
