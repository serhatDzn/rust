#[allow(unused_variables)]

fn main() {
    let mut age:u8 = 30;

    const PI_NUMBER:f64  = 3.14159;

    println!("{:e} is my age!!!\npi number is {}",age,PI_NUMBER);

    // Tek satırda yorum satırı çift slash ile yapılır


    /*
    Çoklu yorum satırı
    Böyle yapılır
     */

    let id:u32=1;
    let _age_of_serhat:u8=30;

    let n1:i8 = 19;
    let n2:i8 = 21;
    let mut n3:i8 = n1+n2;
    n3 += n2;
    println!("number 3 = {}",n3);

    let s1:u8 = 33;
    let s2:u8 = 11;
    println!("{} ile {} nin toplamı {}'dir.",s1,s2,s1+s2);

    let myage:u8 = 21;
    println!("Benim yaşım {myage} dir.");


    // Boolean

    let is_rust_fun: bool = true;

    // Float
    let x:f32 = 2.89889; // f32
    let y = 5.14.421321321312312321 // f64

    //chars
    let c:char = 's'; //char
    let c1 = 'd'; //char

    

}
