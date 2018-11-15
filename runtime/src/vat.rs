
//use parity_codec::Encode;
use srml_support::{StorageValue, StorageMap, dispatch::Result};
//use runtime_primitives::traits::Hash;
use runtime_io::print;
use balances;
//use {system::{self, ensure_signed}};

pub trait Trait: balances::Trait {}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

    fn deliver(_origin, vat_id: u64) -> Result {
			//let sender = ensure_signed(origin)?;
      print("delivery to vat_id");
      print(vat_id);
      //print("key:"); print(<VatCounter<T>>::key());
      if <VatState<T>>::exists(vat_id) {
          <VatState<T>>::mutate(vat_id, |state| *state += 1);
          print(" incremented vat state to");
          print(<VatState<T>>::get(vat_id));
      } else {
          print("no such vat_id");
      }
			//let payment = Self::payment().ok_or("Must have payment amount set")?;
			//<balances::Module<T>>::decrease_free_balance(&sender, payment)?;
			//<balances::Module<T>>::increase_free_balance_creating(&sender, <Pot<T>>::take());
      Ok(())
    }

		fn create_vat(_origin) -> Result {
			<VatCounter<T>>::mutate(|c| *c += 1);
      let vat_id = Self::vatCounter();
			print("vatCounter is");
			print(vat_id);
      <VatState<T>>::insert(vat_id, 0);
      // format! is not available in nostd
      //print(format!("vatCounter is {}", Self::vatCounter()));
			Ok(())
    }
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Demo {
		VatCounter get(vatCounter) config(): u64;
		VatState get(vatState): map u64 => u64;
		Unused get(unused) config(): T::Balance;
	}
}
