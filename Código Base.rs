fn main() {
   let a:u16 = 0x6db7;
   let b:u16 = 0xa726;
   let c:u16 = 0x7ff;
   let d:u32 = 0xc5;
   let e:u32 = 0x1111;
   let x:u8  = 1;
   
   let mut x0:u8;
   let mut result:u16;
   let mut result1:u32;
   
   result = a & b;
   println!("(a & b) => 0x{:x} ",result);
   
   println!("{:16b}",a);

   result = a | b;
   println!("(a | b) => 0x{:x} ",result) ;

   result = a ^ b;
   println!("(a ^ b) => 0x{:x} ",result);

   result = !a;
   println!("(!a) => 0x{:x} ",result);
   
   result = !c;
   println!("(!c) => 0x{:x} ",result);
   
   result1 = !d;
   println!("(!d) => 0x{:x} ",result1);
   
   result1 = !e;
   println!("(!e) => 0x{:x} ",result1);
   
   result = a & 0x3f;
   println!("(a & 0x3f) => 0x{:x} ",result);
   
   x0 = x << 0;
   println!("(x >> 0) => {:08b} ",x0);
   
   x0 = x << 1;
   println!("(x >> 1) => {:08b} ",x0);
}