//Exemplo 3
// a Possui o valor 0x6DB7 e este código mostra qual
//o valor de ~a que neste caso é 0x9248 
fn main() { 
   let a:u16 = 0x6DB7;
   let result:u16;
   
   result=!a;
   println!("a = {:016b} = 0x{:x} ",a,a);
   println!("~a = {:016b} = 0x{:x} ",result,result);
}
