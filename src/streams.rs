use std::sync::{Arc, Mutex};

use crate::get_class;
use crate::jvm::JavaInstance;
use crate::jvm::StackFrame;
use crate::java_class::JavaClass;

/////////////////// java.util.stream.Stream

pub trait StreamFunction {
    fn next_object(&mut self, function_idx: usize, stream: &NativeStreamInstance, sf: &mut StackFrame) -> Option<Arc<Mutex<dyn JavaInstance>>>;
    fn get_class_name(&self) -> String { return "".to_string(); }
    fn get_method_name(&self) -> String { return "".to_string(); }
    fn print(&self);
}

pub struct NativeStreamData {
    data: Arc<Mutex<Vec<Arc<Mutex<dyn JavaInstance>>>>>,
    idx: usize
}

impl NativeStreamData {
    pub fn new(data: Arc<Mutex<Vec<Arc<Mutex<dyn JavaInstance>>>>>) -> Arc<Mutex<dyn StreamFunction>> {
        Arc::new(Mutex::new(NativeStreamData {
            data,
            idx: 0
        }))
    }
}

impl StreamFunction for NativeStreamData {
    fn next_object(&mut self, _function_idx: usize, _stream: &NativeStreamInstance, _sf: &mut StackFrame) -> Option<Arc<Mutex<dyn JavaInstance>>> {
        let object = match self.data.lock().unwrap().get(self.idx) {
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
    pub operations: Vec<Arc<Mutex<dyn StreamFunction>>>
}
impl JavaInstance for NativeStreamInstance {
    fn get_class_name(&self) -> String {
        return "java/util/stream/Stream".to_string();
    }
    fn execute_method(&mut self, sf: &mut StackFrame, method_name: &String, this: Arc<Mutex<dyn JavaInstance>>, args: Vec<Arc<Mutex<dyn JavaInstance>>>) {
        match &method_name[..] {
            "filter" | "map" => {
                let stream_function = args.get(0).unwrap().lock().unwrap().get_stream_function();
                self.operations.insert(0, stream_function);
                sf.push(this.clone());
            },
            "forEach" => {
                let arg = args.get(0).unwrap();
                let consumer = arg.lock().unwrap().get_stream_function();
                let current_function = match self.operations.get(0) {
                    Some(function) => &*function,
                    _ => panic!("Stream.{}(): missing function 0", method_name)
                };
                let class = get_class(&consumer.lock().unwrap().get_class_name());

                loop {
                    match current_function.lock().unwrap().next_object(0, self, sf) {
                        Some(object) => {
                            sf.push(object.clone());
                            class.execute_static_method(sf, &consumer.lock().unwrap().get_method_name(), 1);
                        },
                        None => break
                    }
                }
            },
            _ => panic!("Native class {} does not have method [{}]", self.get_class_name(), method_name)
        };
    }
}

impl NativeStreamInstance {
    pub fn new(data: Arc<Mutex<Vec<Arc<Mutex<dyn JavaInstance>>>>>) -> NativeStreamInstance {
        let list: Arc<Mutex<dyn StreamFunction>> = NativeStreamData::new(data);
        let op: Vec<Arc<Mutex<dyn StreamFunction>>> = vec![list];
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

    fn execute_static_method(&self, sf: &mut StackFrame, method_name: &String, _nb_args: usize) {
        if method_name.eq("metafactory") {
            let _arg3 = sf.pop();
            let arg2 = sf.pop_int();
            let _arg1 = sf.pop();
            let _type = sf.pop();
            let action = sf.pop_string();
            let class_name = sf.pop_string();

            let class = get_class(&class_name);
            let the_class = class;

            let bootstrap_method = match the_class.get_method_handles().get(&(arg2 as usize)) {
                Some(method) => method,
                _ => panic!("LambdaMetafactory.metafactory(): unknown method handle {}", arg2)
            };

            let class_name = bootstrap_method.class_name.clone();
            let method_name = bootstrap_method.method_name.clone();
            let type_desc = bootstrap_method.type_name.clone();

            match &action[..] {
                "test" => {
                    let object = NativePredicateInstance {
                        class_name,
                        method_name,
                        type_desc
                    };
                    sf.push(Arc::new(Mutex::new(object)));
                },
                "apply" => {
                    let object = NativeFunctionInstance {
                        class_name,
                        method_name,
                        type_desc
                    };
                    sf.push(Arc::new(Mutex::new(object)));
                },
                "accept" => {
                    let object = NativeConsumerInstance {
                        class_name,
                        method_name,
                        type_desc
                    };
                    sf.push(Arc::new(Mutex::new(object)));
                },
                _ => panic!("LambdaMetafactory.metafactory(): Unsupported command {}", action)
            };
            return;
        }
        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }
}

/////////////////// java.lang.util.function.Predicate

pub struct NativePredicateInstance {
    class_name: String,
    method_name: String,
    type_desc: String
}

impl JavaInstance for NativePredicateInstance {
    fn get_class_name(&self) -> String {
        return "java/lang/util/function/Predicate".to_string();
    }
    fn get_stream_function(&self) -> Arc<Mutex<dyn StreamFunction>> {
        return Arc::new(Mutex::new(self.clone()));
    }
}

impl Clone for NativePredicateInstance {
    fn clone(&self) -> NativePredicateInstance {
        NativePredicateInstance {
            class_name: self.class_name.clone(),
            method_name: self.method_name.clone(),
            type_desc: self.type_desc.clone()
        }
    }
}

pub struct NativePredicateClass { }

impl StreamFunction for NativePredicateInstance {
    fn next_object(&mut self, function_idx: usize, stream: &NativeStreamInstance, sf: &mut StackFrame) -> Option<Arc<Mutex<dyn JavaInstance>>> {
        match stream.operations.get(function_idx + 1) {
            Some(function) => {
                let class = get_class(&self.class_name);

                loop {
                    let object = (**function).lock().unwrap().next_object(function_idx + 1, &stream, sf);

                    match object {
                        Some(obj) => {
                            sf.push(obj.clone());
                            class.execute_static_method(sf, &self.method_name, 1);
                            let is_predicate_valid = sf.pop_bool();
                            if is_predicate_valid {
                                return Some(obj.clone());
                            }
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
}

pub struct NativeFunctionInstance {
    class_name: String,
    method_name: String,
    type_desc: String
}

impl JavaInstance for NativeFunctionInstance {
    fn get_class_name(&self) -> String {
        return "java/lang/util/function/Function".to_string();
    }
    fn get_stream_function(&self) -> Arc<Mutex<dyn StreamFunction>> {
        return Arc::new(Mutex::new(self.clone()));
    }
}

impl Clone for NativeFunctionInstance {
    fn clone(&self) -> NativeFunctionInstance {
        NativeFunctionInstance {
            class_name: self.class_name.clone(),
            method_name: self.method_name.clone(),
            type_desc: self.type_desc.clone()
        }
    }
}

impl StreamFunction for NativeFunctionInstance {
    fn next_object(&mut self, function_idx: usize, stream: &NativeStreamInstance, sf: &mut StackFrame) -> Option<Arc<Mutex<dyn JavaInstance>>> {
        match stream.operations.get(function_idx + 1) {
            Some(function) => {
                let class = get_class(&self.class_name);

                let object = function.lock().unwrap().next_object(function_idx + 1, &stream, sf);

                match object {
                    Some(obj) => {
                        sf.push(obj);
                        class.execute_static_method(sf, &self.method_name, 1);
                        return Some(sf.pop());
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
}

pub struct NativeConsumerInstance {
    class_name: String,
    method_name: String,
    type_desc: String
}

impl JavaInstance for NativeConsumerInstance {
    fn get_class_name(&self) -> String {
        return "java/lang/util/function/Consumer".to_string();
    }
    fn get_stream_function(&self) -> Arc<Mutex<dyn StreamFunction>> {
        return Arc::new(Mutex::new(self.clone()));
    }
}

impl StreamFunction for NativeConsumerInstance {
    fn next_object(&mut self, _function_idx: usize, _stream: &NativeStreamInstance, _sf: &mut StackFrame) -> Option<Arc<Mutex<dyn JavaInstance>>> {
        return None;
    }

    fn get_class_name(&self) -> String {
        return self.class_name.clone();
    }

    fn get_method_name(&self) -> String {
        return self.method_name.clone();
    }

    fn print(&self) {
        print!("Consumer");
    }
}

impl Clone for NativeConsumerInstance {
    fn clone(&self) -> NativeConsumerInstance {
        NativeConsumerInstance {
            class_name: self.class_name.clone(),
            method_name: self.method_name.clone(),
            type_desc: self.type_desc.clone()
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
}
