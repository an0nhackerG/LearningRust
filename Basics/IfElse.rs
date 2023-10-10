fn main(){

    /*
        Let is used to declare variables, is important to note that rust treats 
        a variable declared with let as imutable. To declare a mutable variable you 
        must explicit specify it, for this we use "let mut".

        trying to modify a immutable variable result in a compiler error.

        rust use snake_case for variable names (good practices for rust)
    */
   
    // Declare a int immutable variable.
    let x = 10;
    let y = 20; 
    let z = 30;
    let target = 60;
    let result = x + y + z;

    let mut is_result_valid = false; // declare a boolean mutable variable

    // declare a string immutable variable
    let print_if_is_true = String::from("Result is true");
    let print_if_is_false = String::from("Result is false");

    let mut result_in_char = 'f'; // declare a char mutable variable, note that char must be into single quotes, 

    // verify if target is equal to result.
    if target == result {
        is_result_valid = true; // assign true to _is_result_valid. Note that this variable is declared as mutable
        result_in_char = 't'; // assign 't' to _result_in_char. Note that this variable is declared as mutable
        println!("{}", print_if_is_true); // print the value of print_if_is_true
    } else{
        println!("{}", print_if_is_false); // print the value of print_if_is_false
    }
}