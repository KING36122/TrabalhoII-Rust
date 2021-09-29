fn main(){
	let b:u8 = 0b1010_1111;
	let masc1:u8 = 0b1111_1100;
	let masc2:u8 = 0b0000_0010;

	let result:u8;
	let result1:u8;

	result = b & masc1;
	println!("B                   => {:08b}",b);
	println!("Masc1               => {:08b}",masc1);
	println!("B & Masc1           => {:08b}",result);

	result1 = result | masc2;
	println!("Masc2               => {:08b}",masc2);
	println!("(B & Masc1) | Masc2 => {:08b}",result1);
}
