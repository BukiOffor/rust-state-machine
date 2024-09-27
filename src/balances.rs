use std::collections::BTreeMap;

pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Pallet {
		Self { balances: BTreeMap::new() }
	}
	pub fn set_balance(&mut self, who: &String, amount: u128) {
		self.balances.insert(who.clone(), amount);
	}
	pub fn balance(&self, who: &String) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}
    pub fn transfer(&mut self, from: &String, to: &String, amount: u128)-> Result<(), &'static str> {
        let from_balance = self.balances.get(from).ok_or("User does not exist")?;
        let new_from_balance = from_balance
            .checked_sub(amount)
            .ok_or("Not enough funds.")?;
        self.set_balance(from, new_from_balance);
        **self.balances.get_mut(to).get_or_insert(&mut 0_u128) += amount;
        Ok(())
    }
}


#[test]
fn init_balances() { 
    let mut balances = crate::balances::Pallet::new();
    assert_eq!(balances.balance(&"alice".to_string()), 0);
    balances.set_balance(&"alice".to_string(), 100);
    assert_eq!(balances.balance(&"alice".to_string()), 100);
    assert_eq!(balances.balance(&"bob".to_string()), 0);

}
