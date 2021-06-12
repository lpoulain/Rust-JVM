use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;

use crate::StackFrame;
use crate::jvm::JavaObject;
use crate::jvm::Classes;
use crate::java_class::JavaClass;
use crate::streams::NativeStreamClass;
use crate::streams::NativeLambdaMetafactoryClass;
use crate::streams::NativeStreamInstance;

pub fn register_native_classes(classes: &mut Classes) {
    classes.add_class(Rc::new(NativeObjectClass {}));
    classes.add_class(Rc::new(NativePrintStreamClass {}));
    classes.add_class(Rc::new(NativeSystemClass {}));
    classes.add_class(Rc::new(NativeStringClass {}));
    classes.add_class(Rc::new(NativeIntegerClass {}));
    classes.add_class(Rc::new(NativeArraysClass {}));
    classes.add_class(Rc::new(NativeListClass {}));
    classes.add_class(Rc::new(NativeArrayListClass {}));
    classes.add_class(Rc::new(NativeStreamClass {}));
    classes.add_class(Rc::new(NativeMathClass {}));
    classes.add_class(Rc::new(NativeLambdaMetafactoryClass {}));
    classes.add_class(Rc::new(NativeEnumClass {}));
}

//////////

pub struct NativeObjectClass { }

impl JavaClass for NativeObjectClass {
    fn get_name(&self) -> String {
        return "java/lang/Object".to_string();
    }

    fn print(&self) {
        println!("Native Object class");
    }

    fn execute_method(&self, sf: &mut StackFrame, _classes: &Classes, method_name: &String, _nb_args: usize) {
        match &method_name[..] {
            "<init>" => {
                sf.pop();
            },
            _ => panic!("Class {} does not support method {}", self.get_name(), method_name)
        };
    }
}

/////////////////// java.io.PrintStream

pub struct NativePrintStreamClass { }

impl JavaClass for NativePrintStreamClass {
    fn get_name(&self) -> String {
        return "java/io/PrintStream".to_string();
    }

    fn print(&self) {
        println!("Native Stream class");
    }

    fn execute_method(&self, sf: &mut StackFrame, _classes: &Classes, method_name: &String, _nb_args: usize) {
        match &method_name[..] {
            "println" => {
                let string = sf.pop();
                sf.pop();

                match &*string {
                    JavaObject::STRING(value) => println!("{}", value),
                    _ => println!("???")
                };
            },
            "print" => {
                let string = sf.pop();
                sf.pop();

                match &*string {
                    JavaObject::STRING(value) => print!("{}", value),
                    _ => print!("???")
                };
            },
            _ => panic!("Native class {} does not have method {}", self.get_name(), method_name)
        }
    }
}

/////////////////// java.lang.System

pub struct NativeSystemClass { }

impl JavaClass for NativeSystemClass {
    fn get_name(&self) -> String {
        return "java/lang/System".to_string();
    }

    fn print(&self) {
        println!("Native System class");
    }

    fn get_static_object(&self, field_name: &String) -> Rc<JavaObject> {
        if field_name.eq("out") {
            return Rc::new(JavaObject::INSTANCE(self.get_name().clone(), RefCell::new(HashMap::new())));
        }
            
        panic!("Native class {} does not have static field [{}]", self.get_name(), field_name);
    }

}

/////////////////// java.lang.Integer
 
pub struct NativeIntegerClass { }

impl JavaClass for NativeIntegerClass {
    fn get_name(&self) -> String {
        return "java/lang/Integer".to_string();
    }

    fn print(&self) {
        println!("Native Integer class");
    }

    fn execute_static_method(&self, sf: &mut StackFrame, _classes: &Classes, method_name: &String, _nb_args: usize) {
        if method_name.eq("parseInt") || method_name.eq("valueOf") {
            let int_value = match &*(sf.pop()) {
                JavaObject::STRING(st) => Rc::new(JavaObject::INTEGER(st.parse::<i32>().unwrap())),
                JavaObject::INTEGER(int) => Rc::new(JavaObject::INTEGER(*int)),
                _ => panic!("Integer.parseInt() not supported for this type")
            };
            sf.push(int_value);
            return;
        }

        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }
}

/////////////////// java.lang.String

pub struct NativeStringClass { }

impl JavaClass for NativeStringClass {
    fn get_name(&self) -> String {
        return "java/lang/String".to_string();
    }

    fn print(&self) {
        println!("Native Integer class");
    }

