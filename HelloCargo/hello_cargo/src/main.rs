fn main() {
    let chocolate1 = 10;
    let chocolate2 = 10;
    let total: u32 = chocolate1 + chocolate2; //Const. can't be used because we are dealing with non-const. values
    println!("The sum of x and y is: {}", total);
}
