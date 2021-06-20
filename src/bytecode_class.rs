use std::fs;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::CLASSES;
use crate::bytecode::InstrNextAction;
use crate::get_debug;
use crate::java_class::JavaClass;
use crate::jvm::Classes;
use crate::jvm::StackFrame;
use crate::native_java_classes::NativeObjectInstance;
use crate::{bytecode::ByteCode, jvm::JavaInstance, native_java_classes::NativeArrayInstance};
use crate::java_class::BytecodeInstance;

pub struct BytecodeClass {
    pub name: String,
    superclass_name: String,
    pub constants_class: HashMap<usize, ConstantClass>,
    constants_string: HashMap<usize, ConstantString>,
    constants_string_ref: HashMap<usize, ConstantStringRef>,
    constants_method: HashMap<usize, ConstantMethod>,
    constants_field: HashMap<usize, ConstantField>,
    constants_name_type: HashMap<usize, ConstantNameType>,
    constants_method_handle: HashMap<usize, ConstantMethodHandle>,
    constants_dynamic: HashMap<usize, ConstantInvokeDynamic>,
    methods: Rc<HashMap<String, ByteCode>>,
    pub bootstrap_methods: Vec<AttributeBootstrapMethod>,
    static_fields: Rc<RefCell<HashMap<String, Rc<RefCell<dyn JavaInstance>>>>>,
    static_fields_desc: HashMap<String, String>,
    has_static_init: bool
}

impl JavaClass for BytecodeClass {
    fn new(&self) -> Rc<RefCell<dyn JavaInstance>> {
        unsafe {
            let superclass = CLASSES.get(&self.superclass_name);
            let parent = superclass.borrow().new();
            return Rc::new(RefCell::new(BytecodeInstance { class_name: self.get_name(), parent: Some(parent), fields: HashMap::new() }));
        }
    }

