#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod test {

	use ink_prelude::string::String;

	#[ink(event)]
	pub struct SampleEvent;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Test {
        /// Stores a single `int` value on the storage.
        num: i64,
		/// Stores a single `String` value on the storage.
		s: String,
    }

    impl Test {
        /// Constructor that initializes the `int` value to the given `init_num` and the `String` value to the given `init_s`.
        #[ink(constructor)]
        pub fn new(init_num: i64, init_s: String) -> Self {
            Self { num: init_num, s: init_s }
        }

        /// Constructor that initializes the `int` value to 0 and the `String` value to "".
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(0, String::from(""))
        }

        /// A message that can be called on instantiated contracts.

		/// This one increments the value of the stored `int` by 1.
        #[ink(message)]
        pub fn update_num(&mut self, new_num: i64) {
			self.num = new_num;
        }

        /// Simply returns the current value of our `int`.
        #[ink(message)]
        pub fn get_num(&self) -> i64 {
            self.num
        }

		/// Updates the value of our `String`.
		#[ink(message)]
		pub fn update_s(&mut self, new_s: String) {
			self.s = new_s;
		}

		/// Simply returns the current value of `String`.
		#[ink(message)]
		pub fn get_s(&self) -> String {
			self.s.clone()
		}

		#[ink(message)]
		pub fn fibonacci(&self, n: u32) -> u128 {
			if n < 2 {
				n.into()
			} else {
				self.fibonacci(n-1) + self.fibonacci(n-2)
			}
		}

		#[ink(message)]
		pub fn odd_product(&self, n: u32) -> u128 {
			(1..=n as u128).fold(1, |prod, x| prod * (2 * x - 1))
		}

		#[ink(message)]
		pub fn triangle_number(&self, n: u32) -> u128 {
			(1..=n as u128).fold(0, |sum, x| sum + x)
		}

		#[ink(message)]
		pub fn emit_sample_event(&self) {
			self.env().emit_event(SampleEvent{});
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
            let test = Test::default();
            assert_eq!(test.get_num(), 0);
			assert_eq!(test.get_s(), String::new());
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
			let num = 100;
			let init_s = String::from("Hi!");
			let new_s = String::from("Hello!");
            let mut test = Test::new(num, init_s);
            assert_eq!(test.get_num(), 100);
			assert_eq!(test.get_s(), init_s);
            test.increment_num();
			test.update_s(new_s);
            assert_eq!(test.get_num(), num+1);
			assert_eq!(test.get_s(), new_s);
        }
    }
}
