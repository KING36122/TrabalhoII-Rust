//Exemplo 4
// b Possui o valor 0xA726 e este código mostra qual
//o valor de ~b que neste caso é 0x58D9
fn main() { 
   let b:u16 = 0xA726;
   let result:u16;
   result=!b;
   println!("b  = {:016b} = 0x{:x} ",b,b);
   println!("~b = {:016b} = 0x{:x} ",result,result);
}
