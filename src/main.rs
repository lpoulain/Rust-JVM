mod java_class;
mod jvm;
mod bytecode;

use std::rc::Rc;

extern crate clap;
use clap::{Arg, App};

use crate::java_class::JavaClass;
use crate::java_class::BytecodeClass;
use crate::java_class::NativePrintStreamClass;
use crate::java_class::NativeSystemClass;
use crate::java_class::NativeStringClass;
use crate::java_class::NativeIntegerClass;
use crate::jvm::JVM;
use crate::jvm::Classes;

fn main() {
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

    let java_class: Rc<dyn 'static+JavaClass> = Rc::new(BytecodeClass::new(&String::from(class_name), debug));
    if debug >= 2 { java_class.print(); }

    let mut jvm = JVM::new(&arguments, debug);
    let mut classes = Classes::new();

    classes.add_class(java_class.clone());
    classes.add_class(Rc::new(NativePrintStreamClass {}));
    classes.add_class(Rc::new(NativeSystemClass {}));
    classes.add_class(Rc::new(NativeStringClass {}));
    classes.add_class(Rc::new(NativeIntegerClass {}));

    java_class.execute_static_method(&mut jvm, &classes, &"main".to_string());
}
