fn main() {
     let a:u16 = 0x6db7; // declaração da variavel de apoio
    let b:u16 = 0x6c00;  //declaração da variavel de apoio
    let m:u16 = 0xfc00;  //declaração da variavel de apoio


    let mut result:u16; // declaracao da variavel de resultado
    
    //resultado mostrando o A,a máscara 1 e o resultado da operação a & 0cfc00
    result= a;
    println!(" -a = {:016b} =  0x{:x} ",a, result);

    
    result = m;
    println!(" -m = {:016b} =  0x{:x} ",m, result);
    
    
    result = a & 0xfc00;
    println!(" -b = {:016b} =  0x{:x} ",b, result);
}
