//Exemplo 5
// a Possui o valor 0x6DB7 e b Possui o valor 0xA726,este código mostra qual
//o valor de a and b que neste caso é 0x2526
fn main() {
    let a:u16 = 0x6DB7;
   let b:u16 = 0xA726;
   let result:u16;
   result=a & b;
   println!("a = {:16b} = 0x{:x} ",a,a);
   println!("b = {:16b} = 0x{:x} ",b,b);
   println!("a & b = {:16b} = 0x{:x} ",result,result);
}
