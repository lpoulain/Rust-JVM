use std::rc::Rc;
use std::collections::HashMap;

use crate::JVM;
use crate::jvm::JavaObject;
use crate::jvm::Classes;
use crate::java_class::JavaClass;

/////////////////// java.io.PrintStream

pub struct NativePrintStreamClass { }

impl JavaClass for NativePrintStreamClass {
    fn get_name(&self) -> String {
        return "java/io/PrintStream".to_string();
    }

    fn print(&self) {
        println!("Native Stream class");
    }

    fn execute_method(&self, jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        if !method_name.eq("println") {
            panic!("Native class {} does not have method {}", self.get_name(), method_name);
        }
        let string = jvm.pop();
        let _instance = jvm.pop();

        match &*string {
            JavaObject::STRING(value) => println!("{}", value),
            _ => println!("???")
        };
    }

    fn execute_static_method(&self, _jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        panic!("Native class {} does not have static method {}", self.get_name(), method_name);
    }

    fn get_static_object(&self, field_name: &String) -> JavaObject {
        panic!("Native class {} does not have static field {}", self.get_name(), field_name);
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

    fn execute_method(&self, _jvm: &mut JVM, _classes: &Classes, _method_name: &String) {
        println!("Not implemented yet");
    }

    fn execute_static_method(&self, _jvm: &mut JVM, _classes: &Classes, _method_name: &String) {
        panic!("Not implemented yet");
    }

    fn get_static_object(&self, field_name: &String) -> JavaObject {
        if field_name.eq("out") {
            return JavaObject::INSTANCE(self.get_name().clone(), HashMap::new());
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

    fn execute_method(&self, _jvm: &mut JVM, _classes: &Classes, _method_name: &String) {
        println!("Not implemented yet");
    }

    fn execute_static_method(&self, jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        if method_name.eq("parseInt") || method_name.eq("valueOf") {
            let int_value = match &*(jvm.pop()) {
                JavaObject::STRING(st) => Rc::new(JavaObject::INTEGER(st.parse::<i32>().unwrap())),
                JavaObject::INTEGER(int) => Rc::new(JavaObject::INTEGER(*int)),
                _ => panic!("Integer.parseInt() not supported for this type")
            };
            jvm.push(int_value);
            return;
        }

        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }

    fn get_static_object(&self, _field_name: &String) -> JavaObject {
        panic!("Not implemented yet");
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

    fn execute_method(&self, _jvm: &mut JVM, _classes: &Classes, _method_name: &String) {
        println!("Not implemented yet");
    }

    fn execute_static_method(&self, jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        if method_name.eq("format") {
            let array_arg = jvm.pop();
            let string_arg = jvm.pop();

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
                                        Some(object) => match **object {
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

            jvm.push(Rc::new(JavaObject::STRING(output)));
            return;
        }

        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }

    fn get_static_object(&self, field_name: &String) -> JavaObject {
        panic!("Native class {} does not have static object [{}]", self.get_name(), field_name);
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

    fn execute_method(&self, _jvm: &mut JVM, _classes: &Classes, _method_name: &String) {
        println!("Not implemented yet");
    }

    fn execute_static_method(&self, jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        if method_name.eq("asList") {
            let array_arg = jvm.pop();

            let _array = match &*array_arg {
                JavaObject::ARRAY(array) => array.borrow(),
                _ => panic!("String.format() expects an array in the stack")
            };

            jvm.push(Rc::new(JavaObject::INSTANCE(self.get_name().clone(), HashMap::new())));
            return;
        }

        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }

    fn get_static_object(&self, _field_name: &String) -> JavaObject {
        panic!("Not implemented yet");
    }
}
