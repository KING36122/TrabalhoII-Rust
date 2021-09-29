fn main() {
    //declaração das variaveis de apoio
    let a:u16 = 0x6db7;
    let m:u16 = 0x4;


    //declaração da variavel de resultado
    let mut result:u16;
    
    //resultado mostrando o A,a máscara 1 e o resultado da operação a ^0x4
    result= a;
    println!(" -a = {:016b} =  0x{:x} ",a, result);

    
    result = m;
    println!(" -m = {:016b} =  0x{:x} ",m, result);
    
    
    result = a ^ 0x4;
    println!(" -b = {:016b} =  0x{:x} ",result, result);
    
    result = m;
    println!(" -m = {:016b} =  0x{:x} ",m, result);
    
    result =  (a^ 0x4)^0x4;
    println!(" -c = {:016b} =  0x{:x} ",result, result);
}
