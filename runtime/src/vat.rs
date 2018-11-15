
//use parity_codec::Encode;
use srml_support::{StorageValue, dispatch::Result};
//use runtime_primitives::traits::Hash;
use runtime_io::print;
use balances;
//use {system::{self, ensure_signed}};

pub trait Trait: balances::Trait {}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

    fn deliver(_origin, vatID: u64) -> Result {
			//let sender = ensure_signed(origin)?;
      print("delivery to vatID");
      print(vatID);
			//let payment = Self::payment().ok_or("Must have payment amount set")?;
			//<balances::Module<T>>::decrease_free_balance(&sender, payment)?;
			//<balances::Module<T>>::increase_free_balance_creating(&sender, <Pot<T>>::take());
      Ok(())
    }

		fn create_vat(_origin) -> Result {
			<VatCounter<T>>::mutate(|c| *c += 1);
			print("vatCounter is");
			print(Self::vatCounter());
      // format! is not available in nostd
      //print(format!("vatCounter is {}", Self::vatCounter()));
			Ok(())
    }
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Demo {
		VatCounter get(vatCounter) config(): u64;
		Unused get(unused) config(): T::Balance;
	}
}
