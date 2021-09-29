fn main() {
    let a:u16 = 0x6db7;
    let b:u16 = 0x37;
    let m:u16 = 0x3F;


    let mut result:u16;
    
    result= a;
    println!(" -a = {:016b} =  0x{:x} ",a, result);

    
    result = m;
    println!(" -m = {:016b} =  0x{:x} ",m, result);
    
    
    result = a & 0x3F;
    println!(" -b = {:016b} =  0x{:x} ",b, result);

    
}