    fn init_static_fields(&mut self, classes: &Classes) {
        for (field_name, class_name) in self.static_fields_desc.iter() {
            if class_name.eq(&self.name) {
                self.static_fields.borrow_mut().insert(field_name.clone(), self.new());
            } else {
                let class = classes.get_class(&class_name);
                let the_class = class.borrow();
                self.static_fields.borrow_mut().insert(field_name.clone(), the_class.new());
            }
        }
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn get_bootstrap_method(&self, idx: usize) -> Option<&AttributeBootstrapMethod> {
        return self.bootstrap_methods.get(idx);
    }

    fn get_method_handles(&self) -> &HashMap<usize, ConstantMethodHandle> {
        return &self.constants_method_handle;
    }

    fn get_dependent_classes(&self) -> Vec<String> {
        let mut class_names: Vec<String> = Vec::new();

        for constant_class in self.constants_class.values() {
            class_names.push(constant_class.name.clone());
        }

        class_names
    }

    fn has_static_init(&self) -> bool { self.has_static_init }

    fn print(&self) {
        for (_, constant_class) in &self.constants_class {
            constant_class.print();
        }
        for (_, constant_method) in &self.constants_method {
            constant_method.print();
        }
        for (_, constant_field) in &self.constants_field {
            constant_field.print();
        }
        for (_, constant_name_type) in &self.constants_name_type {
            constant_name_type.print();
        }
        for (_, constant_string) in &self.constants_string {
            constant_string.print();
        }
        for (_, constant_string_ref) in &self.constants_string_ref {
            constant_string_ref.print();
        }
        for (_, constant_method_handle) in &self.constants_method_handle {
            constant_method_handle.print();
        }
        for (_, constant_dynamic) in &self.constants_dynamic {
            constant_dynamic.print();
        }
        for bootstrap_method in &self.bootstrap_methods {
            bootstrap_method.print();
        }
    }

    fn execute_method(&self, sf: &mut StackFrame, classes: &Classes, method_name: &String, this: Rc<RefCell<dyn JavaInstance>>, args: Vec<Rc<RefCell<dyn JavaInstance>>>) {
        if self.methods.contains_key(method_name) {
            if get_debug() >= 1 { println!("Execute bytecode method {}.{}(<{} arguments>)", self.get_name(), method_name, args.len()); }

            let var = Rc::new(RefCell::new(NativeObjectInstance {}));
            let mut variables: [Rc<RefCell<dyn JavaInstance>>; 16] = [var.clone(), var.clone(), var.clone(), var.clone(),
                var.clone(), var.clone(), var.clone(), var.clone(),
                var.clone(), var.clone(), var.clone(), var.clone(),
                var.clone(), var.clone(), var.clone(), var.clone()];

            let mut i = args.len() + 1;

            for arg in args.iter() {
                i -= 1;
                variables[i] = arg.clone();
            }
            variables[0] = this.clone();

            let mut sf_new = StackFrame::new(variables, sf.debug);

            self.execute_bytecode(&mut sf_new, classes, method_name);
            if sf_new.return_arg {
                sf.push(sf_new.pop());
            }
        } else {
            let superclass = classes.get_class(&self.superclass_name);
            if get_debug() >= 1 { println!("Execute bytecode method {}.{}(<{} arguments>)", superclass.borrow().get_name(), method_name, args.len()); }

            let parent = this.borrow().get_parent();
            match parent {
                Some(p) => superclass.borrow().execute_method(sf, classes, method_name, p, args),
                _ => panic!("Bytecode instance {} does not support method {}", self.get_name(), method_name)
            };
            
        }
    }

    fn execute_static_method(&self, sf: &mut StackFrame, classes: &Classes, method_name: &String, nb_args: usize) {
        if self.methods.contains_key(method_name) {
            if get_debug() >= 1 { println!("Execute static method {}.{}(<{} arguments>)", self.get_name(), method_name, nb_args); }

            let var = Rc::new(RefCell::new(NativeObjectInstance {}));
            let mut variables: [Rc<RefCell<dyn JavaInstance>>; 16] = [var.clone(), var.clone(), var.clone(), var.clone(),
                var.clone(), var.clone(), var.clone(), var.clone(),
                var.clone(), var.clone(), var.clone(), var.clone(),
                var.clone(), var.clone(), var.clone(), var.clone()];

            for i in 0..nb_args {
                variables[nb_args - 1 - i] = sf.pop();
            }

            let mut sf_new = StackFrame::new(variables, sf.debug);

            self.execute_bytecode(&mut sf_new, classes, method_name);
            if sf_new.return_arg {
                sf.push(sf_new.pop());
            }
        } else {
            let superclass = classes.get_class(&self.superclass_name);
            if get_debug() >= 1 { println!("Execute static method {}.{}(<{} arguments>)", superclass.borrow().get_name(), method_name, nb_args); }

            superclass.borrow().execute_static_method(sf, classes, method_name, nb_args);
        }
    }

    fn get_static_object(&self, field_name: &String) -> Rc<RefCell<dyn JavaInstance>> {
        return self.static_fields.borrow().get(field_name).unwrap().clone();
    }

    fn put_static_object(&self, field_name: &String, value: Rc<RefCell<dyn JavaInstance>>) {
        self.static_fields.borrow_mut().insert(field_name.clone(), value.clone());
    }
}

impl BytecodeClass {
    pub fn parse (name: &String) -> BytecodeClass {
        let mut data = Blob::new(&(name.to_owned() + &String::from(".class")));
        if get_debug() >= 3 { data.print(); }
        data.skip(8);

        let constant_pool_count: usize = data.get_u16size();

        let mut constant_idx: usize = 1;
        let mut opcode;

        let mut constants_class: HashMap<usize, ConstantClass> = HashMap::new();
        let mut constants_string: HashMap<usize, ConstantString> = HashMap::new();
        let mut constants_string_ref: HashMap<usize, ConstantStringRef> = HashMap::new();
        let mut constants_method: HashMap<usize, ConstantMethod> = HashMap::new();
        let mut constants_field: HashMap<usize, ConstantField> = HashMap::new();
        let mut constants_name_type: HashMap<usize, ConstantNameType> = HashMap::new();
        let mut constants_method_handle: HashMap<usize, ConstantMethodHandle> = HashMap::new();
        let mut constants_dynamic: HashMap<usize, ConstantInvokeDynamic> = HashMap::new();
        let mut constants_integer: HashMap<usize, ConstantInteger> = HashMap::new();
        let mut constants_long: HashMap<usize, ConstantLong> = HashMap::new();
        let mut constants_float: HashMap<usize, ConstantFloat> = HashMap::new();
        let mut constants_double: HashMap<usize, ConstantDouble> = HashMap::new();
        let mut has_static_init = false;

        while constant_idx < constant_pool_count {
            opcode = data.get_u8();

            match opcode {
                // CONSTANT_Utf8
                1 => {
                    let constant_string = ConstantString::new(&mut data);
                    if get_debug() >= 2 { constant_string.print(); }
                    constants_string.insert(constant_idx, constant_string);
                },
                // CONSTANT_Integer
                3 => {
                    let constant_integer = ConstantInteger::new(&mut data);
                    if get_debug() >= 2 { constant_integer.print(); }
                    constants_integer.insert(constant_idx, constant_integer);
                },
                // CONSTANT_Float
                4 => {
                    let constant_float = ConstantFloat::new(&mut data);
                    if get_debug() >= 2 { constant_float.print(); }
                    constants_float.insert(constant_idx, constant_float);
                },
                // CONSTANT_Long
                5 => {
                    let constant_long = ConstantLong::new(&mut data);
                    if get_debug() >= 2 { constant_long.print(); }
                    constants_long.insert(constant_idx, constant_long);
                },
                // CONSTANT_Double
                6 => {
                    let constant_double = ConstantDouble::new(&mut data);
                    if get_debug() >= 2 { constant_double.print(); }
                    constants_double.insert(constant_idx, constant_double);
                },
                // CONSTANT_Class
                7 => {
                    let constant_class = ConstantClass::new(&mut data);
                    if get_debug() >= 2 { constant_class.print(); }
                    constants_class.insert(constant_idx, constant_class);
                },
                // CONSTANT_String
                8 => {
                    let constant_string_ref = ConstantStringRef::new(&mut data);
                    if get_debug() >= 2 { constant_string_ref.print(); }
                    constants_string_ref.insert(constant_idx, constant_string_ref);
                },
                // CONSTANT_Fieldref
                9 => {
                    let constant_field = ConstantField::new(&mut data);
                    if get_debug() >= 2 { constant_field.print(); }
                    constants_field.insert(constant_idx, constant_field);
                },
                // CONSTANT_Methodref
                10 => {
                    let constant_method = ConstantMethod::new(&mut data);
                    if get_debug() >= 2 { constant_method.print(); }
                    constants_method.insert(constant_idx, constant_method);
                },
                // CONSTANT_InterfaceMethodref
                11 => {
                    let constant_method = ConstantMethod::new(&mut data);
                    if get_debug() >= 2 { constant_method.print(); }
                    constants_method.insert(constant_idx, constant_method);
                }
                // CONSTANT_NameAndType
                12 => {
                    let constant_name_type = ConstantNameType::new(&mut data);
                    if get_debug() >= 2 { constant_name_type.print(); }
                    constants_name_type.insert(constant_idx, constant_name_type);
                },
                // CONSTANT_MethodHandle
                15 => {
                    let constant_method_handle = ConstantMethodHandle::new(&mut data);
                    if get_debug() >= 2 { constant_method_handle.print(); }
                    constants_method_handle.insert(constant_idx, constant_method_handle);
                },
                // CONSTANT_MethodType
                16 => {
                    data.get_u16size();
//                    println!("CONSTANT_MethodType {}", a);
                },
                // CONSTANT_InvokeDynamic
                18 => {
                    let constant_dynamic = ConstantInvokeDynamic::new(&mut data);
                    if get_debug() >= 2 { constant_dynamic.print(); }
                    constants_dynamic.insert(constant_idx, constant_dynamic);
                },
                _ => panic!("Unknown constant code {} ({:#02x}) at offset {:#x}", opcode, opcode, data.offset)
            };

            constant_idx += 1;
        }

        for (_, constant_class) in constants_class.iter_mut() {
            constant_class.init(&constants_string);
        }

        for (_, constant_string_ref) in constants_string_ref.iter_mut() {
            constant_string_ref.init(&constants_string);
        }

        for (_, constant_name_type) in constants_name_type.iter_mut() {
            constant_name_type.init(&constants_string);
        }

        for (_, constant_method) in constants_method.iter_mut() {
            constant_method.init(&constants_class, &constants_name_type);
        }

        for (_, constant_field) in constants_field.iter_mut() {
            constant_field.init(&constants_class, &constants_name_type);
        }

        for (_, constant_method_handle) in constants_method_handle.iter_mut() {
            constant_method_handle.init(&constants_field, &constants_method);
        }

        for (_, constant_dynamic) in constants_dynamic.iter_mut() {
            constant_dynamic.init1(&constants_name_type);
        }

        // skip access flags
        data.skip(2);

        // main class
        let class_idx = data.get_u16size();
        let constant_class = match constants_class.get(&class_idx) {
            Some(class) => class,
            _ => panic!("Unknown class ID {}", class_idx)
        };
        if get_debug() >= 2 { println!("Class {}", constant_class.name); }

        // super class
        let super_class_idx = data.get_u16size();
        let superclass_name = match constants_class.get(&super_class_idx) {
            Some(class) => class.name.clone(),
            _ => panic!("Unknown class ID {}", super_class_idx)
        };
        if get_debug() >= 2 { println!("Super Class {}", superclass_name); }

        // interfaces_count
        let _interfaces_count = data.get_u16size();

        let static_fields: Rc<RefCell<HashMap<String, Rc<RefCell<dyn JavaInstance>>>>> = Rc::new(RefCell::new(HashMap::new()));
        let mut static_fields_desc: HashMap<String, String> = HashMap::new();

        // fields_count
        let fields_count = data.get_u16size();
        if get_debug() >= 2 { println!("Fields: {}", fields_count); }
        for _ in 0..fields_count {
            let field_access_flag = data.get_u16size();
            let field_idx = data.get_u16size();
            let field_name = match constants_string.get(&field_idx) {
                Some(string) => string.value.clone(),
                _ => panic!("Unknown string ID {}", field_idx)
            };
            let field_descriptor_idx = data.get_u16size();
            let stat = if (field_access_flag & 8) == 8 { "  STATIC" } else { "" };
            if get_debug() >= 2 { println!("Field [{}], type={}, access flag={} {}", field_name, field_descriptor_idx, field_access_flag, stat); }
            if (field_access_flag & 8) == 8 {
                match constants_string.get(&field_descriptor_idx) {
                    Some(string) => {
                        if string.value.starts_with("L") && string.value.ends_with(";") {
                            static_fields_desc.insert(field_name, string.value[1..string.value.len()-1].to_string());
                        } else if string.value.starts_with("[") {
                            static_fields.borrow_mut().insert(field_name, Rc::new(RefCell::new(NativeArrayInstance { values: Rc::new(RefCell::new(Vec::new())) })));
                        }
                    },
                    _ => panic!("Unknown string index {}", field_descriptor_idx)
                };
            }

            let attributes_count = data.get_u16size();
            for _ in 0..attributes_count {
                let attribute_name_idx = data.get_u16size();
                let attribute_size = data.get_u32size();

                let attribute_name = match constants_string.get(&attribute_name_idx) {
                    Some(string) => string.value.clone(),
                    _ => panic!("Unknown string ID {}", attribute_name_idx)
                };
                if get_debug() >= 2 { println!("    Field attribute {} (size: {})", attribute_name, attribute_size); }
                data.skip(attribute_size);
            }
        }

        // methods_count
        let methods_count = data.get_u16size();
        let mut methods: HashMap<String, ByteCode> = HashMap::new();

        for _ in 0..methods_count {
            let _method_access_flag = data.get_u16size();

            let method_idx = data.get_u16size();
            let method_name = match constants_string.get(&method_idx) {
                Some(string) => string.value.clone(),
                _ => panic!("Unknown string ID {}", method_idx)
            };
            if get_debug() >= 2 { println!("Method {}", method_name); }
            if method_name.eq("<clinit>") { has_static_init = true; }

            let descriptor_idx = data.get_u16size();
            let descriptor_name = match constants_string.get(&descriptor_idx) {
                Some(string) => string.value.clone(),
                _ => panic!("Unknown string ID {}", descriptor_idx)
            };
            if get_debug() >= 2 { println!("  Descriptor {}", descriptor_name); }

            let attributes_count = data.get_u16size();
            for _ in 0..attributes_count {
                let attribute_name_idx = data.get_u16size();
                let attribute_size = data.get_u32size();

                let attribute_name = match constants_string.get(&attribute_name_idx) {
                    Some(string) => string.value.clone(),
                    _ => panic!("Unknown string ID {}", attribute_name_idx)
                };
                if get_debug() >= 2 { println!("    Method attribute {} (size: {})", attribute_name, attribute_size); }

                if attribute_name.eq("Code") {
                    data.skip(4);
                    let mut code = data.get_blob();
                    let code_size = code.data.len();
                    if get_debug() >= 2 {
                        print!("    Code: ");
                        for _attributes_count in 0..code_size {
                            print!(" {:02x}", code.get_u8());
                        }
                        println!();
                    }

                    let bytecode = ByteCode::new(&mut code, &constants_class, &constants_string,
                        &constants_string_ref, &constants_method, &constants_field, &constants_name_type,
                        &constants_dynamic, &constants_integer, &constants_long, &constants_float, &constants_double,
                        &constant_class.name);
                    methods.insert(method_name.clone(), bytecode);

                    data.skip(attribute_size - 8 - code_size);
                } else {
                    data.skip(attribute_size);
                }
            }
        }

        // attributes_count
        let attributes_count = data.get_u16size();
        let mut bootstrap_methods: Vec<AttributeBootstrapMethod> = Vec::new();

        for _ in 0..attributes_count {
            let attribute_idx = data.get_u16size();
            let attribute_size = data.get_u32size();
            let attribute_name = match constants_string.get(&attribute_idx) {
                Some(string) => string.value.clone(),
                _ => panic!("Cannot find Constant String at index {}", attribute_idx)
            };
            if get_debug() >= 2 {
                println!("Class attribute name [{}], size [0x{:x}]", attribute_name, attribute_size);
            }
            if attribute_name.eq("BootstrapMethods") {
                let bootstrap_methods_count = data.get_u16size();
                for _ in 0..bootstrap_methods_count {
                    let bootstrap = AttributeBootstrapMethod::new(&mut data, &constants_method_handle);
                    bootstrap_methods.push(bootstrap);
                }
            } else {
                data.skip(attribute_size);
            }
        }
        
        // Update dynamic invokes
        for (_, constant_dynamic) in constants_dynamic.iter_mut() {
            constant_dynamic.init2(&bootstrap_methods);
        }

        BytecodeClass {
            name: constant_class.name.clone(),
            superclass_name,
            constants_class,
            constants_string,
            constants_string_ref,
            constants_method,
            constants_field,
            constants_name_type,
            constants_method_handle,
            constants_dynamic,
            bootstrap_methods,
            methods: Rc::new(methods),
            static_fields,
            static_fields_desc,
            has_static_init
        }
    }

