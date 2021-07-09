mod java_class;
mod jvm;
mod bytecode;
mod bytecode_class;
mod native_java_classes;
mod streams;
mod bytecode_test;
mod asm;

use std::collections::HashSet;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

extern crate clap;
use clap::{Arg, App};
use java_class::MethodCallResult;
use jvm::JavaInstance;
use native_java_classes::{NativeGenericClass, NativeNullInstance};
use native_java_classes::NativeStringInstance;

use crate::java_class::JavaClass;
use crate::bytecode_class::BytecodeClass;
use crate::native_java_classes::register_native_classes;
use crate::jvm::StackFrame;

struct Classes {
    classes: Option<HashMap<String, Arc<dyn JavaClass>>>
}

impl Classes {
    fn add(&mut self, value: Arc<dyn JavaClass>) {
        let key = value.get_name();
        self.classes.as_mut().unwrap().insert(key, value);
    }

    fn has(&self, class_name: &String) -> bool {
        self.classes.as_ref().unwrap().contains_key(class_name)
    }

    fn all(&self) -> Vec<Arc<dyn JavaClass>> {
        let mut classes: Vec<Arc<dyn JavaClass>> = Vec::new();
        
        let map = match self.classes.as_ref() {
            Some(m) => m,
            _ => return Vec::new()
        };

        for class in map.values() {
            classes.push(class.clone());
        }

        classes
    }

    fn add_bytecode(&mut self, name: String) -> Arc<dyn JavaClass> {
//        let ptr: Arc<dyn JavaClass> = Arc::new(BytecodeClass::parse(&name));

        let class = Arc::new(BytecodeClass::parse(&name));
        self.classes.as_mut().unwrap().insert(name.clone(), class.clone());
        class
    }
}

// An (unfinished) attempt to have the classes available as a global variable
static mut CLASSES: Classes = Classes { classes: None };
static mut DEBUG: u8 = 0;

pub fn get_debug() -> u8 { unsafe { DEBUG } }
pub fn get_class(class_name: &String) -> Arc<dyn JavaClass> {
    unsafe {
        match &CLASSES.classes {
            Some(map) => {
                let arrays_name = "java/util/Arrays".to_string();
                let class_name_to_find = if class_name.starts_with("[") { &arrays_name } else { class_name };
                match map.get(class_name_to_find) {
                    Some(class) => class.clone(),
                    _ => panic!("Class {} not found", class_name_to_find)
                }
            },
            _ => panic!("Class repository not initialized (key {} not found)", class_name)
        }
    }
}
pub fn class_exists(class_name: &String) -> bool {
    unsafe {
        match &CLASSES.classes {
            Some(map) => {
                let arrays_name = "java/util/Arrays".to_string();
                let class_name_to_find = if class_name.starts_with("[") { &arrays_name } else { class_name };
                map.contains_key(class_name_to_find)
            },
            _ => panic!("Class repository not initialized (key {} not found)", class_name)
        }
    }
}
pub fn get_classes() -> Vec<Arc<dyn JavaClass>> {
    unsafe {
        CLASSES.all()
    }
}

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    // Parses arguments
    let matches = App::new("JVM")
        .version("0.1.0")
        .arg(Arg::with_name("debug")
                .short("d")
                .long("debug")
                .takes_value(true)
                .help("Debug level"))
        .arg(Arg::with_name("asm")
                .short("a")
                .long("asm")
                .takes_value(false)
                .help("Compiles into assembly"))
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
    let asm = matches.is_present("asm");
    let class_name = matches.value_of("class").unwrap();
    let arguments: Vec<&str> = match matches.values_of("arguments") {
        Some(values) => values.collect(),
        _ => Vec::new()
    };

    // Setup the class repository
    unsafe {
        CLASSES.classes = Some(HashMap::new());
    }
    register_native_classes();

    // Load the class and all the dependencies
    let mut classes_to_load: HashSet<String> = HashSet::new();
    classes_to_load.insert(class_name.to_string());

    while classes_to_load.len() > 0 {
        for class_name in classes_to_load.clone().iter() {
            let java_class: Arc<dyn JavaClass> = if class_name.starts_with("java/lang") {
                Arc::new(NativeGenericClass { name: class_name.clone() })
            } else {
                unsafe { CLASSES.add_bytecode(class_name.to_string()) }
            };

            for dependent_class_name in java_class.get_dependent_classes().iter() {
                if !dependent_class_name.starts_with("[") && unsafe { !CLASSES.has(&dependent_class_name) } {
                    classes_to_load.insert(dependent_class_name.clone());
                }
            }

            classes_to_load.remove(class_name);
        }
    }

    let mut java_args: Vec<Arc<Mutex<dyn JavaInstance>>> = Vec::new();
    for i in 0..arguments.len() {
        java_args.push(Arc::new(Mutex::new(NativeStringInstance::new(String::from(arguments[i])))));
    }

    let var = Arc::new(Mutex::new(NativeNullInstance {}));
    let variables: [Arc<Mutex<dyn JavaInstance>>; 16] = [var.clone(), var.clone(), var.clone(), var.clone(),
        var.clone(), var.clone(), var.clone(), var.clone(),
        var.clone(), var.clone(), var.clone(), var.clone(),
        var.clone(), var.clone(), var.clone(), var.clone()];

    let mut sf = StackFrame::new(variables);
    sf.push_array(Arc::new(Mutex::new(java_args)));

    let classes = get_classes();
    let mut main_classes: Vec<Arc<dyn JavaClass>> = Vec::new();
    let mut hidden_classes: Vec<Arc<dyn JavaClass>> = Vec::new();

    for class in classes.iter() {
        let is_hidden = class.get_name().contains("$");
        if is_hidden {
            hidden_classes.push(class.clone());
        } else {
            main_classes.push(class.clone());
        }
    }

    for class in main_classes.iter_mut() {
        if class.has_static_init() {
            class.execute_static_method(&mut sf, &"<clinit>".to_string(), 0);
        }
    }

    for class in hidden_classes.iter_mut() {
        if class.has_static_init() {
            class.execute_static_method(&mut sf, &"<clinit>".to_string(), 0);
        }
    }

    let java_class = get_class(&String::from(class_name));
    if debug >= 2 { java_class.print(); }

    if asm {
        java_class.convert_to_intel_asm(&"main".to_string());
        return;
    }

    let result = java_class.execute_static_method(&mut sf, &"main".to_string(), 1);

    match result {
        MethodCallResult::SUCCESS => {},
        MethodCallResult::EXCEPTION(e) => {
            let mut object = e.lock().unwrap();
            object.execute_method(&mut sf, &"printStackTrace".to_string(), e.clone(), Vec::new());
        }
    };

    if debug >= 1 { sf.print_stack(); }
    if debug >= 2 { sf.print_variables(); }

     // Wait for other threads to finish.
    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(1)); 
    }
}
