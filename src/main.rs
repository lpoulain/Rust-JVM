mod java_class;
mod jvm;
mod bytecode;
mod bytecode_class;
mod native_java_classes;
mod streams;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::HashMap;

extern crate clap;
use clap::{Arg, App};
use jvm::JavaInstance;
use native_java_classes::NativeObjectInstance;
use native_java_classes::NativeStringInstance;

use crate::java_class::JavaClass;
use crate::bytecode_class::BytecodeClass;
use crate::native_java_classes::register_native_classes;
use crate::jvm::StackFrame;
use crate::jvm::Classes;

struct Classes2 {
    classes: Option<HashMap<String, Rc<RefCell<dyn JavaClass>>>>
}

impl Classes2 {
    fn add(&mut self, value: Rc<RefCell<dyn JavaClass>>) {
        let key = value.borrow().get_name();
        self.classes.as_mut().unwrap().insert(key, value);
    }

    fn add_bytecode(&mut self, name: String, debug: u8) {
        self.classes.as_mut().unwrap().insert(name.clone(), Rc::new(RefCell::new(BytecodeClass::parse(&name, debug))));
    }

    fn get(&self, key: &String) -> Rc<RefCell<dyn JavaClass>> {
        match &self.classes {
            Some(map) => map.get(key).unwrap().clone(),
            _ => panic!("Class repository not initialized (key {} not found)", key)
        }
    }
}

// An (unfinished) attempt to have the classes available as a global variable
static mut CLASSES: Classes2 = Classes2 { classes: None };
static mut DEBUG: u8 = 0;

fn main() {
    // Parses arguments
    let matches = App::new("JVM")
        .version("0.1.0")
        .arg(Arg::with_name("debug")
                .short("d")
                .long("debug")
                .takes_value(true)
                .help("Debug level"))
        .arg(Arg::with_name("class")
                .takes_value(false)
                .required(true))
        .arg(Arg::with_name("arguments")
                .takes_value(false)
                .multiple(true))
        .get_matches();

    let debug: u8 = match matches.value_of("debug") {
        Some(st) => st.parse::<u8>().unwrap(),
        _ => 0
    };
    unsafe {
        DEBUG = debug;
    }
    let class_name = matches.value_of("class").unwrap();
    let arguments: Vec<&str> = match matches.values_of("arguments") {
        Some(values) => values.collect(),
        _ => Vec::new()
    };

    // Setup the class repository
    let mut classes = Classes::new();
    unsafe {
        CLASSES.classes = Some(HashMap::new());
    }
    register_native_classes(&mut classes);

    // Load the class and all the dependencies
    let mut classes_to_load: HashSet<String> = HashSet::new();
    classes_to_load.insert(class_name.to_string());

    while classes_to_load.len() > 0 {
        for class_name in classes_to_load.clone().iter() {
            let bytecode_class = BytecodeClass::parse(&String::from(class_name), debug);
            let java_class: Rc<RefCell<dyn 'static+JavaClass>> = Rc::new(RefCell::new(bytecode_class));
            classes.add_class(java_class.clone());
            unsafe { CLASSES.add(java_class.clone()); }

            for dependent_class_name in java_class.borrow().get_dependent_classes().iter() {
                if !dependent_class_name.starts_with("[") && !classes.has_class(&dependent_class_name) {
                    classes_to_load.insert(dependent_class_name.clone());
                }
            }

            classes_to_load.remove(class_name);
        }
    }

    let mut java_args: Vec<Rc<RefCell<dyn JavaInstance>>> = Vec::new();
    for i in 0..arguments.len() {
        java_args.push(Rc::new(RefCell::new(NativeStringInstance::new(String::from(arguments[i])))));
    }

    let var = Rc::new(RefCell::new(NativeObjectInstance {}));
    let variables: [Rc<RefCell<dyn JavaInstance>>; 16] = [var.clone(), var.clone(), var.clone(), var.clone(),
        var.clone(), var.clone(), var.clone(), var.clone(),
        var.clone(), var.clone(), var.clone(), var.clone(),
        var.clone(), var.clone(), var.clone(), var.clone()];

    let mut sf = StackFrame::new(variables, debug);
    sf.push_array(Rc::new(RefCell::new(java_args)));
    
    let mut classes_to_init: Vec<Rc<RefCell<dyn JavaClass>>> = Vec::new();

    for (_, class) in classes.classes.iter_mut() {
        classes_to_init.push(class.clone());
        unsafe {
            CLASSES.add(class.clone());
        }
    }

    for class in classes_to_init.iter_mut() {
        class.borrow_mut().init_static_fields(&classes);
    }

    for class in classes_to_init.iter_mut() {
        if class.borrow().has_static_init() {
            class.borrow().execute_static_method(&mut sf, &classes, &"<clinit>".to_string(), 0);
        }
    }

    let java_class = classes.get_class(&String::from(class_name));
    if debug >= 2 { java_class.borrow().print(); }

    java_class.borrow().execute_static_method(&mut sf, &classes, &"main".to_string(), 1);
    if debug >= 1 { sf.print_stack(); }
    if debug >= 2 { sf.print_variables(); }
}