    fn execute_bytecode(&self, sf: &mut StackFrame, classes: &Classes, method_name: &String) {
        let bytecode = match self.methods.get(method_name) {
            Some(method) => method,
            _ => panic!("Unknown method {} in class {}", method_name, self.name)
        };

        let mut instr_idx: usize = 0;

        loop {
            match bytecode.instructions.get(instr_idx) {
                Some(instr) => {
                    if get_debug() >= 1 {
                        print!("Execute {} ", instr_idx);
                        instr.print();
                    }
                    match instr.execute(sf, classes) {
                        InstrNextAction::NEXT => {
                            instr_idx += 1;
                        },
                        InstrNextAction::GOTO(idx) => {
                            instr_idx = idx;
                        }
                        InstrNextAction::RETURN => {
                            if get_debug() >= 1 { sf.print_stack(); }
                            return;
                        }
                    }
                },
                _ => panic!("No instruction {}", instr_idx)
            }
        }
    }

}

pub struct Blob {
    offset: usize,
    data: Vec<u8>
}

impl Blob {
    pub fn new (name: &str) -> Blob {
        let mut filename = &name.to_string();
        let mut new_path: String = String::from("java/");
        new_path.push_str(filename);
        let filename2 = new_path.clone();

        if !Path::new(filename).exists() {
            if !Path::new(&new_path).exists() {
                panic!("Cannot find {}", name);
            }

            filename = &filename2;
        }

        let mut f = File::open(filename).expect("no file found");
        let metadata = fs::metadata(filename).expect("unable to read metadata");
        let mut data = vec![0; metadata.len() as usize];
        f.read(&mut data).expect("buffer overflow");

        Blob {
            offset: 0,
            data: data
        }
    }

