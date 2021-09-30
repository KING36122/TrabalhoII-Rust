//Exemplo 6
// a Possui o valor 0x6DB7 e b Possui o valor 0xA726,este código mostra qual
//o valor de a XOR b que neste caso é 0xCA91
fn main() {
    let a:u16 = 0x6DB7;
   let b:u16 = 0xA726;
   let result:u16;
   result=a ^ b;
   println!("a     = {:016b} = 0x{:x} ",a,a);
   println!("b     = {:016b} = 0x{:x} ",b,b);
   println!("a ^ b = {:016b} = 0x{:x} ",result,result);
}
