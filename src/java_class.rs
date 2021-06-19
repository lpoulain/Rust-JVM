use std::collections::HashMap;
use std::rc::Rc;

use crate::StackFrame;
use crate::bytecode_class::AttributeBootstrapMethod;
use crate::bytecode_class::ConstantMethodHandle;
use crate::jvm::JavaInstance;
use crate::jvm::Classes;
use std::cell::RefCell;

pub fn get_nb_arguments(type_desc: &String) -> usize {
    let start_bytes = type_desc.find("(").unwrap_or(0);
    let end_bytes = type_desc.find(")").unwrap_or(type_desc.len());
    let arguments = &type_desc[start_bytes+1..end_bytes];

    let mut nb_arguments = 0;
    let mut idx: usize = 0;
    let size: usize = arguments.len();
    while idx < size {
        match arguments.chars().nth(idx) {
            Some('B') | Some('C')| Some('D')| Some('F')| Some('I')| Some('J')| Some('S')| Some('Z') => { nb_arguments += 1; idx += 1; },
            Some('L') => {
                idx += arguments[idx..].find(";").unwrap() + 1;
                nb_arguments += 1;
            }
            Some(_) => { idx += 1; }
            None => { break; }
        }
    };
    return nb_arguments;
}

///////////////////////////////////////////
///////////////////////////////////////////

pub struct JavaClassInstance { name: String }

impl JavaClassInstance {
    pub fn new(name: String) -> JavaClassInstance {
        JavaClassInstance { name }
    }
}

impl JavaInstance for JavaClassInstance {
    fn get_class_name(&self) -> String {
        return self.name.clone();
    }
}

pub trait JavaClass {
    fn new(&self) -> Rc<RefCell<dyn JavaInstance>> { panic!("Class {} cannot be instantiated", self.get_name()); }
    fn has_static_init(&self) -> bool { false }
    fn get_dependent_classes(&self) -> Vec<String> { Vec::new() }
    fn init_static_fields(&mut self, _classes: &Classes) {}
    fn get_bootstrap_method(&self, _idx: usize) -> Option<&AttributeBootstrapMethod> { return None; }
    fn get_name(&self) -> String;
    fn print(&self) { }
    fn execute_method(&self, sf: &mut StackFrame, classes: &Classes, method_name: &String, this: Rc<RefCell<dyn JavaInstance>>, args: Vec<Rc<RefCell<dyn JavaInstance>>>) {
        if sf.debug >= 1 { println!("Execute native method {}.{}(<{} arguments>)", self.get_name(), method_name, args.len()); }

        let mut object = this.clone();
        let expected_class = self.get_name();
        let mut this_class = this.borrow().get_class_name();
        while !expected_class.eq(&this_class) && !object.borrow().supports_interface(&self.get_name()) {
            let new_obj = match &object.borrow().get_parent() {
                Some(parent) => parent.clone(),
                _ => panic!("Native class {} does not support interface {}", this_class, expected_class)
            };
            object = new_obj.clone();
            this_class = object.borrow().get_class_name();
        }
        object.borrow_mut().execute_method(sf, classes, method_name, object.clone(), args);
    }
    fn execute_static_method(&self, _sf: &mut StackFrame, _classes: &Classes, method_name: &String, _nb_args: usize) {
        panic!("Class {} does not support static method {}", self.get_name(), method_name);
    }
    fn get_static_object(&self, field_name: &String) -> Rc<RefCell<dyn JavaInstance>> {
        panic!("Class {} does not have static field {}", self.get_name(), field_name);
    }
    fn put_static_object(&self, field_name: &String, _value: Rc<RefCell<dyn JavaInstance>>) {
        panic!("Class {} does not have static field {} to update", self.get_name(), field_name);
    }
    fn get_method_handles(&self) -> &HashMap<usize, ConstantMethodHandle> {
        panic!("Class {} has no get_method_handles() implemented", self.get_name());
    }
}

////////////////////////////////////////////////

pub struct BytecodeInstance {
    pub class_name: String,
    pub parent: Option<Rc<RefCell<dyn JavaInstance>>>,
    pub fields: HashMap<String, Rc<RefCell<dyn JavaInstance>>>
}

impl JavaInstance for BytecodeInstance {
    fn is_bytecode(&self) -> bool { return true; }
    fn get_class_name(&self) -> String {
        return self.class_name.clone();
    }
    fn get_parent(&self) -> Option<Rc<RefCell<dyn JavaInstance>>> {
        match &self.parent {
            Some(p) => Some(p.clone()),
            None => None
        }
    }
    fn print(&self) {
        print!("<{} bytecode instance>", self.get_class_name());
    }

    fn get_field(&self, field_name: &String) -> Rc<RefCell<dyn JavaInstance>> {
        return match self.fields.get(field_name) {
            Some(value) => value.clone(),
            _ => panic!("{} instance does not have field {}", self.get_class_name(), field_name)
        };
    }

    fn set_field(&mut self, field_name: &String, value: Rc<RefCell<dyn JavaInstance>>) {
        self.fields.insert(field_name.clone(), value);
    }
}


