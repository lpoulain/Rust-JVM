use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use std::cell::Ref;

use crate::JVM;
use crate::jvm::JavaObject;
use crate::jvm::Classes;
use crate::java_class::JavaClass;
use crate::streams::NativeStreamClass;
use crate::streams::NativeLambdaMetafactoryClass;
use crate::streams::NativeStreamInstance;

pub fn register_native_classes(classes: &mut Classes) {
    classes.add_class(Rc::new(NativePrintStreamClass {}));
    classes.add_class(Rc::new(NativeSystemClass {}));
    classes.add_class(Rc::new(NativeStringClass {}));
    classes.add_class(Rc::new(NativeIntegerClass {}));
    classes.add_class(Rc::new(NativeArraysClass {}));
    classes.add_class(Rc::new(NativeListClass { content: Rc::new(Vec::new()) }));
    classes.add_class(Rc::new(NativeStreamClass { }));
    classes.add_class(Rc::new(NativeMathClass { }));
    classes.add_class(Rc::new(NativeLambdaMetafactoryClass {}));
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

    fn execute_method(&self, jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        match &method_name[..] {
            "println" => {
                let string = jvm.pop();
                let _instance = jvm.pop();

                match &*string {
                    JavaObject::STRING(value) => println!("{}", value),
                    _ => println!("???")
                };
            },
            "print" => {
                let string = jvm.pop();
                let _instance = jvm.pop();

                match &*string {
                    JavaObject::STRING(value) => print!("{}", value),
                    _ => print!("???")
                };
            },
            _ => panic!("Native class {} does not have method {}", self.get_name(), method_name)
        }
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

    fn execute_method(&self, jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        match &method_name[..] {
            "startsWith" => {
                let arg = jvm.pop();
                let this = jvm.pop();

                let comparison = match &*arg {
                    JavaObject::STRING(str) => str,
                    _ => panic!("String.startsWith() requires a string as a parameter")
                };

                let the_string = match &*this {
                    JavaObject::STRING(str) => str,
                    _ => panic!("String.startWith() requires 'this' to be a string")
                };

                jvm.push(Rc::new(JavaObject::BOOLEAN(the_string.starts_with(comparison))));
            },
            "toLowerCase" => {
                let this = jvm.pop();
                let the_string = match &*this {
                    JavaObject::STRING(str) => str,
                    _ => panic!("String.toLowerCase() requires 'this' to be a string")
                };
                jvm.push(Rc::new(JavaObject::STRING(the_string.to_lowercase())));
            }
            _ => panic!("String.{}() not implemented yet", method_name)
        };
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

            let array = match &*array_arg {
                JavaObject::ARRAY(array) => array.borrow(),
                _ => panic!("Arrays.asList() expects an array in the stack")
            };

            let mut list: Vec<Rc<JavaObject>> = Vec::new();
            for elt in array.iter() {
                list.push(elt.clone());
            }

            jvm.push(Rc::new(JavaObject::InstanceList(RefCell::new(NativeListClass { content: Rc::new(list) }))));

//            let mut fields: HashMap<String, Rc<JavaObject>> = HashMap::new();
//            fields.insert(String::from("content"), Rc::new(JavaObject::ARRAY(RefCell::new(list))));

//            jvm.push(Rc::new(JavaObject::INSTANCE(String::from("java/util/List"), fields)));
            return;
        }

        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }

    fn get_static_object(&self, _field_name: &String) -> JavaObject {
        panic!("Not implemented yet");
    }
}

/////////////////// java.util.List

pub struct NativeListClass {
    content: Rc<Vec<Rc<JavaObject>>>
}

impl JavaClass for NativeListClass {
    fn get_name(&self) -> String {
        return "java/util/List".to_string();
    }

    fn print(&self) {
        println!("Native List class");
    }

    fn execute_method(&self, jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        if method_name.eq("stream") {
            let list = jvm.pop();

            let object: Ref<NativeListClass>;

            match &*list {
                JavaObject::InstanceList(obj) => object = (&obj).borrow(),
                _ => panic!("List.stream() expects a List in the stack")
            };

            jvm.push(Rc::new(JavaObject::InstanceStream(RefCell::new(NativeStreamInstance::new(object.content.clone())))));

            return;
        }

        panic!("Native class {} does not have method [{}]", self.get_name(), method_name);
    }

    fn execute_static_method(&self, _jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }

    fn get_static_object(&self, _field_name: &String) -> JavaObject {
        panic!("Not implemented yet");
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

    fn execute_method(&self, _jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        panic!("Native class {} does not have method [{}]", self.get_name(), method_name);
    }

    fn execute_static_method(&self, jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        match &method_name[..] {
            "sqrt" => {
                let nb = jvm.pop_double();
                jvm.push(Rc::new(JavaObject::DOUBLE(nb.sqrt())));
            },
            "log" => {
                let nb = jvm.pop_double();
                jvm.push(Rc::new(JavaObject::DOUBLE(nb.ln())));
            }
            _ => panic!("Native class {} does not have static method [{}]", self.get_name(), method_name)
        };
    }

    fn get_static_object(&self, _field_name: &String) -> JavaObject {
        panic!("Not implemented yet");
    }
}
