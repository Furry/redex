# Functions
Functons in Redex are simple in nature. They take in parameters, and they output a single element or tuple. Errors or the like are simply returned if thrown and propigated that way rather than having them as a seperate entity.

## Operators
 - `fn` Denotes a new function definition 

### Basic Function
```rs
fn myFunction() {
    print("Hi!");
}

fn myParameterFunction(param1, param2, param3) {
    print("I recieved:", param1, param2, param3);
}
```

### An exception example
```rs
fn toNumber(param1) {
    if ~Number.isNumeric(param1) {
        return (null, "Input is not a valid number.");
    } else {
        return Number.parse(param1);
    }
}

let val, err = toNumber("23");
if err {
    print("Your int value is:", val);
} else {
    print("The provided number was ~not~ a number.");
}
```