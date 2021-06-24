use std::sync::{Arc, Mutex};

use crate::native_java_classes::{NativeArrayInstance, NativeByteInstance, NativeCharInstance, NativeNullInstance, NativeShortInstance};
use crate::native_java_classes::NativeBooleanInstance;
use crate::native_java_classes::NativeFloatInstance;
use crate::native_java_classes::NativeDoubleInstance;
use crate::native_java_classes::NativeIntegerInstance;
use crate::native_java_classes::NativeLongInstance;
use crate::native_java_classes::NativeStringInstance;
use crate::streams::StreamFunction;

//////////////////////////////////////////

pub trait JavaInstance {
    fn is_bytecode(&self) -> bool { false }
    fn get_parent(&self) -> Option<Arc<Mutex<dyn JavaInstance>>> { None }
    fn supports_interface(&self, _interface_name: &String) -> bool { false }
    fn get_class_name(&self) -> String;
    fn get_int(&self) -> i32 { panic!("{} cannot be converted into an integer", self.get_class_name()); }
    fn get_long(&self) -> i64 { panic!("{} cannot be converted into an long", self.get_class_name()); }
    fn get_float(&self) -> f32 { panic!("{} cannot be converted into a float", self.get_class_name()); }
    fn get_double(&self) -> f64 { panic!("{} cannot be converted into a double", self.get_class_name()); }
    fn get_string(&self) -> String { panic!("{} cannot be converted into a string", self.get_class_name()); }
    fn get_bool(&self) -> bool { panic!("{} cannot be converted into a boolean", self.get_class_name()); }
    fn get_short(&self) -> i16 { panic!("{} cannot be converted into a short", self.get_class_name()); }
    fn get_byte(&self) -> u8 { panic!("{} cannot be converted into a byte", self.get_class_name()); }
    fn get_char(&self) -> char { panic!("{} cannot be converted into a char", self.get_class_name()); }
    fn get_array(&self) -> Arc<Mutex<Vec<Arc<Mutex<dyn JavaInstance>>>>> { panic!("{} cannot be converted into an array", self.get_class_name()); }
    fn is_null(&self) -> bool { false }

    fn execute_method(&mut self, _sf: &mut StackFrame, method_name: &String, _this: Arc<Mutex<dyn JavaInstance>>, _args: Vec<Arc<Mutex<dyn JavaInstance>>>) {
        panic!("{} does not support any method ({} requested)", self.get_class_name(), method_name);
    }
    fn get_field(&self, field_name: &String) -> Arc<Mutex<dyn JavaInstance>> {
        panic!("This {} instance has no field ({} requested)", self.get_class_name(), field_name);
    }
    fn set_field(&mut self, field_name: &String, _value: Arc<Mutex<dyn JavaInstance>>) {
        panic!("This {} instance has no field to update ({} requested)", self.get_class_name(), field_name);
    }
    fn get_stream_function(&self) -> Arc<Mutex<dyn StreamFunction>> { panic!("{} cannot be converted into a StreamFunction", self.get_class_name()); }
    fn print(&self) {
        print!("<{} instance>", self.get_class_name());
    }
}

pub struct StackFrame {
    stack: Vec<Arc<Mutex<dyn JavaInstance>>>,
    variables: [Arc<Mutex<dyn JavaInstance>>; 16],
    pub return_arg: bool
}

impl StackFrame {
    pub fn new(variables: [Arc<Mutex<dyn JavaInstance>>; 16]) -> StackFrame {
        StackFrame {
            stack: Vec::new(),
            variables,
            return_arg: false
        }
    }

    pub fn set_return_arg_flag(&mut self) {
        self.return_arg = true;
    }

    pub fn push(&mut self, object: Arc<Mutex<dyn JavaInstance>>) { self.stack.push(object.clone()); }
    pub fn pop(&mut self) -> Arc<Mutex<dyn JavaInstance>> { return self.stack.pop().unwrap(); }

    pub fn push_null(&mut self) { self.push(Arc::new(Mutex::new(NativeNullInstance::new()))); }
    pub fn pop_isnull(&mut self) -> bool { return (*self.pop()).lock().unwrap().is_null(); }

    pub fn pop_int(&mut self) -> i32 { return (*self.pop()).lock().unwrap().get_int(); }
    pub fn push_int(&mut self, value: i32) { self.push(Arc::new(Mutex::new(NativeIntegerInstance::new(value)))); }

    pub fn pop_long(&mut self) -> i64 { return (*self.pop()).lock().unwrap().get_long(); }
    pub fn push_long(&mut self, value: i64) { self.push(Arc::new(Mutex::new(NativeLongInstance::new(value)))); }

    pub fn pop_short(&mut self) -> i16 { return (*self.pop()).lock().unwrap().get_short(); }
    pub fn push_short(&mut self, value: i16) { self.push(Arc::new(Mutex::new(NativeShortInstance::new(value)))); }

    pub fn pop_byte(&mut self) -> u8 { return (*self.pop()).lock().unwrap().get_byte(); }
    pub fn push_byte(&mut self, value: u8) { self.push(Arc::new(Mutex::new(NativeByteInstance::new(value)))); }

    pub fn pop_char(&mut self) -> char { return (*self.pop()).lock().unwrap().get_char(); }
    pub fn push_char(&mut self, value: char) { self.push(Arc::new(Mutex::new(NativeCharInstance::new(value)))); }

    pub fn pop_float(&mut self) -> f32 { return (*self.pop()).lock().unwrap().get_float(); }
    pub fn push_float(&mut self, value: f32) { self.push(Arc::new(Mutex::new(NativeFloatInstance::new(value)))); }

    pub fn pop_double(&mut self) -> f64 { return (*self.pop()).lock().unwrap().get_double(); }
    pub fn push_double(&mut self, value: f64) { self.push(Arc::new(Mutex::new(NativeDoubleInstance::new(value)))); }

    pub fn pop_string(&mut self) -> String { return (*self.pop()).lock().unwrap().get_string(); }
    pub fn push_string(&mut self, value: String) { self.push(Arc::new(Mutex::new(NativeStringInstance::new(value)))); }

    pub fn pop_bool(&mut self) -> bool { return (*self.pop()).lock().unwrap().get_bool(); }
    pub fn push_bool(&mut self, value: bool) { self.push(Arc::new(Mutex::new(NativeBooleanInstance::new(value)))); }

    pub fn pop_array(&mut self) -> Arc<Mutex<Vec<Arc<Mutex<dyn JavaInstance>>>>> {
        let object = self.pop();
        return object.lock().unwrap().get_array();
    }
    pub fn push_array(&mut self, value: Arc<Mutex<Vec<Arc<Mutex<dyn JavaInstance>>>>>) { self.push(Arc::new(Mutex::new(NativeArrayInstance { values: value }))); }

    pub fn stack_to_variable(&mut self, idx: usize) {
        self.variables[idx] = self.stack.pop().unwrap().clone();
    }

    pub fn variable_to_stack(&mut self, idx: usize) {
        self.stack.push(self.variables[idx].clone());
    }

    pub fn print_stack(&self) {
        println!("    Stack:");
        for frame in &self.stack {
            print!("    > ");
            (**frame).lock().unwrap().print();
            println!("");
        }
    }

    pub fn print_variables(&self) {
        for i in 0..8 {
            print!("    Var {}: ", i);
            (*self.variables[i]).lock().unwrap().print();
            println!("");
        }
    }
}
