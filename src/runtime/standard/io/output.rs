use crate::runtime::{Callable, primitives::{Scope, VariableStorage, Variable, traits::StdConversions}};

pub struct Print;
impl Print {
    pub fn call(parent: &Scope, args: Vec<VariableStorage>) -> Option<VariableStorage> {

        // If Args is greater than 1, error.
        if args.len() != 1 {
            panic!("print() takes 1 argument, {} given", args.len());
        }

        print!("{}", args[0].to_compound_string().store);

        None
    }
}

// impl Callable for Print {
//     fn call(&parent: Scope, args: Vec<VariableStorage>) -> Option<VariableStorage> {

//         // If Args is greater than 1, error.
//         if args.len() != 1 {
//             panic!("print() takes 1 argument, {} given", args.len());
//         }

//         print!("{}", args[0].to_compound_string().store);

//         None
//     }

//     fn name(&self) -> String {
//         "print".to_string()
//     }
// }

pub struct PrintLine;
impl Callable for PrintLine {
    fn call(&self, parent: Scope, args: Vec<VariableStorage>) -> Option<VariableStorage> {

        // If Args is greater than 1, error.
        if args.len() != 1 {
            panic!("print() takes 1 argument, {} given", args.len());
        }

        println!("{}", args[0].to_compound_string().store);

        None
    }

    fn name(&self) -> String {
        "printline".to_string()
    }
}