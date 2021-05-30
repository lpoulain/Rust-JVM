use std::rc::Rc;
use std::cell::RefCell;

use crate::jvm::JVM;
use crate::jvm::JavaObject;
use crate::jvm::Classes;
use crate::java_class::JavaClass;

/////////////////// java.util.stream.Stream

pub trait StreamFunction {
    fn next_object(&mut self, function_idx: usize, stream: &NativeStreamInstance, jvm: &mut JVM, classes: &Classes) -> Option<Rc<JavaObject>>;
    fn print(&self);
}

pub struct NativeStreamData {
    data: Rc<Vec<Rc<JavaObject>>>,
    idx: usize
}

impl NativeStreamData {
    pub fn new(data: Rc<Vec<Rc<JavaObject>>>) -> Box<dyn StreamFunction> {
        Box::new(NativeStreamData {
            data,
            idx: 0
        })
    }
}

impl StreamFunction for NativeStreamData {
    fn next_object(&mut self, _function_idx: usize, _stream: &NativeStreamInstance, _jvm: &mut JVM, _classes: &Classes) -> Option<Rc<JavaObject>> {
        let object = match self.data.get(self.idx) {
            Some(obj) => Some(obj.clone()),
            _ => None
        };
        self.idx += 1;
        return object;
    }
    fn print(&self) {
        println!("Data");
    }
}

pub struct NativeStreamInstance {
    pub operations: Vec<RefCell<Box<dyn StreamFunction>>>
}

impl NativeStreamInstance {
    pub fn new(data: Rc<Vec<Rc<JavaObject>>>) -> NativeStreamInstance {
        let list: Box<dyn StreamFunction> = NativeStreamData::new(data);
        let op: Vec<RefCell<Box<dyn StreamFunction>>> = vec![RefCell::new(list)];
        NativeStreamInstance {
            operations: op
        }
    }
}

pub struct NativeStreamClass {
}

impl JavaClass for NativeStreamClass {
    fn get_name(&self) -> String {
        return "java/util/stream/Stream".to_string();
    }

    fn print(&self) {
        println!("Native Stream class");
    }

    fn execute_method(&self, jvm: &mut JVM, classes: &Classes, method_name: &String) {
        let arg = jvm.pop();
        let this = jvm.pop();

        match &method_name[..] {
            "filter" | "map" | "forEach" => {
                match &*this {
                    JavaObject::InstanceStream(stream) => {
                        match &*arg {
                            JavaObject::InstancePredicate(predicate) => {
                                if !method_name.eq("filter") {
                                    panic!("Stream.{}() does not support a predicate", method_name);
                                }

                                let new_predicate = Box::new((*(predicate.borrow_mut())).clone());
                                stream.borrow_mut().operations.insert(0, RefCell::new(new_predicate));
                            },
                            JavaObject::InstanceFunction(function) => {
                                if !method_name.eq("map") {
                                    panic!("Stream.{}() does not support a function", method_name);
                                }

                                let new_function = Box::new((*(function.borrow_mut())).clone());
                                stream.borrow_mut().operations.insert(0, RefCell::new(new_function));
                            },
                            JavaObject::InstanceConsumer(consumer) => {
                                if !method_name.eq("forEach") {
                                    panic!("Stream.{}() does not support a function", method_name);
                                }

                                let operations = &stream.borrow().operations;

                                let current_function = match operations.get(0) {
                                    Some(function) => &*function,
                                    _ => panic!("Stream.{}(): missing function 0", method_name)
                                };
                                let class = classes.get_class(&consumer.borrow().class_name);

                                loop {
                                    match current_function.borrow_mut().next_object(0, &*stream.borrow(), jvm, &classes) {
                                        Some(object) => {
                                            jvm.variables[0] = object;
                                            class.execute_method(jvm, classes, &consumer.borrow().method_name);
                                        },
                                        None => break
                                    }
                                }
                            }
                            _ => panic!("Stream.{}() does not support the argument", method_name)
                        };
                    },
                    _ => panic!("Stream.filter() expects an instance as its first argument")
                };

                jvm.push(this);
                return;
            },
            _ => panic!("Native class {} does not have method [{}]", self.get_name(), method_name)
        };
    }

    fn execute_static_method(&self, _jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }

    fn get_static_object(&self, _field_name: &String) -> JavaObject {
        panic!("Not implemented yet");
    }
}

/////////////////// java.lang.invoke.LambdaMetafactory

pub struct NativeLambdaMetafactoryClass { }

impl JavaClass for NativeLambdaMetafactoryClass {
    fn get_name(&self) -> String {
        return "java/lang/invoke/LambdaMetafactory".to_string();
    }

    fn print(&self) {
        println!("Native Stream class");
    }

    fn execute_method(&self, _jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        panic!("Native class {} does not have method [{}]", self.get_name(), method_name);
    }