    pub fn get_offset(&self) -> usize {
        return self.offset;
    }

    pub fn print(&self) {
        let num_rows = self.data.len() / 16;

        for i in 0..num_rows+1 {
            print!("{:#06x}    ", i * 16);
            let num_cols = if (i+1) * 16 > self.data.len() { self.data.len() - i*16 } else { 16 };
            for j in 0..num_cols {
                print!("  {:02x}", self.data[i*16 + j]);
            }
            println!();
        }

        println!();
    }

    pub fn get_u8(&mut self) -> u8 {
        let offset = self.offset;
        self.offset += 1;
        return self.data[offset];
    }

    pub fn get_i8(&mut self) -> i8 {
        let offset = self.offset;
        self.offset += 1;
        return self.data[offset] as i8;
    }

    pub fn get_i16(&mut self) -> i16 {
        let offset = self.offset;
        let array: [u8; 2] = [self.data[offset+1], self.data[offset]];
        self.offset += 2;
        return i16::from_ne_bytes(array);
    }

    pub fn get_i32(&mut self) -> i32 {
        let offset = self.offset;
        let array: [u8; 4] = [self.data[offset+3], self.data[offset+2], self.data[offset+1], self.data[offset]];
        self.offset += 4;
        return i32::from_ne_bytes(array);
    }

