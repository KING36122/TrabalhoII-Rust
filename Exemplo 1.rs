fn main() {
        
   /// Declaração das variaveis 
   let a: u16 = 0x7FF;
   let b: u32 = 0xC5;
   let c: u32 = 0x1111;
   let d: u32 = 0xFFFF;
   let e: u32 = 0x5B3C;
   
   let result: u16;
   let mut result1: u32; // Variavel que pode ser usada multiplas vezes
   
   // Exemplo 1: Operador Complementar "~" de 16 bits
   
   // ~0x7FF
   result = !a; // Complemento de a
   // dados de saidas com resultados
   println!("Representação em bits de 0x7FF   = {:016b}", a);
   println!("Complemento de 0x7FF             = 0x{:x}", result);
   println!("Representação em bits de 0x{:x}  = {:016b\n\n}",result, result);
   
   
   // Operador Complementar "~" com 32 bits
   
   // ~0xC5 
   result1 = !b; // Complemento de b
   // dados de saidas com resultados
   println!("Representação em bits de 0xC5   = {:032b}", b);
   println!("Complemento de 0xC5             = 0x{:x}", result1);
   println!("Representação em bits de 0x{:x} = {:032b\n\n}",result1, result1);
   
   // ~0x1111
   result1 = !c; // Complemento de c
   // dados de saidas com resultados
   println!("Representação em bits de 0x1111 = {:032b}", c);
   println!("Complemento de 0x1111           = 0x{:x}", result1);
   println!("Representação em bits de 0x{:x} = {:032b\n\n}",result1, result1);
   
   // 0xFFFF
   result1 = !d; // Complemento de d
   // dados de saidas com resultados
   println!("Representação em bits de 0xFFFF = {:032b}", d);
   println!("Complemento de 0xFFFF           = 0x{:x}", result1);
   println!("Representação em bits de 0x{:x} = {:032b\n\n}",result1, result1);
   
   // ~0x5B3C
   result1 = !e; // Complemento de e
   // dados de saidas com resultados
   println!("Representação em bits de 0x5B3C = {:032b}", e);
   println!("Complemento de 0x5B3C           = 0x{:x}", result1);
   println!("Representação em bits de 0x{:x} = {:032b\n\n}",result1, result1);
}
