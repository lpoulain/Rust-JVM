use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::JavaClass;
use crate::native_java_classes::NativeArrayListInstance;
use crate::streams::NativeStreamInstance;
use crate::streams::NativePredicateInstance;
use crate::streams::NativeFunctionInstance;
use crate::streams::NativeConsumerInstance;

//////////////////////////////////////////
// I don't know if this is the best way to implement it, but it's the only way I saw
// to be able to push a wide variety of objects in the JVM stack
pub enum JavaObject {
    STRING(String),
    INSTANCE(String, RefCell<HashMap<String, Rc<JavaObject>>>),
    ARRAY(RefCell<Vec<Rc<JavaObject>>>),
    INTEGER(i32),
    FLOAT(f32),
    DOUBLE(f64),
    LONG(i64),
    BOOLEAN(bool),
    CLASS(String),
    NULL(),
    InstanceList(RefCell<NativeArrayListInstance>),
    InstanceStream(RefCell<NativeStreamInstance>),
    InstancePredicate(RefCell<NativePredicateInstance>),
    InstanceFunction(RefCell<NativeFunctionInstance>),
    InstanceConsumer(RefCell<NativeConsumerInstance>)
}

//////////////////////////////////////////

pub struct Classes {
    classes: HashMap<String, Rc<dyn JavaClass>>
}

impl Classes {
    pub fn new() -> Classes {
        Classes {
            classes: HashMap::new()
        }
    }

    pub fn get_class(&self, class_name: &String) -> &Rc<dyn JavaClass> {
        return match self.classes.get(class_name) {
            Some(class) => class,
            _ => panic!("Unknown class {}", class_name)
        }
    }

    pub fn add_class(&mut self, class: Rc<dyn JavaClass>) {
        self.classes.insert(class.get_name(), class);
    }
}

pub trait JavaInstance {
    fn get_class_name(&self) -> String;
    fn get_int(&self) -> i32 { panic!("{} cannot be converted into an integer", self.get_class_name()); }
    fn get_long(&self) -> i64 { panic!("{} cannot be converted into an long", self.get_class_name()); }
    fn get_float(&self) -> f32 { panic!("{} cannot be converted into a float", self.get_class_name()); }
    fn get_double(&self) -> f64 { panic!("{} cannot be converted into a double", self.get_class_name()); }
    fn call_method(&self, _method_name: &String, _nb_args: usize) { panic!("{} does not support any method", self.get_class_name()); }
    fn update_field(&mut self, _value: &dyn JavaInstance) {

    }
}

struct IntegerInstance { value: i32 }
impl JavaInstance for IntegerInstance {
    fn get_class_name(&self) -> String { String::from("java/util/Integer") }
    fn update_field(&mut self, value: &dyn JavaInstance) {
        self.value = value.get_int();
    }
}



pub struct StackFrame {
    stack2: Vec<Rc<RefCell<dyn JavaInstance>>>,
    stack: Vec<Rc<JavaObject>>,
    variables: [Rc<JavaObject>; 16],
    pub debug: u8,
    pub return_arg: bool
}

impl StackFrame {
    pub fn new(variables: [Rc<JavaObject>; 16], debug: u8) -> StackFrame {
        StackFrame {
            stack2: Vec::new(),
            stack: Vec::new(),
            variables,
            debug,
            return_arg: false
        }
    }

    pub fn set_return_arg_flag(&mut self) {
        let object = self.stack2.pop().unwrap();
        object.borrow_mut().update_field(&IntegerInstance { value: 42 });
    }

    pub fn push2(&mut self, object: Rc<RefCell<dyn JavaInstance>>) {
        self.stack2.push(object.clone());
    }

    pub fn push(&mut self, object: Rc<JavaObject>) {
        self.stack.push(object.clone());
    }

    pub fn pop2(&mut self) -> Rc<RefCell<dyn JavaInstance>> {
        let instance = self.stack2.pop().unwrap();
        return instance.clone();
//        return self.stack.pop().unwrap();
    }

    pub fn pop(&mut self) -> Rc<JavaObject> {
        return self.stack.pop().unwrap();
    }

    pub fn pop_int(&mut self) -> i32 {
//        return self.pop().get_int();
        let arg = self.pop();
        return match &*arg {
            JavaObject::INTEGER(int) => *int,
            _ => panic!("Expected int")
        };
    }

    pub fn pop_long(&mut self) -> i64 {
        let arg = self.pop();
        return match &*arg {
            JavaObject::LONG(long) => *long,
            _ => panic!("Expected long")
        };
    }

    pub fn pop_float(&mut self) -> f32 {
        let arg = self.pop();
        return match &*arg {
            JavaObject::FLOAT(f) => *f,
            _ => panic!("Expected float")
        };
    }

    pub fn pop_double(&mut self) -> f64 {
        let arg = self.pop();
        return match &*arg {
            JavaObject::DOUBLE(d) => *d,
            _ => panic!("Expected double")
        };
    }

    pub fn stack_to_variable(&mut self, idx: usize) {
        self.variables[idx] = self.stack.pop().unwrap().clone();
    }

    pub fn variable_to_stack(&mut self, idx: usize) {
        self.stack.push(self.variables[idx].clone());
    }

    pub fn set_variable(&mut self, idx: usize, object: &Rc<JavaObject>) {
        self.variables[idx] = object.clone();
    }

    pub fn get_variable(&self, idx: usize) -> Rc<JavaObject> {
        return self.variables[idx].clone();
    }

    pub fn print_stack(&self) {
        for frame in &self.stack {
            print!("    > ");
            self.print_java_object(&(**frame));
            println!("");
        }
    }

    pub fn print_variables(&self) {
        for i in 0..8 {
            print!("    Var {}: ", i);
            self.print_java_object(&self.variables[i]);
            println!("");
        }
    }

    pub fn print_java_object(&self, java_object: &JavaObject) {
        match java_object {
            JavaObject::STRING(st) => print!("\"{}\"", st),
            JavaObject::INTEGER(int) => print!("{}", int),
            JavaObject::LONG(long) => print!("{}", long),
            JavaObject::BOOLEAN(b) => print!("{}", b),
            JavaObject::FLOAT(f) => print!("{}", f),
            JavaObject::DOUBLE(d) => print!("{}", d),
            JavaObject::NULL() => print!("<null>"),
            JavaObject::INSTANCE(cl, keys) => {
                print!("<{} instance> (", cl);
                let fields = keys.borrow();
                for (key, value) in fields.iter() {
                    print!("{}:", key);
                    self.print_java_object(value);
                    print!("  ");
                }
                print!(")");
            },
            JavaObject::ARRAY(array) => {
                print!("[");
                for sub_obj in array.borrow().iter() {
                    self.print_java_object(sub_obj);
                    print!(", ");
                }
                print!("]")
            },
            JavaObject::CLASS(class) => { print!("Class {}", class); }
            JavaObject::InstanceList(_) => print!("<List instance>"),
            JavaObject::InstanceStream(_) => print!("<Stream instance>"),
            JavaObject::InstanceFunction(_) => print!("<Function instance>"),
            JavaObject::InstancePredicate(_) => print!("<Predicate instance>"),
            JavaObject::InstanceConsumer(_) => print!("<Consumer instance>"),
        };
    }
}
