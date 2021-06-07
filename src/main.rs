mod java_class;
mod jvm;
mod bytecode;
mod native_java_classes;
mod streams;

use std::rc::Rc;
use std::cell::RefCell;

extern crate clap;
use clap::{Arg, App};

use crate::java_class::JavaClass;
use crate::java_class::BytecodeClass;
use crate::native_java_classes::register_native_classes;
use crate::jvm::JVM;
use crate::jvm::Classes;
use crate::jvm::JavaObject;

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
    let class_name = matches.value_of("class").unwrap();
    let arguments: Vec<&str> = match matches.values_of("arguments") {
        Some(values) => values.collect(),
        _ => Vec::new()
    };

    // Loads the Java class
    let java_class: Rc<dyn 'static+JavaClass> = Rc::new(BytecodeClass::parse(&String::from(class_name), debug));
    if debug >= 2 { java_class.print(); }

    let mut java_args: Vec<Rc<JavaObject>> = Vec::new();
    for i in 0..arguments.len() {
        java_args.push(Rc::new(JavaObject::STRING(String::from(arguments[i]))));
    }

    let var = Rc::new(JavaObject::NULL());
    let variables: [Rc<JavaObject>; 16] = [var.clone(), var.clone(), var.clone(), var.clone(),
        var.clone(), var.clone(), var.clone(), var.clone(),
        var.clone(), var.clone(), var.clone(), var.clone(),
        var.clone(), var.clone(), var.clone(), var.clone()];

    let mut jvm = JVM::new(variables, debug);
    jvm.push(Rc::new(JavaObject::ARRAY(RefCell::new(java_args))));
    let mut classes = Classes::new();

    classes.add_class(java_class.clone());
    register_native_classes(&mut classes);

    java_class.execute_static_method(&mut jvm, &classes, &"main".to_string(), 1);
    if debug >= 1 { jvm.print_stack(); }
    if debug >= 2 { jvm.print_variables(); }
}
