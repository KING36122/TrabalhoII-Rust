fn main() {
        
   /// Declaração da variavel 
   let a: u16 = 0x7FF;
      
   let result: u16; // Variavel que pode ser usada multiplas vezes
   
   // Exemplo 1: Operador Complementar "~" de 16 bits
   
   // ~0x7FF
   result = !a; // Complemento de a
   // dados de saidas com resultados
   println!("Representação em bits de 0x7FF   = {:016b}", a);
   println!("Complemento de 0x7FF             = 0x{:x}", result);
   println!("Representação em bits de 0x{:x}  = {:016b\n\n}",result, result);
}
