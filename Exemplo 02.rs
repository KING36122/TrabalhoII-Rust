fn main() {
        
   // Declaração das variaveis 
   let b: u32 = 0xC5;
   let c: u32 = 0x1111;
   let d: u32 = 0xFFFF;
   let e: u32 = 0x5B3C;
   
   let mut result1: u32; // Variavel que pode ser usada multiplas vezes
   
   // Operador Complementar "~" com 32 bits
   
   // ~0xC5 
   result1 = !b; // Complemento de b
   // dados de saidas com resultados
   println!("Representação em bits de 0xC5   = \t{:032b}", b);
   println!("Complemento de 0xC5             = \t0x{:x}", result1);
   println!("Representação em bits de 0x{:x} = \t{:032b}\n",result1, result1);
   
   // ~0x1111
   result1 = !c; // Complemento de c
   // dados de saidas com resultados
   println!("Representação em bits de 0x1111 = \t{:032b}", c);
   println!("Complemento de 0x1111           = \t0x{:x}", result1);
   println!("Representação em bits de 0x{:x} = \t{:032b}\n",result1, result1);
   
   // 0xFFFF
   result1 = !d; // Complemento de d
   // dados de saidas com resultados
   println!("Representação em bits de 0xFFFF = \t{:032b}", d);
   println!("Complemento de 0xFFFF           = \t0x{:x}", result1);
   println!("Representação em bits de 0x{:x} = \t{:032b}\n",result1, result1);
   
   // ~0x5B3C
   result1 = !e; // Complemento de e
   // dados de saidas com resultados
   println!("Representação em bits de 0x5B3C = \t{:032b}", e);
   println!("Complemento de 0x5B3C           = \t0x{:x}", result1);
   println!("Representação em bits de 0x{:x} = \t{:032b}\n",result1, result1);
}