    pub fn get_i64(&mut self) -> i64 {
        let offset = self.offset;
        let array: [u8; 8] = [self.data[offset+7], self.data[offset+6], self.data[offset+5], self.data[offset+4],
            self.data[offset+3], self.data[offset+2], self.data[offset+1], self.data[offset]];
        self.offset += 8;
        return i64::from_ne_bytes(array);
    }

    pub fn get_u16size(&mut self) -> usize {
        let offset = self.offset;
        self.offset += 2;
        return usize::from(self.data[offset]) * 256 + usize::from(self.data[offset+1]);
    }
    
    pub fn get_u32size(&mut self) -> usize {
        let offset = self.offset;
        self.offset += 4;
        return usize::from(self.data[offset]) * 16777216
            + usize::from(self.data[offset+1]) * 65536
            + usize::from(self.data[offset+2]) * 256
            + usize::from(self.data[offset+3]);
    }

    pub fn get_f32(&mut self) -> f32 {
        let offset = self.offset;
        let array: [u8; 4] = [self.data[offset+3], self.data[offset+2], self.data[offset+1], self.data[offset]];
        self.offset += 4;
        return f32::from_ne_bytes(array);
    }

    pub fn get_f64(&mut self) -> f64 {
        let offset = self.offset;
        let array: [u8; 8] = [self.data[offset+7], self.data[offset+6], self.data[offset+5], self.data[offset+4],
            self.data[offset+3], self.data[offset+2], self.data[offset+1], self.data[offset]];
        self.offset += 8;
        return f64::from_ne_bytes(array);
    }

