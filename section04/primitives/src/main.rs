#[allow(unused_variables)]

fn main() {

    // as

    let float_type: f32=110.123321_f32;

    let integer_type: u8=float_type as u8;

    let char_type:char = integer_type as char;

    println!("float {float_type}\ninteger {integer_type}");


    println!("hexadecimal: {:x}\nchar: {}", integer_type,char_type);


    // arrays

    let chars_of_name: [char;6] = ['s','e','r','h','a','t'];

    println!("{}", chars_of_name[0]);

    let mut use_later: [i32;5]=[0;5];
    println!("{}",use_later.len());

    //tuples

    let tuple: (i32,f64,char, (i32,char))=(11,2.97,'z',(34,'y'));
    println!("{}",tuple.3.0);

}
