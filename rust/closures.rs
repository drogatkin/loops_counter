fn main() {
    let number = 1;
    
    let result = ["zero", "one", "two", "Default Case"].get(number)
     .unwrap_or(&"Default Case");
    println!{"result: {result}"}
    
    let zero = || println!{"zero"};
    let one = || println!{"one"};
    let two = || println!{"two"};
    let default = || {println!{"Default Case"}};
    
    [zero, one, two].get(number).unwrap_or_else(|| &((|| println!{"Default Case"}) as fn()))();
    [zero, one, two].get(number).copied().unwrap_or(default)();
    [|| println!{"zero"}, || println!{"one"}, || println!{"two"}].get(number).copied().unwrap_or(|| println!{"Default Case"})();
}