fn main(){

	//declaração da variável
	let a:u16 = 0b0110_1101_1011_0111;
	
	//declaração da variável auxiliar
	let mut b:u16;

	//print do resultado do deslocamento dos bits um por um
	println!("Resultado do deslocamento dos bits:");

	b = a >> 0;
	println!("{:016b}",b);

	b = a >> 1;
	println!("\n{:016b}",b);

	b = a >> 2;
	println!("{:016b}",b);

	b = a >> 3;
	println!("{:016b}",b);

	b = a >> 4;
	println!("{:016b}",b);

	b = a >> 5;
	println!("{:016b}",b);

	b = a >> 6;
	println!("{:016b}",b);
	
	let result: u16 = 0x1B6;
	println!("\nb = 0x{:x}",result);

}