    pub fn rewind(&mut self) {
        self.offset = 0;
    }

    pub fn skip(&mut self, nb: usize) {
        self.offset += nb;
    }

    pub fn get_string(&mut self) -> String {
        let offset = self.offset;
        let size = self.get_u16size();
        self.offset += size;
        return String::from_utf8_lossy(&self.data[offset + 2..offset+size+2]).to_string();
    }

    pub fn get_blob(&mut self) -> Blob {
        let size = self.get_u32size();
        let offset = self.offset;
        self.offset += size;

        return Blob {
            offset: 0,
            data: self.data[offset..offset+size].to_vec()
        };
    }

    pub fn has_more_data(&self) -> bool {
        return self.offset < self.  data.len();
    }
}

///////////////////////////////////////////
pub struct ConstantString {
    size: usize,
    pub value: String
}

impl ConstantString {
    pub fn new (data: &mut Blob) -> ConstantString {
        let string_content = data.get_string();
        let size = string_content.len();

        ConstantString {
            size: size,
            value: string_content
        }
    }

    pub fn print(&self) {
        println!("String: [{}], {} bytes", self.value, self.size);
    }
}

///////////////////////////////////////////
pub struct ConstantMethod {
    idx_class: usize,
    idx_name_type: usize,
    pub class_name: String,
    pub method_name: String,
    pub type_name: String
}

impl ConstantMethod {
    pub fn new (data: &mut Blob) -> ConstantMethod {
        let idx_class = data.get_u16size();
        let idx_name_type = data.get_u16size();

        ConstantMethod {
            idx_class: idx_class,
            idx_name_type: idx_name_type,
            class_name: "".to_string(),
            method_name: "".to_string(),
            type_name: "".to_string()
        }
    }

    pub fn init(&mut self, classes: &HashMap<usize, ConstantClass>, name_types: &HashMap<usize, ConstantNameType>) {
        self.class_name = match classes.get(&self.idx_class) {
            Some(class) => class.name.clone(),
            _ => "n/a".to_string()
        };
        match name_types.get(&self.idx_name_type) {
            Some(name_type) => {
                self.method_name = name_type.name.clone();
                self.type_name = name_type.type_desc.clone();
            }
            _ => {}
        };
    }

    pub fn print(&self) {
        println!("Method: [{}], Class: [{}], Type: [{}]", self.method_name, self.class_name, self.type_name);
    }
}

///////////////////////////////////////////
pub struct ConstantField {
    idx_class: usize,
    idx_name_type: usize,
    pub class_name: String,
    pub field_name: String,
    pub type_name: String
}

impl ConstantField {
    pub fn new (data: &mut Blob) -> ConstantField {
        let idx_class = data.get_u16size();
        let idx_name_type = data.get_u16size();

        ConstantField {
            idx_class: idx_class,
            idx_name_type: idx_name_type,
            class_name: "".to_string(),
            field_name: "".to_string(),
            type_name: "".to_string()
        }
    }

    pub fn init(&mut self, classes: &HashMap<usize, ConstantClass>, name_types: &HashMap<usize, ConstantNameType>) {
        self.class_name = match classes.get(&self.idx_class) {
            Some(class) => class.name.clone(),
            _ => "n/a".to_string()
        };
        match name_types.get(&self.idx_name_type) {
            Some(name_type) => {
                self.field_name = name_type.name.clone();
                self.type_name = name_type.type_desc.clone();
            }
            _ => {}
        };
    }

    pub fn print(&self) {
        println!("Field: [{}], Class: [{}], Type: [{}]", self.field_name, self.class_name, self.type_name);
    }
}

///////////////////////////////////////////
pub struct ConstantStringRef {
    idx: usize,
    pub value: String
}

impl ConstantStringRef {
    pub fn new (data: &mut Blob) -> ConstantStringRef {
        ConstantStringRef {
            idx: data.get_u16size(),
            value: "".to_string()
        }
    }

    pub fn init(&mut self, strings: &HashMap<usize, ConstantString>) {
        self.value = match strings.get(&self.idx) {
            Some(string) => string.value.clone(),
            _ => "n/a".to_string()
        }
    }

