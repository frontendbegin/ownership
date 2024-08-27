fn main() {
    //    let mut str = String::from("Hello");

    //     str.push_str(", World");

    //     println!("{str}");
    //     // in this when s2 is equates to s1 s1 is no longer valid
    //    let s1 = String::from("Hello");
    //    let s2 = s1;
    // //    println!("{s1}") gives error
    // //    exceept we call clone method
    //    let s3 =  String::from("Hello");
    //    let s4 = s3.clone();
    //    println!("s3 = {s3}, s4 = {s4}");

    //     let s1 = gives_ownership();
    //     println!("{s1}");
    //     let s2 = String::from("Hello");
    //     println!("{s2}");
    //     let s3 = takes_ownership_and_gives_ownership_back(s2);
    //     println!("{s3}");

    //     let s1 = String::from("Hello");

    //     let (s2, len) = calculate_length(s1);
    //     println!("The size of {} is equal to {}", s2, len);
    
        let s1 = String::from("Hello");
        let _s2 = ref_calculate_length(&s1);

        let s = String::from("hello world");
       let fw =  first_word_slice(&s);
       let sw =  second_word_slice(&s);
        println!("This is the first word {fw}");
        println!("This is the second word {sw}");
    
    // let s3 = dangle();

}
fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}
fn takes_ownership_and_gives_ownership_back(a_string: String) -> String {
    a_string
}
fn calculate_length(a_string: String) -> (String, usize) {
    let length = a_string.len();
    (a_string, length)
}
fn ref_calculate_length(a_string: &String) -> usize {
    let length = a_string.len();
    length
}
// fn dangle () -> String{
//     let dangler = String::from("Hello");
//     &dangler
// }
fn first_word(s:String) -> usize{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}
fn first_word_slice(s:&str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
    
}
fn second_word_slice(s:&str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[i..];
        }
    }
    &s[..]
    
}