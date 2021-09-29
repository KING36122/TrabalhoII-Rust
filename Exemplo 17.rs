fn main(){

	
	//declaração da variável
	let x:u8 = 1;

	//declaração da variável auxiliar
	let mut aux:u8;

	//print do resultado do deslocamento dos bits um por um
	println!("Resultado do deslocamento dos bits:");

	aux = x << 0;
	println!("{:08b}",aux);

	aux = x << 1;
	println!("{:08b}",aux);

	aux = x << 2;
	println!("{:08b}",aux);

	aux = x << 3;
	println!("{:08b}",aux);

	aux = x << 4;
	println!("{:08b}",aux);

	aux = x << 5;
	println!("{:08b}",aux);

	aux = x << 6;
	println!("{:08b}",aux);

	aux = x << 7;
	println!("{:08b}",aux);
}