    pub fn print(&self) {
        println!("String ref: [{}]", self.value);
    }
}

///////////////////////////////////////////
pub struct ConstantClass {
    idx: usize,
    pub name: String
}

impl ConstantClass {
    pub fn new (data: &mut Blob) -> ConstantClass {
        ConstantClass {
            idx: data.get_u16size(),
            name: "".to_string()
        }
    }

    pub fn init(&mut self, strings: &HashMap<usize, ConstantString>) {
        self.name = match strings.get(&self.idx) {
            Some(string) => string.value.clone(),
            _ => "n/a".to_string()
        }
    }

    pub fn print(&self) {
        println!("Class: [{}]", self.name);
    }    
}

///////////////////////////////////////////
pub struct ConstantNameType {
    idx_name: usize,
    idx_type: usize,
    pub name: String,
    pub type_desc: String
}

impl ConstantNameType {
    pub fn new (data: &mut Blob) -> ConstantNameType {
        let idx_class = data.get_u16size();
        let idx_type = data.get_u16size();

        ConstantNameType {
            idx_name: idx_class,
            idx_type: idx_type,
            name: "".to_string(),
            type_desc: "".to_string()
        }
    }

    pub fn init(&mut self, strings: &HashMap<usize, ConstantString>) {
        self.name = match strings.get(&self.idx_name) {
            Some(string) => string.value.clone(),
            _ => "n/a".to_string()
        };
        self.type_desc = match strings.get(&self.idx_type) {
            Some(string) => string.value.clone(),
            _ => "n/a".to_string()
        };
    }

    pub fn print(&self) {
        println!("Name/type: [{}][{}]", self.name, self.type_desc);
    }    
}

///////////////////////////////////////////
/*
pub struct ConstantInterfaceMethod {
    idx_class: usize,
    idx_name_type: usize,
    pub class_name: String,
    pub method_name: String,
    pub type_name: String
}

impl ConstantInterfaceMethod {
    pub fn new (data: &mut Blob) -> ConstantInterfaceMethod {
        let idx_class = data.get_u16size();
        let idx_name_type = data.get_u16size();

        ConstantInterfaceMethod {
            idx_class: idx_class,
            idx_name_type: idx_name_type,
            class_name: "".to_string(),
            method_name: "".to_string(),
            type_name: "".to_string()
        }
    }

    pub fn init(&mut self, classes: &HashMap<usize, ConstantClass>, name_types: &HashMap<usize, ConstantNameType>) {
        self.class_name = match classes.get(&self.idx_class) {
            Some(class) => class.name.clone(),
            _ => "n/a".to_string()
        };
        match name_types.get(&self.idx_name_type) {
            Some(name_type) => {
                self.method_name = name_type.name.clone();
                self.type_name = name_type.type_desc.clone();
            }
            _ => {}
        };
    }

    pub fn print(&self) {
        println!("Interface method: [{}], Class: [{}], Type: [{}]", self.method_name, self.class_name, self.type_name);
    }
}
*/
///////////////////////////////////////////
pub struct ConstantInvokeDynamic {
    pub idx_bootstrap_method: usize,
    idx_name_type: usize,
    bootstrap_class_name: String,
    bootstrap_method_name: String,
    bootstrap_type_name: String,
    bootstrap_arguments: Vec<usize>,
    pub class_name: String,
    pub method_name: String,
    pub type_name: String
}

impl ConstantInvokeDynamic {
    pub fn new (data: &mut Blob) -> ConstantInvokeDynamic {
        let idx_bootstrap_method = data.get_u16size();
        let idx_name_type = data.get_u16size();

        ConstantInvokeDynamic {
            idx_bootstrap_method: idx_bootstrap_method,
            idx_name_type: idx_name_type,
            bootstrap_class_name: "".to_string(),
            bootstrap_method_name: "".to_string(),
            bootstrap_type_name: "".to_string(),
            bootstrap_arguments: Vec::new(),
            class_name: "".to_string(),
            method_name: "".to_string(),
            type_name: "".to_string()
        }
    }

    pub fn init1(&mut self, name_types: &HashMap<usize, ConstantNameType>) {
        match name_types.get(&self.idx_name_type) {
            Some(name_type) => {
                self.method_name = name_type.name.clone();
                self.type_name = name_type.type_desc.clone();
            },
            _ => panic!("Cannot find name/type {}", self.idx_name_type)
        };
    }

