mod balances;

fn main() {
	let z = add(256,0);
	println!("{}",z);

}


fn add(x:u16, y:u8) -> u8 {
	(x as u8 ) + y as u8
}