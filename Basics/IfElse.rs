fn main(){
    let x = 10;
    let y = 20;
    let z = 30;

    let target = 60;
    let result = x + y + z;
    let mut is_result_valid = false;
    let print_if_is_true = String::from("Result is true");
    let print_if_is_false = String::from("Result is false");
    let mut result_in_char = 'f';

    if target == result {
        is_result_valid = true;
        result_in_char = 't';
        println!("{}", print_if_is_true);
    } else{
        println!("{}", print_if_is_false);
    }
}