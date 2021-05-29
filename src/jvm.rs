use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::JavaClass;
use crate::native_java_classes::NativeListClass;
use crate::streams::NativeStreamInstance;
use crate::streams::NativePredicateInstance;
use crate::streams::NativeFunctionInstance;
use crate::streams::NativeConsumerInstance;

//////////////////////////////////////////
// I don't know if this is the best way to implement it, but it's the only way I saw
// to be able to push a wide variety of objects in the JVM stack
pub enum JavaObject {
    STRING(String),
    INSTANCE(String, HashMap<String, Rc<JavaObject>>),
    ARRAY(RefCell<Vec<Rc<JavaObject>>>),
    INTEGER(i32),
    BOOLEAN(bool),
    InstanceList(RefCell<NativeListClass>),
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

pub struct JVM {
    stack: Vec<Rc<JavaObject>>,
    pub var0: Rc<JavaObject>,
    pub var1: Rc<JavaObject>,
    pub var2: Rc<JavaObject>,
    pub var3: Rc<JavaObject>,
    pub debug: u8
}

impl JVM {
    pub fn new(args: &Vec<&str>, debug: u8) -> JVM {
        let mut java_args: Vec<Rc<JavaObject>> = Vec::new();
        for i in 0..args.len() {
            java_args.push(Rc::new(JavaObject::STRING(String::from(args[i]))));
            if debug >= 1 { println!("Arg: {}", args[i]); }
        }

        JVM {
            stack: Vec::new(),
            var0: Rc::new(JavaObject::ARRAY(RefCell::new(java_args))),
            var1: Rc::new(JavaObject::INTEGER(0)),
            var2: Rc::new(JavaObject::INTEGER(0)),
            var3: Rc::new(JavaObject::INTEGER(0)),
            debug: debug
        }
    }

    pub fn push(&mut self, object: Rc<JavaObject>) {
        self.stack.push(object);
    }

    pub fn pop(&mut self) -> Rc<JavaObject> {
        return match self.stack.pop() {
            Some(object) => object,
            _ => panic!("Stack empty, nothing to pop")
        };
    }

    pub fn pop_int(&mut self) -> i32 {
        let arg = self.pop();
        return match &*arg {
            JavaObject::INTEGER(int) => *int,
            _ => panic!("Expected integer")
        };
    }

    pub fn print_stack(&self) {
        for frame in &self.stack {
            print!("> ");
            self.print_java_object(&(**frame));
            println!("");
        }
    }

    pub fn print_java_object(&self, java_object: &JavaObject) {
        match java_object {
            JavaObject::STRING(st) => print!("\"{}\"", st),
            JavaObject::INTEGER(int) => print!("{}", int),
            JavaObject::BOOLEAN(b) => print!("{}", b),
            JavaObject::INSTANCE(cl, keys) => {
                print!("{} instance (", cl);
                for (key, value) in keys {
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
            JavaObject::InstanceList(_) => print!("List instance"),
            JavaObject::InstanceStream(_) => print!("Stream instance"),
            JavaObject::InstanceFunction(_) => print!("Function instance"),
            JavaObject::InstancePredicate(_) => print!("Predicate instance"),
            JavaObject::InstanceConsumer(_) => print!("Consumer instance"),
        };
    }
}
