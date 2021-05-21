use std::str;
use std::fs;
use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::rc::Rc;

use crate::JVM;
use crate::bytecode::ByteCode;
use crate::bytecode::InstrNextAction;
use crate::jvm::JavaObject;
use crate::jvm::Classes;

pub struct Blob {
    offset: usize,
    data: Vec<u8>
}

impl Blob {
    pub fn new (name: &str) -> Blob {
        let filename = &name.to_string();
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
        return self.data[offset]
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
            + usize::from(self.data[offset+2]) * 156
            + usize::from(self.data[offset+3]);
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
///////////////////////////////////////////

pub trait JavaClass {
    fn get_name(&self) -> String;
    fn print(&self);
    fn execute_method(&self, jvm: &mut JVM, classes: &Classes, method_name: &String);
    fn execute_static_method(&self, jvm: &mut JVM, classes: &Classes, method_name: &String);
    fn get_static_object(&self, field_name: &String) -> JavaObject;
}

pub struct BytecodeClass {
    name: String,
    constants_class: HashMap<usize, ConstantClass>,
    constants_string: HashMap<usize, ConstantString>,
    constants_string_ref: HashMap<usize, ConstantStringRef>,
    constants_method: HashMap<usize, ConstantMethod>,
    constants_field: HashMap<usize, ConstantField>,
    constants_name_type: HashMap<usize, ConstantNameType>,
    methods: HashMap<String, ByteCode>,
    debug: u8
}

impl BytecodeClass {
    pub fn new (name: &String, debug: u8) -> BytecodeClass {
        let mut data = Blob::new(&(name.to_owned() + &String::from(".class")));
        if debug >= 3 { data.print(); }
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

        while constant_idx < constant_pool_count {
            opcode = data.get_u8();

            match opcode {
                1 => {
                    let constant_string = ConstantString::new(&mut data);
                    constants_string.insert(constant_idx, constant_string);
                },
                7 => {
                    let constant_class = ConstantClass::new(&mut data);
                    constants_class.insert(constant_idx, constant_class);
                },
                8 => {
                    let constant_string_ref = ConstantStringRef::new(&mut data);
                    constants_string_ref.insert(constant_idx, constant_string_ref);
                },
                9 => {
                    let constant_field = ConstantField::new(&mut data);
                    constants_field.insert(constant_idx, constant_field);
                },
                10 => {
                    let constant_method = ConstantMethod::new(&mut data);
                    constants_method.insert(constant_idx, constant_method);
                },
                12 => {
                    let constant_name_type = ConstantNameType::new(&mut data);
                    constants_name_type.insert(constant_idx, constant_name_type);
                },
                _ => panic!("Unknown upcode {} at offset {}", opcode, data.offset)
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

        // skip access flags
        data.skip(2);

        // main class
        let class_idx = data.get_u16size();
        let constant_class = match constants_class.get(&class_idx) {
            Some(class) => class,
            _ => panic!("Unknown class ID {}", class_idx)
        };
        if debug >= 2 { println!("Class {}", constant_class.name); }

        // super class
        let super_class_idx = data.get_u16size();
        let constant_super_class = match constants_class.get(&super_class_idx) {
            Some(class) => class,
            _ => panic!("Unknown class ID {}", super_class_idx)
        };
        if debug >= 2 { println!("Super Class {}", constant_super_class.name); }

        // interface_scount
        let _interfaces_count = data.get_u16size();

        // fields_count
        let _fields_count = data.get_u16size();

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
            if debug >= 2 { println!("Method {}", method_name); }

            let descriptor_idx = data.get_u16size();
            let descriptor_name = match constants_string.get(&descriptor_idx) {
                Some(string) => string.value.clone(),
                _ => panic!("Unknown string ID {}", descriptor_idx)
            };
            if debug >= 2 { println!("  Descriptor {}", descriptor_name); }

            let attributes_count = data.get_u16size();
            for _ in 0..attributes_count {
                let attribute_name_idx = data.get_u16size();
                let attribute_size = data.get_u32size();

                let attribute_name = match constants_string.get(&attribute_name_idx) {
                    Some(string) => string.value.clone(),
                    _ => panic!("Unknown string ID {}", attribute_name_idx)
                };
                if debug >= 2 { println!("    Attribute {} (size: {})", attribute_name, attribute_size); }

                if attribute_name.eq("Code") {
                    data.skip(4);
                    let mut code = data.get_blob();
                    let code_size = code.data.len();
                    if debug >= 2 {
                        print!("    Code: ");
                        for _attributes_count in 0..code_size {
                            print!(" {:02x}", code.get_u8());
                        }
                        println!();
                    }

                    let bytecode = ByteCode::new(&mut code, &constants_class, &constants_string, &constants_string_ref,
                        &constants_method, &constants_field, &constants_name_type, debug);
                    methods.insert(method_name.clone(), bytecode);

                    data.skip(attribute_size - 8 - code_size);
                } else {
                    data.skip(attribute_size);
                }
            }
        }

        // attributes_count
        let _attributes_count = data.get_u16size();
        
        BytecodeClass {
            name: constant_class.name.clone(),
            constants_class: constants_class,
            constants_string: constants_string,
            constants_string_ref: constants_string_ref,
            constants_method: constants_method,
            constants_field: constants_field,
            constants_name_type: constants_name_type,
            methods: methods,
            debug: debug
        }
    }
}

impl JavaClass for BytecodeClass {
    fn get_name(&self) -> String {
        return self.name.clone();
    }

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
    }

    fn execute_method(&self, _jvm: &mut JVM, _classes: &Classes, _method_name: &String) {
        panic!("Not implemented yet");
    }

    fn execute_static_method(&self, jvm: &mut JVM, classes: &Classes, method_name: &String) {
        if self.debug >= 1 { println!("Executing method {}", method_name); }

        let bytecode = match self.methods.get(method_name) {
            Some(method) => method,
            _ => panic!("Unknown method {} in class {}", method_name, self.name)
        };

        let mut instr_idx: usize = 0;

        loop {
            match bytecode.instructions.get(instr_idx) {
                Some(instr) => {
                    if self.debug >= 1 {
                        print!("Execute {} ", instr_idx);
                        instr.print();
                    }
                    match instr.execute(self, jvm, classes) {
                        InstrNextAction::NEXT => {
                            instr_idx += 1;
                        },
                        InstrNextAction::GOTO(idx) => {
                            instr_idx = idx;
                        }
                        InstrNextAction::RETURN => {
                            if self.debug >= 1 { jvm.print_stack(); }
                            return;
                        }
                    }        
                },
                _ => panic!("No instruction {}", instr_idx)
            }
        }
    }

    fn get_static_object(&self, _field_name: &String) -> JavaObject {
        panic!("Not implemented yet");
    }
}

pub struct NativePrintStreamClass { }

impl JavaClass for NativePrintStreamClass {
    fn get_name(&self) -> String {
        return "java/io/PrintStream".to_string();
    }

    fn print(&self) {
        println!("Native Stream class");
    }

    fn execute_method(&self, jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        if !method_name.eq("println") {
            panic!("Native class {} does not have method {}", self.get_name(), method_name);
        }
        let string = jvm.pop();
        let _instance = jvm.pop();

        match &*string {
            JavaObject::STRING(value) => println!("{}", value),
            _ => println!("???")
        };
    }

    fn execute_static_method(&self, _jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        panic!("Native class {} does not have static method {}", self.get_name(), method_name);
    }

    fn get_static_object(&self, field_name: &String) -> JavaObject {
        panic!("Native class {} does not have static field {}", self.get_name(), field_name);
    }
}    

pub struct NativeSystemClass { }

impl JavaClass for NativeSystemClass {
    fn get_name(&self) -> String {
        return "java/lang/System".to_string();
    }

    fn print(&self) {
        println!("Native System class");
    }

    fn execute_method(&self, _jvm: &mut JVM, _classes: &Classes, _method_name: &String) {
        println!("Not implemented yet");
    }

    fn execute_static_method(&self, _jvm: &mut JVM, _classes: &Classes, _method_name: &String) {
        panic!("Not implemented yet");
    }

    fn get_static_object(&self, field_name: &String) -> JavaObject {
        if field_name.eq("out") {
            return JavaObject::INSTANCE(self.get_name().clone());
        }
            
        panic!("Native class {} does not have static field [{}]", self.get_name(), field_name);
    }

}

pub struct NativeIntegerClass { }

impl JavaClass for NativeIntegerClass {
    fn get_name(&self) -> String {
        return "java/lang/Integer".to_string();
    }

    fn print(&self) {
        println!("Native Integer class");
    }

    fn execute_method(&self, _jvm: &mut JVM, _classes: &Classes, _method_name: &String) {
        println!("Not implemented yet");
    }

    fn execute_static_method(&self, jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        if method_name.eq("parseInt") || method_name.eq("valueOf") {
            let int_value = match &*(jvm.pop()) {
                JavaObject::STRING(st) => Rc::new(JavaObject::INTEGER(st.parse::<i32>().unwrap())),
                JavaObject::INTEGER(int) => Rc::new(JavaObject::INTEGER(*int)),
                _ => panic!("Integer.parseInt() not supported for this type")
            };
            jvm.push(int_value);
            return;
        }

        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }

    fn get_static_object(&self, _field_name: &String) -> JavaObject {
        panic!("Not implemented yet");
    }
}

pub struct NativeStringClass { }

impl JavaClass for NativeStringClass {
    fn get_name(&self) -> String {
        return "java/lang/String".to_string();
    }

    fn print(&self) {
        println!("Native Integer class");
    }

    fn execute_method(&self, _jvm: &mut JVM, _classes: &Classes, _method_name: &String) {
        println!("Not implemented yet");
    }

    fn execute_static_method(&self, jvm: &mut JVM, _classes: &Classes, method_name: &String) {
        if method_name.eq("format") {
            let array_arg = jvm.pop();
            let string_arg = jvm.pop();

            let array = match &*array_arg {
                JavaObject::ARRAY(array) => array.borrow(),
                _ => panic!("String.format() expects an array in the stack")
            };

            let mut output = String::new();
            let mut special = false;
            let mut array_idx: usize = 0;

            match &*string_arg {
                JavaObject::STRING(value) => {
                    for c in value.chars() {
                        if special {
                            match c {
                                'd' => {
                                    match array.get(array_idx) {
                                        Some(object) => match **object {
                                            JavaObject::INTEGER(int) => {
                                                output.push_str(&int.to_string());
                                            },
                                            _ => panic!("String.format() expects an integer for argument {}", array_idx + 1)
                                        },
                                        _ => panic!("String.format() does not have enought arguments")
                                    };
                                },
                                's' => {
                                    match array.get(array_idx) {
                                        Some(object) => match &**object {
                                            JavaObject::STRING(str) => {
                                                output.push_str(&str.clone());
                                            },
                                            _ => panic!("String.format() expects a string for argument {}", array_idx + 1)
                                        },
                                        _ => panic!("String.format() does not have enought arguments")
                                    };
                                },
                                _ => panic!("String.format() does not support %{}", c)
                            };
                            special = false;
                            array_idx += 1;
                        } else {
                            match c {
                                '%' => { special = true; },
                                _ => { output.push(c); }
                            };
                        }
                    }
        
                },
                _ => panic!("String.format() expects a string as parameter")
            };

            jvm.push(Rc::new(JavaObject::STRING(output)));
            return;
        }

        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }

    fn get_static_object(&self, field_name: &String) -> JavaObject {
        panic!("Native class {} does not have static object [{}]", self.get_name(), field_name);
    }
}