    fn execute_static_method(&self, jvm: &mut JVM, classes: &Classes, method_name: &String) {
        if method_name.eq("metafactory") {
            let _arg3 = jvm.pop();
            let arg2 = jvm.pop();
            let _arg1 = jvm.pop();
            let _type = jvm.pop();
            let action = jvm.pop();
            let class = jvm.pop();

            let class = match &*class {
                JavaObject::STRING(class_name) => classes.get_class(class_name),
                _ => panic!("LambdaMetafactory.metafactory(): expecting arg 1 to be a string")
            };

            let bootstrap_method = match &*arg2 {
                JavaObject::INTEGER(int) => match class.get_method_handles().get(&(*int as usize)) {
                    Some(method) => method,
                    _ => panic!("LambdaMetafactory.metafactory(): unknown method handle {}", int)
                },
                _ => panic!("LambdaMetafactory.metafactory(): expecting arg 5 to be an integer")
            };

            let class_name = bootstrap_method.class_name.clone();
            let method_name = bootstrap_method.method_name.clone();

            match &*action {
                JavaObject::STRING(str) => {
                    match &str[..] {
                        "test" => {
                            let object = NativePredicateInstance {
                                class_name,
                                method_name
                            };
                            jvm.push(Rc::new(JavaObject::InstancePredicate(RefCell::new(object))));
                            return;
                        },
                        "apply" => {
                            let object = NativeFunctionInstance {
                                class_name,
                                method_name
                            };
                            jvm.push(Rc::new(JavaObject::InstanceFunction(RefCell::new(object))));
                            return;
                        },
                        "accept" => {
                            let object = NativeConsumerInstance {
                                class_name,
                                method_name
                            };
                            jvm.push(Rc::new(JavaObject::InstanceConsumer(RefCell::new(object))));
                            return;
                        },
                        _ => panic!("LambdaMetafactory.metafactory(): Unsupported command {}", str)
                    };
                },
                _ => panic!("LambdaMetafactory.metafactory(): expecting arg 2 to be a string")
            };
        }
        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }

    fn get_static_object(&self, _field_name: &String) -> JavaObject {
        panic!("Not implemented yet");
    }
}

/////////////////// java.lang.util.function.Predicate

pub struct NativePredicateInstance {
    class_name: String,
    method_name: String
}

impl Clone for NativePredicateInstance {
    fn clone(&self) -> NativePredicateInstance {
        NativePredicateInstance {
            class_name: self.class_name.clone(),
            method_name: self.method_name.clone()
        }
    }
}

pub struct NativePredicateClass { }

impl StreamFunction for NativePredicateInstance {
    fn next_object(&mut self, function_idx: usize, stream: &NativeStreamInstance, jvm: &mut JVM, classes: &Classes) -> Option<Rc<JavaObject>> {
        match stream.operations.get(function_idx + 1) {
            Some(function) => {
                let class = classes.get_class(&self.class_name);

                loop {
                    let object = function.borrow_mut().next_object(function_idx + 1, &stream, jvm, classes);

                    match object {
                        Some(obj) => {
                            jvm.variables[0] = obj.clone();
                            class.execute_method(jvm, classes, &self.method_name);
                            let result = jvm.pop();
                            match &*result {
                                JavaObject::BOOLEAN(is_predicate_valid) => {
                                    if *is_predicate_valid {
                                        return Some(obj.clone());
                                    }
                                },
                                _ => panic!("Predicate expects an integer as a result")
                            };
                        },
                        None => return None
                    };
                }
            },
            _ => panic!("No more function")
        };
    }
    fn print(&self) { println!("Predicate"); }
}

impl JavaClass for NativePredicateClass {
    fn get_name(&self) -> String {
        return "java/lang/util/function/Predicate".to_string();
    }

    fn print(&self) {
        println!("Native Predicate class");
    }

    fn execute_method(&self, _jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        panic!("Native class {} does not have method [{}]", self.get_name(), method_name);
    }

    fn execute_static_method(&self, _jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }

    fn get_static_object(&self, _field_name: &String) -> JavaObject {
        panic!("Not implemented yet");
    }
}

pub struct NativeFunctionInstance {
    class_name: String,
    method_name: String
}

impl Clone for NativeFunctionInstance {
    fn clone(&self) -> NativeFunctionInstance {
        NativeFunctionInstance {
            class_name: self.class_name.clone(),
            method_name: self.method_name.clone()
        }
    }
}

impl StreamFunction for NativeFunctionInstance {
    fn next_object(&mut self, function_idx: usize, stream: &NativeStreamInstance, jvm: &mut JVM, classes: &Classes) -> Option<Rc<JavaObject>> {
        match stream.operations.get(function_idx + 1) {
            Some(function) => {
                let class = classes.get_class(&self.class_name);

                let object = function.borrow_mut().next_object(function_idx + 1, &stream, jvm, classes);

                match object {
                    Some(obj) => {
                        jvm.variables[0] = obj;
                        class.execute_method(jvm, classes, &self.method_name);
                        return Some(jvm.pop());
                    },
                    None => return None
                };
            },
            _ => panic!("No more function")
        };
    }
    fn print(&self) { println!("Function"); }
}

pub struct NativeFunctionClass { }

impl JavaClass for NativeFunctionClass {
    fn get_name(&self) -> String {
        return "java/lang/util/function/Function".to_string();
    }

    fn print(&self) {
        println!("Native Function class");
    }

    fn execute_method(&self, _jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        panic!("Native class {} does not have method [{}]", self.get_name(), method_name);
    }

    fn execute_static_method(&self, _jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }

    fn get_static_object(&self, _field_name: &String) -> JavaObject {
        panic!("Not implemented yet");
    }
}

pub struct NativeConsumerInstance {
    class_name: String,
    method_name: String
}

impl Clone for NativeConsumerInstance {
    fn clone(&self) -> NativeConsumerInstance {
        NativeConsumerInstance {
            class_name: self.class_name.clone(),
            method_name: self.method_name.clone()
        }
    }
}

pub struct NativeConsumerClass { }

impl JavaClass for NativeConsumerClass {
    fn get_name(&self) -> String {
        return "java/lang/util/function/Consumer".to_string();
    }

    fn print(&self) {
        println!("Native Consumer class");
    }

    fn execute_method(&self, _jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        panic!("Native class {} does not have method [{}]", self.get_name(), method_name);
    }

    fn execute_static_method(&self, _jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }

    fn get_static_object(&self, _field_name: &String) -> JavaObject {
        panic!("Not implemented yet");
    }
}
