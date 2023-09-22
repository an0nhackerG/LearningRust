/*
    Here you will find a very simple example of loops
*/
fn main(){
    // Declare 1 int imutable variable and 2 int mutable variable
    let x = 30;
    let mut for_result = 0;
    let mut while_result = 0;

    /*
        this will make i (aka index) loop starting from 0 and stop in x+1 (aka 31), we do this because 
        the loop will gonna stop in x-1. 
     */
    for i in 0..x+1{
        for_result = i;
        // print the value of i and for_result for each iteration
        println!("Loop number {} for_result value: {}", i, for_result);
    }
    
    // the loop while is the same as the other languages
    while while_result < x{
        while_result += 1; // while_result = while_result + 1 
    }
    println!("for_result: {}", for_result);
    println!("while_result: {}", while_result);
}