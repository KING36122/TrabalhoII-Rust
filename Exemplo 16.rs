fn main(){

	
	//declaração de variáveis de apoio
	let b:u8 = 0b1010_1111;
	let masc1:u8 = 0b1110_0011;
	let masc2:u8 = 0b0001_0100;

	//declaração de variáveis de resultado
	let result:u8;
	let result1:u8;

	//primeira parte mostrando o B e a máscara 1 e o resultado da operação B & Masc1
	result = b & masc1;
	println!("B                   => {:08b}",b);
	println!("Masc1               => {:08b}",masc1);
	println!("B & Masc1           => {:08b}",result);

	//segunda parte mostrando a máscara 2 e o resultado da operação (B & Masc1) | Masc2
	result1 = result | masc2;
	println!("Masc2               => {:08b}",masc2);
	println!("(B & Masc1) | Masc2 => {:08b}",result1);
}