    pub fn init2(&mut self, bootstrap_methods: &Vec<AttributeBootstrapMethod>) {
        match bootstrap_methods.get(self.idx_bootstrap_method) {
            Some(bootstrap) => {
                self.bootstrap_class_name = bootstrap.class_name.clone();
                self.bootstrap_method_name = bootstrap.method_name.clone();
                self.bootstrap_type_name = bootstrap.type_name.clone();
                for arg in &bootstrap.arguments {
                    self.bootstrap_arguments.push(*arg);
                }
            }
            _ => panic!("Cannot find bootstrap method {}", self.idx_bootstrap_method)
        };
    }

    pub fn print(&self) {
        println!("Interface method: [{}], Class: [{}], Type: [{}]", self.method_name, self.class_name, self.type_name);
    }
}

///////////////////////////////////////////
pub struct ConstantMethodHandle {
    reference_kind: u8,
    idx_reference: usize,
    pub class_name: String,
    pub method_name: String,
    pub type_name: String,
    pub field_name: String
}

impl ConstantMethodHandle {
    pub fn new (data: &mut Blob) -> ConstantMethodHandle {
        let reference_kind = data.get_u8();
        let idx_reference = data.get_u16size();

        ConstantMethodHandle {
            reference_kind: reference_kind,
            idx_reference: idx_reference,
            class_name: "".to_string(),
            method_name: "".to_string(),
            type_name: "".to_string(),
            field_name: "".to_string()
        }
    }

    pub fn init(&mut self, constants_field: &HashMap<usize, ConstantField>, constants_method: &HashMap<usize, ConstantMethod>) {
        match self.reference_kind {
            1..=4 => {
                match constants_field.get(&self.idx_reference) {
                    Some(field) => { self.field_name = field.field_name.clone(); },
                    _ => panic!("Unknown constant field {}", &self.idx_reference)
                };
            },
            5..=9 => {
                match constants_method.get(&self.idx_reference) {
                    Some(method) => {
                        self.class_name = method.class_name.clone();
                        self.method_name = method.method_name.clone();
                        self.type_name = method.type_name.clone();
                    },
                    _ => panic!("Unknown constant method {}", self.idx_reference)
                }
            },
            _ => panic!("Unknown Constant MethodHandle reference kind {}", self.reference_kind)
        };
    }

    pub fn print(&self) {
        println!("Method Handle: [{}], Class: [{}], Type: [{}]", self.method_name, self.class_name, self.type_name);
    }
}

///////////////////////////////////////////
pub struct ConstantFloat {
    pub value: f32
}

impl ConstantFloat {
    pub fn new (data: &mut Blob) -> ConstantFloat {
        ConstantFloat {
            value: data.get_f32()
        }
    }

    pub fn print(&self) {
        println!("Float: {}", self.value);
    }
}

pub struct ConstantDouble {
    pub value: f64
}

impl ConstantDouble {
    pub fn new (data: &mut Blob) -> ConstantDouble {
        ConstantDouble {
            value: data.get_f64()
        }
    }

    pub fn print(&self) {
        println!("Double: {}", self.value);
    }
}

pub struct ConstantInteger {
    pub value: i32
}

impl ConstantInteger {
    pub fn new (data: &mut Blob) -> ConstantInteger {
        ConstantInteger {
            value: data.get_i32()
        }
    }

    pub fn print(&self) {
        println!("Integer: {}", self.value);
    }
}

pub struct ConstantLong {
    pub value: i64
}

impl ConstantLong {
    pub fn new (data: &mut Blob) -> ConstantLong {
        ConstantLong {
            value: data.get_i64()
        }
    }

    pub fn print(&self) {
        println!("Long: {}", self.value);
    }
}

///////////////////////////////////////////
pub struct AttributeBootstrapMethod {
    pub class_name: String,
    pub method_name: String,
    pub type_name: String,
    pub arguments: Vec<usize>
}

impl AttributeBootstrapMethod {
    pub fn new (data: &mut Blob, constants_method_handle: &HashMap<usize, ConstantMethodHandle>) -> AttributeBootstrapMethod {
        let idx_reference = data.get_u16size();
        let class_name: String;
        let method_name: String;
        let type_name: String;

        match constants_method_handle.get(&idx_reference) {
            Some(method) => {
                class_name = method.class_name.clone();
                method_name = method.method_name.clone();
                type_name = method.type_name.clone();
            },
            _ => panic!("Cannot find method handle {}", idx_reference)
        };

        let arguments_count = data.get_u16size();
        let mut arguments: Vec<usize> = Vec::new();
        for _ in 0..arguments_count {
            arguments.push(data.get_u16size());
        }

        AttributeBootstrapMethod {
            class_name: class_name,
            method_name: method_name,
            type_name: type_name,
            arguments: arguments
        }
    }

    pub fn print(&self) {
        print!("Bootstrap method, class [{}], method [{}], args:[", self.class_name, self.method_name);
        for arg in &self.arguments {
            print!("{} ", arg);
        }
        println!("]");
    }
}