    fn execute_method(&self, sf: &mut StackFrame, _classes: &Classes, method_name: &String, _nb_args: usize) {
        match &method_name[..] {
            "startsWith" => {
                let arg = sf.pop();
                let this = sf.pop();

                let comparison = match &*arg {
                    JavaObject::STRING(str) => str,
                    _ => panic!("String.startsWith() requires a string as a parameter")
                };

                let the_string = match &*this {
                    JavaObject::STRING(str) => str,
                    _ => panic!("String.startWith() requires 'this' to be a string")
                };
                sf.push(Rc::new(JavaObject::BOOLEAN(the_string.starts_with(comparison))));
            },
            "toLowerCase" => {
                let this = sf.pop();
                let the_string = match &*this {
                    JavaObject::STRING(str) => str,
                    _ => panic!("String.toLowerCase() requires 'this' to be a string")
                };
                sf.push(Rc::new(JavaObject::STRING(the_string.to_lowercase())));
            },
            "hashCode" => {
                let this = sf.pop();
                let the_string = match &*this {
                    JavaObject::STRING(str) => str,
                    _ => panic!("String.toLowerCase() requires 'this' to be a string")
                };
                let mut n = the_string.len() as u32;
    
                let mut hash: i32 = 0;
    
                let str = the_string.as_bytes();
                let thirty_one: i32 = 31;
    
                for c in str {
                    n -= 1;
                    hash += (*c as i32) * i32::pow(thirty_one, n);
                }
                sf.push(Rc::new(JavaObject::INTEGER(hash)));
            },
            "equals" => {
                let arg = sf.pop();
                let this = sf.pop();

                let string1 = match &*this {
                    JavaObject::STRING(str) => str,
                    _ => panic!("String.toLowerCase() requires 'this' to be a string")
                };

                let string2 = match &*arg {
                    JavaObject::STRING(str) => str,
                    _ => panic!("String.toLowerCase() requires 'this' to be a string")
                };
                sf.push(Rc::new(JavaObject::BOOLEAN(string1.eq(string2))));
            }
            _ => panic!("String.{}() not implemented yet", method_name)
        };
    }

    fn execute_static_method(&self, sf: &mut StackFrame, _classes: &Classes, method_name: &String, _nb_args: usize) {
        if method_name.eq("format") {
            let array_arg = sf.pop();
            let string_arg = sf.pop();

            let array = match &*array_arg {
                JavaObject::ARRAY(array) => array.borrow(),
                _ => panic!("String.format() expects an array in the stack")
            };

            let mut output = String::new();
            let mut special = false;
            let mut array_idx: usize = 0;

            match &*string_arg {
                JavaObject::STRING(value) => {
                    for c in value.chars() {
                        if special {
                            match c {
                                'd' => {
                                    match array.get(array_idx) {
                                        Some(object) => match &**object {
                                            JavaObject::INTEGER(int) => {
                                                output.push_str(&int.to_string());
                                            },
                                            _ => panic!("String.format() expects an integer for argument {}", array_idx + 1)
                                        },
                                        _ => panic!("String.format() does not have enought arguments")
                                    };
                                },
                                's' => {
                                    match array.get(array_idx) {
                                        Some(object) => match &**object {
                                            JavaObject::STRING(str) => {
                                                output.push_str(&str.clone());
                                            },
                                            _ => panic!("String.format() expects a string for argument {}", array_idx + 1)
                                        },
                                        _ => panic!("String.format() does not have enought arguments")
                                    };
                                },
                                _ => panic!("String.format() does not support %{}", c)
                            };
                            special = false;
                            array_idx += 1;
                        } else {
                            match c {
                                '%' => { special = true; },
                                _ => { output.push(c); }
                            };
                        }
                    }
        
                },
                _ => panic!("String.format() expects a string as parameter")
            };

            sf.push(Rc::new(JavaObject::STRING(output)));
            return;
        }

        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }
}

/////////////////// java.util.Arrays

pub struct NativeArraysClass { }

impl JavaClass for NativeArraysClass {
    fn get_name(&self) -> String {
        return "java/util/Arrays".to_string();
    }

    fn print(&self) {
        println!("Native Arrays class");
    }

    fn execute_static_method(&self, sf: &mut StackFrame, _classes: &Classes, method_name: &String, _nb_args: usize) {
        if method_name.eq("asList") {
            let array_arg = sf.pop();

            let array = match &*array_arg {
                JavaObject::ARRAY(array) => array.borrow(),
                _ => panic!("Arrays.asList() expects an array in the stack")
            };

            let mut list: Vec<Rc<JavaObject>> = Vec::new();
            for elt in array.iter() {
                list.push(elt.clone());
            }

            sf.push(Rc::new(JavaObject::InstanceList(RefCell::new(NativeArrayListInstance { content: RefCell::new(list) }))));
            return;
        }

        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }
}

