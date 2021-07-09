use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::{StackFrame, get_debug};
use crate::bytecode_class::AttributeBootstrapMethod;
use crate::bytecode_class::ConstantMethodHandle;
use crate::jvm::JavaInstance;

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

pub enum MethodCallResult {
    SUCCESS,
    EXCEPTION(Arc<Mutex<dyn JavaInstance>>)
}

pub trait JavaClass {
    fn new(&self) -> Arc<Mutex<dyn JavaInstance>> { panic!("Class {} cannot be instantiated", self.get_name()); }
    fn has_static_init(&self) -> bool { false }
    fn get_dependent_classes(&self) -> Vec<String> { Vec::new() }
    fn get_bootstrap_method(&self, _idx: usize) -> Option<&AttributeBootstrapMethod> { return None; }
    fn convert_to_intel_asm(&self, _method_name: &String) { panic!("Class {} does not support conversion to x64 assembly", self.get_name()); }
    fn get_name(&self) -> String;
    fn print(&self) { }
    fn get_parent(&self) -> String { "".to_string() }
    fn execute_method(&self, sf: &mut StackFrame, method_name: &String, this: Arc<Mutex<dyn JavaInstance>>, args: Vec<Arc<Mutex<dyn JavaInstance>>>) -> MethodCallResult {
        if get_debug() >= 1 { println!("Execute native method {}.{}(<{} arguments>)", self.get_name(), method_name, args.len()); }

        let expected_class = self.get_name();
        let this_class = this.lock().unwrap().get_class_name();
        let this2 = this.clone();
        let object = if expected_class.eq(&this_class) || this.lock().unwrap().supports_interface(&expected_class) {
            this
        } else {
            this.lock().unwrap().cast_as(this2, &expected_class)
        };

        object.lock().unwrap().execute_method(sf, method_name, object.clone(), args);
        return MethodCallResult::SUCCESS;
    }
    fn execute_static_method(&self, _sf: &mut StackFrame, method_name: &String, _nb_args: usize) -> MethodCallResult {
        panic!("Class {} does not support static method {}", self.get_name(), method_name);
    }
    fn get_static_object(&self, field_name: &String) -> Arc<Mutex<dyn JavaInstance>> {
        panic!("Class {} does not have static field {}", self.get_name(), field_name);
    }
    fn put_static_object(&self, field_name: &String, _value: Arc<Mutex<dyn JavaInstance>>) {
        panic!("Class {} does not have static field {} to update", self.get_name(), field_name);
    }
    fn get_method_handles(&self) -> &HashMap<usize, ConstantMethodHandle> {
        panic!("Class {} has no get_method_handles() implemented", self.get_name());
    }
}

////////////////////////////////////////////////

pub struct BytecodeInstance {
    pub class_name: String,
    pub parent: Option<Arc<Mutex<dyn JavaInstance>>>,
    pub fields: HashMap<String, Arc<Mutex<dyn JavaInstance>>>
}

impl JavaInstance for BytecodeInstance {
    fn is_bytecode(&self) -> bool { return true; }
    fn get_class_name(&self) -> String {
        return self.class_name.clone();
    }
    fn cast_as(&self, _this: Arc<Mutex<dyn JavaInstance>>, class_name: &String) -> Arc<Mutex<dyn JavaInstance>> {
        match &self.parent {
            Some(p) => {
                if !p.lock().unwrap().get_class_name().eq(class_name) {
                    panic!("Instance of class {} cannot be cast into {}", self.class_name, class_name);
                }
                p.clone()
            },
            None => panic!("Instance of class {} cannot be cast into {}", self.class_name, class_name)
        }
    }
    fn print(&self) {
        print!("<{} bytecode instance>", self.get_class_name());
    }

    fn get_field(&self, field_name: &String) -> Arc<Mutex<dyn JavaInstance>> {
        return match self.fields.get(field_name) {
            Some(value) => value.clone(),
            _ => panic!("{} instance does not have field {}", self.get_class_name(), field_name)
        };
    }

    fn set_field(&mut self, field_name: &String, value: Arc<Mutex<dyn JavaInstance>>) {
        self.fields.insert(field_name.clone(), value);
    }
}


#[cfg(test)]
mod tests {
    use crate::java_class::get_nb_arguments;

    #[test]
    fn test_get_nb_arguments() {
        assert_eq!(get_nb_arguments(&"(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;".to_string()), 2);
    }
}
