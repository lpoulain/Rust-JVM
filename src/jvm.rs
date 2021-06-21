use std::rc::Rc;
use std::cell::RefCell;

use crate::native_java_classes::NativeArrayInstance;
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
    fn get_parent(&self) -> Option<Rc<RefCell<dyn JavaInstance>>> { None }
    fn supports_interface(&self, _interface_name: &String) -> bool { false }
    fn get_class_name(&self) -> String;
    fn get_int(&self) -> i32 { panic!("{} cannot be converted into an integer", self.get_class_name()); }
    fn get_long(&self) -> i64 { panic!("{} cannot be converted into an long", self.get_class_name()); }
    fn get_float(&self) -> f32 { panic!("{} cannot be converted into a float", self.get_class_name()); }
    fn get_double(&self) -> f64 { panic!("{} cannot be converted into a double", self.get_class_name()); }
    fn get_string(&self) -> String { panic!("{} cannot be converted into a double", self.get_class_name()); }
    fn get_bool(&self) -> bool { panic!("{} cannot be converted into a boolean", self.get_class_name()); }
    fn get_array(&self) -> Rc<RefCell<Vec<Rc<RefCell<dyn JavaInstance>>>>> { panic!("{} cannot be converted into an array", self.get_class_name()); }
    fn execute_method(&mut self, _sf: &mut StackFrame, method_name: &String, _this: Rc<RefCell<dyn JavaInstance>>, _args: Vec<Rc<RefCell<dyn JavaInstance>>>) {
        panic!("{} does not support any method ({} requested)", self.get_class_name(), method_name);
    }
    fn get_field(&self, field_name: &String) -> Rc<RefCell<dyn JavaInstance>> {
        panic!("This {} instance has no field ({} requested)", self.get_class_name(), field_name);
    }
    fn set_field(&mut self, field_name: &String, _value: Rc<RefCell<dyn JavaInstance>>) {
        panic!("This {} instance has no field to update ({} requested)", self.get_class_name(), field_name);
    }
    fn get_stream_function(&self) -> Rc<RefCell<dyn StreamFunction>> { panic!("{} cannot be converted into a StreamFunction", self.get_class_name()); }
    fn print(&self) {
        print!("<{} instance>", self.get_class_name());
    }
}

pub struct StackFrame {
    stack: Vec<Rc<RefCell<dyn JavaInstance>>>,
    variables: [Rc<RefCell<dyn JavaInstance>>; 16],
    pub debug: u8,
    pub return_arg: bool
}

impl StackFrame {
    pub fn new(variables: [Rc<RefCell<dyn JavaInstance>>; 16], debug: u8) -> StackFrame {
        StackFrame {
            stack: Vec::new(),
            variables,
            debug,
            return_arg: false
        }
    }

    pub fn set_return_arg_flag(&mut self) {
        self.return_arg = true;
    }

    pub fn push(&mut self, object: Rc<RefCell<dyn JavaInstance>>) { self.stack.push(object.clone()); }
    pub fn pop(&mut self) -> Rc<RefCell<dyn JavaInstance>> { return self.stack.pop().unwrap(); }

    pub fn pop_int(&mut self) -> i32 { return (*self.pop()).borrow().get_int(); }
    pub fn push_int(&mut self, value: i32) { self.push(Rc::new(RefCell::new(NativeIntegerInstance::new(value)))); }

    pub fn pop_long(&mut self) -> i64 { return (*self.pop()).borrow().get_long(); }
    pub fn push_long(&mut self, value: i64) { self.push(Rc::new(RefCell::new(NativeLongInstance::new(value)))); }
 
    pub fn pop_float(&mut self) -> f32 { return (*self.pop()).borrow().get_float(); }
    pub fn push_float(&mut self, value: f32) { self.push(Rc::new(RefCell::new(NativeFloatInstance::new(value)))); }

    pub fn pop_double(&mut self) -> f64 { return (*self.pop()).borrow().get_double(); }
    pub fn push_double(&mut self, value: f64) { self.push(Rc::new(RefCell::new(NativeDoubleInstance::new(value)))); }

    pub fn pop_string(&mut self) -> String { return (*self.pop()).borrow().get_string(); }
    pub fn push_string(&mut self, value: String) { self.push(Rc::new(RefCell::new(NativeStringInstance::new(value)))); }

    pub fn pop_bool(&mut self) -> bool { return (*self.pop()).borrow().get_bool(); }
    pub fn push_bool(&mut self, value: bool) { self.push(Rc::new(RefCell::new(NativeBooleanInstance::new(value)))); }

    pub fn pop_array(&mut self) -> Rc<RefCell<Vec<Rc<RefCell<dyn JavaInstance>>>>> {
        let object = self.pop();
        return object.borrow().get_array();
    }
    pub fn push_array(&mut self, value: Rc<RefCell<Vec<Rc<RefCell<dyn JavaInstance>>>>>) { self.push(Rc::new(RefCell::new(NativeArrayInstance { values: value }))); }

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
            (**frame).borrow().print();
            println!("");
        }
    }

    pub fn print_variables(&self) {
        for i in 0..8 {
            print!("    Var {}: ", i);
            (*self.variables[i]).borrow().print();
            println!("");
        }
    }
}