/////////////////// java.util.ArrayList

pub struct NativeArrayListInstance {
    content: RefCell<Vec<Rc<JavaObject>>>
}

pub struct NativeArrayListClass { }

impl JavaClass for NativeArrayListClass {

    fn new(&self) -> JavaObject {
        JavaObject::InstanceList(RefCell::new(NativeArrayListInstance { content: RefCell::new(Vec::new()) }))
    }

    fn get_name(&self) -> String {
        return "java/util/ArrayList".to_string();
    }

    fn print(&self) {
        println!("Native ArrayList class");
    }

    fn execute_method(&self, sf: &mut StackFrame, _classes: &Classes, method_name: &String, _nb_args: usize) {
        if method_name.eq("<init>") {
            sf.pop();
            return;
        }
        panic!("Native class {} does not have method [{}]", self.get_name(), method_name);
    }
}

/////////////////// java.util.List

pub struct NativeListClass { }

impl JavaClass for NativeListClass {
    fn get_name(&self) -> String {
        return "java/util/List".to_string();
    }

    fn print(&self) {
        println!("Native List class");
    }

    fn execute_method(&self, sf: &mut StackFrame, _classes: &Classes, method_name: &String, _nb_args: usize) {
        match &method_name[..] {
            "stream" => {
                let list = sf.pop();

                let object: Vec<Rc<JavaObject>>;
                match &*list {
                    JavaObject::InstanceList(obj) => object = (&obj).borrow().content.borrow().clone(),
                    _ => panic!("List.stream() expects a List in the stack")
                };

                sf.push(Rc::new(JavaObject::InstanceStream(RefCell::new(NativeStreamInstance::new(Rc::new(object) )))));
            },
            "add" => {
                let value= sf.pop();
                let list = sf.pop();

                match &*list {
                    JavaObject::InstanceList(obj) => {
                        obj.borrow_mut().content.borrow_mut().push(value.clone());
                    },
                    _ => panic!("List.add() expects a List as its first argument")
                };

                sf.push(Rc::new(JavaObject::BOOLEAN(true)));
            }
            _ => panic!("Native class {} does not have method [{}]", self.get_name(), method_name)
        }
    }
}

/////////////////// java.lang.Math

struct NativeMathClass {}

impl JavaClass for NativeMathClass {
    fn get_name(&self) -> String {
        return "java/lang/Math".to_string();
    }

    fn print(&self) {
        println!("Native Math class");
    }

    fn execute_static_method(&self, sf: &mut StackFrame, _classes: &Classes, method_name: &String, _nb_args: usize) {
        match &method_name[..] {
            "sqrt" => {
                let arg = sf.pop();
                let nb = match &*arg {
                    JavaObject::DOUBLE(nb) => nb,
                    _ => panic!("Math.sqrt() expects a double as an argument")
                };
                sf.push(Rc::new(JavaObject::DOUBLE(nb.sqrt())));
            },
            "log" => {
                let arg = sf.pop();
                let nb = match &*arg {
                    JavaObject::DOUBLE(nb) => nb,
                    _ => panic!("Math.log() expects a double as an argument")
                };
                sf.push(Rc::new(JavaObject::DOUBLE(nb.ln())));
            }
            _ => panic!("Native class {} does not have static method [{}]", self.get_name(), method_name)
        };
    }
}

/////////////////// java.lang.Enum
struct NativeEnumClass {}

impl JavaClass for NativeEnumClass {
    fn get_name(&self) -> String {
        return "java/lang/Enum".to_string();
    }

    fn print(&self) {
        println!("Native Enum class");
    }

    fn execute_method(&self, sf: &mut StackFrame, _classes: &Classes, method_name: &String, _nb_args: usize) {
        match &method_name[..] {
            "ordinal" => {
                let arg = sf.pop();
                let nb = match &*arg {
                    JavaObject::DOUBLE(nb) => nb,
                    _ => panic!("Math.sqrt() expects a double as an argument")
                };
                sf.push(Rc::new(JavaObject::DOUBLE(nb.sqrt())));
            },
            _ => panic!("Native class {} does not have method [{}]", self.get_name(), method_name)
        };
    }
}
