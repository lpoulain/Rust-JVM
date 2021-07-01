use core::time;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use std::thread;

use rand::Rng;

use crate::{CLASSES, GLOBAL_THREAD_COUNT, class_exists, get_class};
use crate::StackFrame;
use crate::jvm::JavaInstance;
use crate::java_class::{JavaClass, MethodCallResult};
use crate::streams::NativeStreamClass;
use crate::streams::NativeLambdaMetafactoryClass;
use crate::streams::NativeStreamInstance;

pub fn register_native_classes() {
    unsafe {
        CLASSES.add(Arc::new(NativeObjectClass {}));
        CLASSES.add(Arc::new(NativePrintStreamClass {}));
        CLASSES.add(Arc::new(NativeSystemClass {}));
        CLASSES.add(Arc::new(NativeStringClass {}));
        CLASSES.add(Arc::new(NativeIntegerClass {}));
        CLASSES.add(Arc::new(NativeArraysClass {}));
        CLASSES.add(Arc::new(NativeListClass {}));
        CLASSES.add(Arc::new(NativeArrayListClass {}));
        CLASSES.add(Arc::new(NativeStreamClass {}));
        CLASSES.add(Arc::new(NativeMathClass {}));
        CLASSES.add(Arc::new(NativeLambdaMetafactoryClass {}));
        CLASSES.add(Arc::new(NativeEnumClass {}));
        CLASSES.add(Arc::new(NativeNoSuchFieldErrorClass {}));
        CLASSES.add(Arc::new(NativeMethodHandlesLookupClass {}));
        CLASSES.add(Arc::new(NativeMethodHandlesClass {}));
        CLASSES.add(Arc::new(NativeStringBuilderClass {}));
        CLASSES.add(Arc::new(NativeThreadClass {}));
        CLASSES.add(Arc::new(NativeGenericExceptionClass { name: "java/lang/Throwable".to_string(), parent: "".to_string() }));
        CLASSES.add(Arc::new(NativeGenericExceptionClass { name: "java/lang/Exception".to_string(), parent: "java/lang/Throwable".to_string() }));
        CLASSES.add(Arc::new(NativeGenericExceptionClass { name: "java/lang/RuntimeException".to_string(), parent: "java/lang/Exception".to_string() }));
        CLASSES.add(Arc::new(NativeGenericExceptionClass { name: "java/lang/ArithmeticException".to_string(), parent: "java/lang/RuntimeException".to_string() }));
    }
}

//////////

pub struct NativeObjectInstance {}
impl JavaInstance for NativeObjectInstance {
    fn get_class_name(&self) -> String {
        return "java/lang/Object".to_string();
    }
    fn execute_method(&mut self, _sf: &mut StackFrame, method_name: &String, _this: Arc<Mutex<dyn JavaInstance>>, _args: Vec<Arc<Mutex<dyn JavaInstance>>>) {
        match &method_name[..] {
            "<init>" => { },
            _ => panic!("Class {} does not support method {}", self.get_class_name(), method_name)
        };
    }
}

pub struct NativeObjectClass { }

impl JavaClass for NativeObjectClass {
    fn new(&self) -> Arc<Mutex<dyn JavaInstance>> {
        Arc::new(Mutex::new(NativeObjectInstance {}))
    }
    fn get_name(&self) -> String {
        return "java/lang/Object".to_string();
    }

    fn print(&self) {
        println!("Native Object class");
    }

    fn execute_method(&self, _sf: &mut StackFrame, method_name: &String, _this: Arc<Mutex<dyn JavaInstance>>, _args: Vec<Arc<Mutex<dyn JavaInstance>>>) -> MethodCallResult {
        match &method_name[..] {
            "<init>" => {
                return MethodCallResult::SUCCESS;
            },
            _ => panic!("Class {} does not support method {}", self.get_name(), method_name)
        };
    }

    fn execute_static_method(&self, _sf: &mut StackFrame, method_name: &String, _nb_args: usize) -> MethodCallResult {
        match &method_name[..] {
            "clinit" => {
                return MethodCallResult::SUCCESS;
            },
            _ => panic!("Class {} does not support static method {}", self.get_name(), method_name)
        };
    }
}

/////////////////// java.io.PrintStream

pub struct NativePrintStreamInstance {}
impl JavaInstance for NativePrintStreamInstance {
    fn get_class_name(&self) -> String {
        return "Stream".to_string();
    }
    fn execute_method(&mut self, _sf: &mut StackFrame, method_name: &String, _this: Arc<Mutex<dyn JavaInstance>>, args: Vec<Arc<Mutex<dyn JavaInstance>>>) {
        match &method_name[..] {
            "println" => {
                let object = args.get(0).unwrap();
                println!("{}", (**object).lock().unwrap().get_string());
            },
            "print" => {
                let object = args.get(0).unwrap();
                print!("{}", (**object).lock().unwrap().get_string());
            },
            _ => panic!("Native class {} does not have method {}", self.get_class_name(), method_name)
        }
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

    fn execute_method(&self, _sf: &mut StackFrame, method_name: &String, _this: Arc<Mutex<dyn JavaInstance>>, args: Vec<Arc<Mutex<dyn JavaInstance>>>) -> MethodCallResult {
        match &method_name[..] {
            "println" => {
                let string = args.get(0).unwrap().lock().unwrap().get_string();

                println!("{}", string);
            },
            "print" => {
                let string = args.get(0).unwrap().lock().unwrap().get_string();

                print!("{}", string);
            },
            _ => panic!("Native class {} does not have method {}", self.get_name(), method_name)
        };
        return MethodCallResult::SUCCESS;
    }
}

/////////////////// java.lang.System

pub struct NativeSystemClass { }

impl JavaClass for NativeSystemClass {
    fn get_name(&self) -> String {
        return "java/lang/System".to_string();
    }

    fn print(&self) {
        println!("Native System class");
    }

    fn get_static_object(&self, field_name: &String) -> Arc<Mutex<dyn JavaInstance>> {
        if field_name.eq("out") {
            return Arc::new(Mutex::new(NativePrintStreamInstance {}));
        }
            
        panic!("Native class {} does not have static field [{}]", self.get_name(), field_name);
    }

}

/////////////////// Null

pub struct NativeNullInstance { }

impl NativeNullInstance {
    pub fn new() -> NativeNullInstance {
        NativeNullInstance { }
    }
}

impl JavaInstance for NativeNullInstance {
    fn get_class_name(&self) -> String { "null".to_string() }
    fn is_null(&self) -> bool { true }
    fn print(&self) { print!("<null>"); }
}

/////////////////// java.lang.Integer
 
pub struct NativeIntegerInstance { value: i32 }

impl NativeIntegerInstance {
    pub fn new(value: i32) -> NativeIntegerInstance {
        NativeIntegerInstance { value }
    }
}

impl JavaInstance for NativeIntegerInstance {
    fn get_class_name(&self) -> String { "java/lang/Integer".to_string() }
    fn get_int(&self) -> i32 { self.value }
    fn get_string(&self) -> String { self.value.to_string() }
    fn print(&self) { print!("{}", self.value); }
}

pub struct NativeIntegerClass { }

impl JavaClass for NativeIntegerClass {
    fn get_name(&self) -> String {
        return "java/lang/Integer".to_string();
    }

    fn print(&self) {
        println!("Native Integer class");
    }

    fn execute_static_method(&self, sf: &mut StackFrame, method_name: &String, _nb_args: usize) -> MethodCallResult {
        match &method_name[..] {
            "parseInt" => {
                let string = sf.pop_string();
                sf.push_int(string.parse::<i32>().unwrap());
            },
            "valueOf" => {
                let int = sf.pop_int();
                sf.push_int(int);
            },
            _ => panic!("Native class {} does not have static method [{}]", self.get_name(), method_name)
        };

        return MethodCallResult::SUCCESS;
    }
}

/////////////////// java.lang.Long

pub struct NativeLongInstance { value: i64 }

impl NativeLongInstance {
    pub fn new(value: i64) -> NativeLongInstance {
        NativeLongInstance { value }
    }
}

impl JavaInstance for NativeLongInstance {
    fn get_class_name(&self) -> String { "java/lang/Long".to_string() }
    fn get_long(&self) -> i64 { self.value }
    fn print(&self) { print!("{}l", self.value); }
}

/////////////////// java.lang.Short

pub struct NativeShortInstance { value: i16 }

impl NativeShortInstance {
    pub fn new(value: i16) -> NativeShortInstance {
        NativeShortInstance { value }
    }
}

impl JavaInstance for NativeShortInstance {
    fn get_class_name(&self) -> String { "java/lang/Short".to_string() }
    fn get_short(&self) -> i16 { self.value }
    fn get_int(&self) -> i32 { self.value as i32 }
    fn print(&self) { print!("{}", self.value); }
}

/////////////////// java.lang.Byte

pub struct NativeByteInstance { value: u8 }

impl NativeByteInstance {
    pub fn new(value: u8) -> NativeByteInstance {
        NativeByteInstance { value }
    }
}

impl JavaInstance for NativeByteInstance {
    fn get_class_name(&self) -> String {
        return "java/lang/Byte".to_string();
    }
    fn get_byte(&self) -> u8 { self.value }
    fn get_int(&self) -> i32 { self.value as i32 }
    fn print(&self) {
        print!("{}", self.value);
    }
}

/////////////////// java.lang.Char

pub struct NativeCharInstance { value: char }

impl NativeCharInstance {
    pub fn new(value: char) -> NativeCharInstance {
        NativeCharInstance { value }
    }
}

impl JavaInstance for NativeCharInstance {
    fn get_class_name(&self) -> String {
        return "java/lang/Char".to_string();
    }
    fn get_char(&self) -> char {
        return self.value;
    }
    fn print(&self) {
        print!("{}", self.value);
    }
}

/////////////////// java.lang.Float

pub struct NativeFloatInstance { value: f32 }
impl JavaInstance for NativeFloatInstance {
    fn get_class_name(&self) -> String {
        return "java/lang/Float".to_string();
    }
    fn get_float(&self) -> f32 {
        return self.value;
    }
    fn print(&self) {
        print!("{}", self.value);
    }
}

impl NativeFloatInstance {
    pub fn new(value: f32) -> NativeFloatInstance {
        NativeFloatInstance { value }
    }
}

/////////////////// java.lang.Double

pub struct NativeDoubleInstance { value: f64 }
impl JavaInstance for NativeDoubleInstance {
    fn get_class_name(&self) -> String {
        return "java/lang/Double".to_string();
    }
    fn get_double(&self) -> f64 {
        return self.value;
    }
    fn print(&self) {
        print!("{}", self.value);
    }
}

impl NativeDoubleInstance {
    pub fn new(value: f64) -> NativeDoubleInstance {
        NativeDoubleInstance { value }
    }
}

/////////////////// java.lang.Boolean

pub struct NativeBooleanInstance { value: bool }
impl JavaInstance for NativeBooleanInstance {
    fn get_class_name(&self) -> String {
        return "java/lang/Boolean".to_string();
    }
    fn get_bool(&self) -> bool {
        return self.value;
    }
    fn get_int(&self) -> i32 {
        if self.value {
            return 1;
        }
        return 0;
    }
    fn print(&self) {
        print!("{}", self.value);
    }
}

impl NativeBooleanInstance {
    pub fn new(value: bool) -> NativeBooleanInstance {
        NativeBooleanInstance { value }
    }
}

/////////////////// java.lang.String

pub struct NativeStringInstance { value: String }
impl JavaInstance for NativeStringInstance {
    fn get_class_name(&self) -> String {
        return "java/lang/String".to_string();
    }
    fn get_string(&self) -> String {
        return self.value.clone();
    }
    fn print(&self) {
        print!("\"{}\"", self.value);
    }
    fn execute_method(&mut self, sf: &mut StackFrame, method_name: &String, _this: Arc<Mutex<dyn JavaInstance>>, args: Vec<Arc<Mutex<dyn JavaInstance>>>) {
        match &method_name[..] {
            "<init>" => {},
            "startsWith" => {
                let arg = args.get(0).unwrap().lock().unwrap().get_string();
                let this = self.get_string();

                sf.push_bool(this.starts_with(&arg));
            },
            "toLowerCase" => {
                let this = self.get_string();
                sf.push_string(this.to_lowercase());
            },
            "hashCode" => {
                let this = self.get_string();
                let mut n = this.len() as u32;
    
                let mut hash: i32 = 0;
    
                let str = this.as_bytes();
                let thirty_one: i32 = 31;
    
                for c in str {
                    n -= 1;
                    hash += (*c as i32) * i32::pow(thirty_one, n);
                }
                sf.push_int(hash);
            },
            "equals" => {
                let arg = args.get(0).unwrap().lock().unwrap().get_string();
                let this = self.get_string();

                sf.push_bool(this.eq(&arg));
            }
            _ => panic!("String.{}() not implemented yet", method_name)
        };        
    }
}

impl NativeStringInstance {
    pub fn new(value: String) -> NativeStringInstance {
        NativeStringInstance { value }
    }
}

pub struct NativeStringClass { }

impl JavaClass for NativeStringClass {
    fn new(&self) -> Arc<Mutex<dyn JavaInstance>> {
        Arc::new(Mutex::new(NativeStringInstance { value: "".to_string() }))
    }
    fn get_name(&self) -> String {
        return "java/lang/String".to_string();
    }

    fn print(&self) {
        println!("Native Integer class");
    }

    fn execute_static_method(&self, sf: &mut StackFrame, method_name: &String, _nb_args: usize) -> MethodCallResult {
        if method_name.eq("format") {
            let array = sf.pop_array();
            let string = sf.pop_string();

            let mut output = String::new();
            let mut special = false;
            let mut array_idx: usize = 0;

            for c in string.chars() {
                if special {
                    match c {
                        'd' => {
                            match array.lock().unwrap().get(array_idx) {
                                Some(object) => output.push_str(&(**object).lock().unwrap().get_int().to_string()),
                                _ => panic!("String.format() does not have enought arguments")
                            };
                        },
                        's' => {
                            match array.lock().unwrap().get(array_idx) {
                                Some(object) => output.push_str(&(**object).lock().unwrap().get_string().clone()),
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
        

            sf.push_string(output);
            return MethodCallResult::SUCCESS;
        }

        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }
}

/////////////////// java.lang.StringBuilder

struct NativeStringBuilderInstance { content: String }
impl JavaInstance for NativeStringBuilderInstance {
    fn get_class_name(&self) -> String { "java/lang/StringBuilder".to_string() }

    fn execute_method(&mut self, sf: &mut StackFrame, method_name: &String, this: Arc<Mutex<dyn JavaInstance>>, args: Vec<Arc<Mutex<dyn JavaInstance>>>) {
        match &method_name[..] {
            "<init>" => { },
            "append" => {
                let string = args[0].lock().unwrap().get_string();
                self.content.push_str(&string);
                sf.push(this.clone());
            },
            "toString" => {
                sf.push_string(self.content.clone());
            },
            _ => panic!("Native class {} does not have method [{}]", self.get_class_name(), method_name)
        };
    }
}

struct NativeStringBuilderClass {}

impl JavaClass for NativeStringBuilderClass {
    fn new(&self) -> Arc<Mutex<dyn JavaInstance>> {
        Arc::new(Mutex::new(NativeStringBuilderInstance { content: "".to_string() }))
    }

    fn get_name(&self) -> String { "java/lang/StringBuilder".to_string() }
    fn print(&self) { println!("Native StringBuilder class"); }
}

/////////////////// java.util.Arrays

pub struct NativeArrayInstance { pub values: Arc<Mutex<Vec<Arc<Mutex<dyn JavaInstance>>>>> }
impl JavaInstance for NativeArrayInstance {
    fn get_class_name(&self) -> String {
        return "java/util/Arrays".to_string();
    }

    fn execute_method(&mut self, sf: &mut StackFrame, method_name: &String, _this: Arc<Mutex<dyn JavaInstance>>, _args: Vec<Arc<Mutex<dyn JavaInstance>>>) {
        match &method_name[..] {
            "clone" => {
                let array: Vec<Arc<Mutex<dyn JavaInstance>>> = self.values.lock().unwrap().clone();
                sf.push_array(Arc::new(Mutex::new(array)));
            },
            _ => panic!("Native instance {} does not support method {}", self.get_class_name(), method_name)
        };
    }

    fn get_array(&self) -> Arc<Mutex<Vec<Arc<Mutex<dyn JavaInstance>>>>> {
        return self.values.clone();
    }

    fn print(&self) {
        print!("[");
        for value in self.values.lock().unwrap().iter() {
            value.lock().unwrap().print();
            print!(", ")
        }
        print!("]");
    }
}

pub struct NativeArraysClass { }

impl JavaClass for NativeArraysClass {
    fn get_name(&self) -> String {
        return "java/util/Arrays".to_string();
    }

    fn print(&self) {
        println!("Native Arrays class");
    }

    fn execute_static_method(&self, sf: &mut StackFrame, method_name: &String, _nb_args: usize) -> MethodCallResult {
        if method_name.eq("asList") {
            let array = sf.pop_array();

            let mut list: Vec<Arc<Mutex<dyn JavaInstance>>> = Vec::new();
            for elt in array.lock().unwrap().iter() {
                list.push((*elt).clone());
            }

            sf.push(Arc::new(Mutex::new(NativeArrayListInstance { content: Arc::new(Mutex::new(list)) })));
            return MethodCallResult::SUCCESS;
        }

        panic!("Native class {} does not have static method [{}]", self.get_name(), method_name);
    }
}

/////////////////// java.util.ArrayList

pub struct NativeArrayListInstance {
    content: Arc<Mutex<Vec<Arc<Mutex<dyn JavaInstance>>>>>
}
impl JavaInstance for NativeArrayListInstance {
    fn get_class_name(&self) -> String {
        return "java/util/ArrayList".to_string();
    }
    fn get_array(&self) -> Arc<Mutex<Vec<Arc<Mutex<dyn JavaInstance>>>>> {
        return self.content.clone();
    }

    fn supports_interface(&self, interface_name: &String) -> bool {
        match &interface_name[..] {
            "java/util/List" => true,
            _ => false
        }
    }
    fn print(&self) {
        let elements = self.content.lock().unwrap();
        print!("<{} instance [", self.get_class_name());
        for element in elements.iter() {
            element.lock().unwrap().print();
            print!(", ");
        }
        print!("]>");
    }
    fn execute_method(&mut self, sf: &mut StackFrame, method_name: &String, _this: Arc<Mutex<dyn JavaInstance>>, args: Vec<Arc<Mutex<dyn JavaInstance>>>) {
        match &method_name[..] {
            "<init>" => {
                return;
            },
            "stream" => {
                let list = self.get_array();
                sf.push(Arc::new(Mutex::new(NativeStreamInstance::new(list))));
            },
            "add" => {
                let value= args.get(0).unwrap();
                self.content.lock().unwrap().push(value.clone());

                sf.push_bool(true);
            },
            _ => panic!("Native class {} does not have method [{}]", self.get_class_name(), method_name)
        };
    }
}

pub struct NativeArrayListClass { }

impl JavaClass for NativeArrayListClass {
    fn new(&self) -> Arc<Mutex<dyn JavaInstance>> {
        Arc::new(Mutex::new(NativeArrayListInstance { content: Arc::new(Mutex::new(Vec::new())) }))
    }

    fn get_name(&self) -> String {
        return "java/util/ArrayList".to_string();
    }

    fn print(&self) {
        println!("Native ArrayList class");
    }
}

/////////////////// java.util.List

struct NativeListInstance {

}
impl JavaInstance for NativeListInstance {
    fn get_class_name(&self) -> String {
        return "java/util/List".to_string();
    }
    fn execute_method(&mut self, sf: &mut StackFrame, method_name: &String, _this: Arc<Mutex<dyn JavaInstance>>, _args: Vec<Arc<Mutex<dyn JavaInstance>>>) {
        match &method_name[..] {
            "stream" => {
                let list = sf.pop_array();
                sf.push(Arc::new(Mutex::new(NativeStreamInstance::new(list))));
            },
            "add" => {
                let value= sf.pop();
                let list = sf.pop_array();
                list.lock().unwrap().push(value);

                sf.push_bool(true);
            }
            _ => panic!("Native class {} does not have method [{}]", self.get_class_name(), method_name)
        };
    }
}

pub struct NativeListClass { }

impl JavaClass for NativeListClass {
    fn get_name(&self) -> String {
        return "java/util/List".to_string();
    }

    fn print(&self) {
        println!("Native List class");
    }
}

/////////////////// java.lang.Math

struct NativeMathClass {}

impl JavaClass for NativeMathClass {
    fn get_name(&self) -> String {
        return "java/lang/Math".to_string();
    }

    fn print(&self) {
        println!("Native Math class");
    }

    fn execute_static_method(&self, sf: &mut StackFrame, method_name: &String, _nb_args: usize) -> MethodCallResult {
        match &method_name[..] {
            "sqrt" => {
                let nb = sf.pop_double();
                sf.push_double(nb.sqrt());
            },
            "log" => {
                let nb = sf.pop_double();
                sf.push_double(nb.ln());
            }
            _ => panic!("Native class {} does not have static method [{}]", self.get_name(), method_name)
        };
        return MethodCallResult::SUCCESS;
    }
}

/////////////////// java.lang.Enum
struct NativeEnumInstance { name: String, ordinal: i32 }
impl JavaInstance for NativeEnumInstance {
    fn get_class_name(&self) -> String {
        return "java/lang/Enum".to_string();
    }

    fn execute_method(&mut self, sf: &mut StackFrame, method_name: &String, _this: Arc<Mutex<dyn JavaInstance>>, args: Vec<Arc<Mutex<dyn JavaInstance>>>) {
        match &method_name[..] {
            "<init>" => {
                self.name = args.get(1).unwrap().lock().unwrap().get_string();
                self.ordinal = args.get(0).unwrap().lock().unwrap().get_int();
            },
            "ordinal" => {
                sf.push_int(self.ordinal);
            },
            _ => panic!("Native class {} does not have method [{}]", self.get_class_name(), method_name)
        };
    }
}

struct NativeEnumClass {}

impl JavaClass for NativeEnumClass {
    fn new(&self) -> Arc<Mutex<dyn JavaInstance>> {
        Arc::new(Mutex::new(NativeEnumInstance { ordinal: 0, name: "".to_string() }))
    }

    fn get_name(&self) -> String {
        return "java/lang/Enum".to_string();
    }

    fn print(&self) {
        println!("Native Enum class");
    }
}

/////////////////// java.lang.Thread

struct NativeThreadInstance {
    object: Arc<Mutex<dyn JavaInstance>>,
    name: String
}

struct ThreadObjects {
    objects: Option<HashMap<i32, Arc<Mutex<dyn JavaInstance>>>>
}

impl ThreadObjects {
    fn add(&mut self, idx: i32, object: Arc<Mutex<dyn JavaInstance>>) {
        unsafe {
            match self.objects.as_mut() {
                Some(map) => { map.insert(idx, object.clone()); },
                None => {
                    let mut mp: HashMap<i32, Arc<Mutex<dyn JavaInstance>>> = HashMap::new();
                    mp.insert(idx, object.clone());
                    THREAD_OBJECTS.objects = Some(mp);
                }
            };
        }
    }
}

fn new_thread(id: i32) {
    let this = get_thread_object(id);
    let var = Arc::new(Mutex::new(NativeObjectInstance {}));
    let variables: [Arc<Mutex<dyn JavaInstance>>; 16] = [var.clone(), var.clone(), var.clone(), var.clone(),
        var.clone(), var.clone(), var.clone(), var.clone(),
        var.clone(), var.clone(), var.clone(), var.clone(),
        var.clone(), var.clone(), var.clone(), var.clone()];

    let mut sf = StackFrame::new(variables);

    let class = get_class(&this.lock().unwrap().get_class_name());
    class.execute_method(&mut sf, &"run".to_string(), this.clone(), Vec::new());
}

static mut THREAD_OBJECTS: ThreadObjects = ThreadObjects { objects: None };

fn get_thread_object(idx: i32) -> Arc<Mutex<dyn JavaInstance>> {
    unsafe {
        match &THREAD_OBJECTS.objects {
            Some(map) => map.get(&idx).unwrap().clone(),
            None => panic!("THREAD_OBJECTS not initialized")
        }
    }
}

impl JavaInstance for NativeThreadInstance {
    fn get_class_name(&self) -> String { "java/lang/Thread".to_string() }

    fn execute_method(&mut self, _sf: &mut StackFrame, method_name: &String, _this: Arc<Mutex<dyn JavaInstance>>, args: Vec<Arc<Mutex<dyn JavaInstance>>>) {
        match &method_name[..] {
            "<init>" => {
                self.name = args[0].lock().unwrap().get_string();
                self.object = args[1].clone();
            },
            "start" => {
                let mut rng = rand::thread_rng();
                let idx = rng.gen::<i32>();

                // Cannot manage to pass the JavaInstance object inside thread::spawn(), so we need to pass it another way
                unsafe { THREAD_OBJECTS.add(idx, self.object.clone()); };
                GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
                thread::spawn(move || {
                    new_thread(idx);
                    GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
                });
            },
            _ => panic!("Class instance {} does not support method {}", self.get_class_name(), method_name)
        };
    }
}

struct NativeThreadClass {}

impl JavaClass for NativeThreadClass {
    fn new(&self) -> Arc<Mutex<dyn JavaInstance>> {
        Arc::new(Mutex::new(NativeThreadInstance { object: Arc::new(Mutex::new(NativeNullInstance {})), name: "".to_string() } ))
    }

    fn execute_static_method(&self, sf: &mut StackFrame, method_name: &String, _nb_args: usize) -> MethodCallResult {
        match &method_name[..] {
            "sleep" => {
                let nb_millis = sf.pop_long() as u64;
                let duration = time::Duration::from_millis(nb_millis);
                thread::sleep(duration);
            },
            _ => panic!("Class instance {} does not support static method {}", self.get_name(), method_name)
        };
        MethodCallResult::SUCCESS
    }

    fn get_name(&self) -> String { "java/lang/Thread".to_string() }
    fn print(&self) { println!("Native Thread class"); }
}

/////////////////// java.lang.NoSuchFieldError

struct NativeNoSuchFieldErrorClass {}

impl JavaClass for NativeNoSuchFieldErrorClass {
    fn get_name(&self) -> String {
        return "java/lang/NoSuchFieldError".to_string();
    }

    fn print(&self) {
        println!("Native NoSuchFieldError class");
    }
}

/////////////////// java.lang.invoke.MethodHandles$Lookup

struct NativeMethodHandlesLookupClass {}

impl JavaClass for NativeMethodHandlesLookupClass {
    fn get_name(&self) -> String {
        return "java/lang/invoke/MethodHandles$Lookup".to_string();
    }

    fn print(&self) {
        println!("Native MethodHandles$Lookup class");
    }
}

/////////////////// java.lang.invoke.MethodHandles

struct NativeMethodHandlesClass {}

impl JavaClass for NativeMethodHandlesClass {
    fn get_name(&self) -> String {
        return "java/lang/invoke/MethodHandles".to_string();
    }

    fn print(&self) {
        println!("Native MethodHandles class");
    }
}

/////////////////// Generic class

pub struct NativeGenericClass { pub name: String }

impl JavaClass for NativeGenericClass {
    fn get_name(&self) -> String { self.name.clone() }
    fn print(&self) { println!("Native {} class", self.name); }
}

/////////////////// java.lang.Exception

pub struct NativeGenericExceptionInstance {
    pub name: String,
    pub message: String,
    pub parent_class_name: String,
    pub stack: Vec<String>
}

impl JavaInstance for NativeGenericExceptionInstance {
    fn get_class_name(&self) -> String { self.name.clone() }
    fn print(&self) { print!("Native {} class", self.name); }
    fn cast_as(&self, this: Arc<Mutex<dyn JavaInstance>>, class_name: &String) -> Arc<Mutex<dyn JavaInstance>> {
        if self.name.eq(class_name) || self.parent_class_name.eq(class_name) {
            return this.clone();
        }
        
        let mut the_class_name = self.parent_class_name.clone();
        let mut class = get_class(&the_class_name);

        while !the_class_name.eq("") {
            the_class_name = class.get_parent();
            if the_class_name.eq(class_name) { return this.clone(); }
            class = get_class(&the_class_name);
        }

        panic!("Instance of class {} cannot be converted to {}", self.name, class_name);
    }
    
    fn execute_method(&mut self, sf: &mut StackFrame, method_name: &String, _this: Arc<Mutex<dyn JavaInstance>>, args: Vec<Arc<Mutex<dyn JavaInstance>>>) {
        match &method_name[..] {
            "<init>" => {
                self.message = args[0].lock().unwrap().get_string();
            },
            "getMessage" => {
                sf.push_string(self.message.clone());
            },
            "printStackTrace" => {
                println!("Exception in {}: {}", self.name, self.message);
                for frame in self.stack.iter() {
                    println!("        at {}", frame);
                }
            },
            "addStackFrame" => {
                self.stack.push(args[0].lock().unwrap().get_string());
            },
            _ => panic!("Instance of class {} does not support method {}", self.get_class_name(), method_name)
        }
    }
}

pub struct NativeGenericExceptionClass {
    name: String,
    parent: String
}

impl JavaClass for NativeGenericExceptionClass {
    fn get_name(&self) -> String { self.name.clone() }
    fn print(&self) { println!("Native {} class", self.get_name()); }
    fn get_parent(&self) -> String {
        self.parent.clone()
    }
    fn new(&self) -> Arc<Mutex<dyn JavaInstance>> {
        let class = get_class(&self.name);
        Arc::new(Mutex::new(NativeGenericExceptionInstance {
            name: self.name.clone(),
            message: "".to_string(),
            parent_class_name: class.get_parent(),
            stack: Vec::new()
        }))
    }
}

impl NativeGenericExceptionClass {
    pub fn new(name: &String, message: &String) -> NativeGenericExceptionInstance {
        let top_exception = "java/lang/Exception".to_string();

        if !class_exists(name) {
            unsafe {
                CLASSES.add(Arc::new(NativeGenericExceptionClass { name: name.clone(), parent: "java/lang/Exception".to_string() }));
            }

            NativeGenericExceptionInstance {
                name: name.clone(),
                message: message.clone(),
                stack: Vec::new(),
                parent_class_name: top_exception.clone()
            }
        } else {
            let class = get_class(name);
            NativeGenericExceptionInstance {
                name: name.clone(),
                message: message.clone(),
                stack: Vec::new(),
                parent_class_name: class.get_name()
            }    
        }
    }
}
/*
pub struct NativeExceptionClass { }

impl JavaClass for NativeExceptionClass {
    fn get_name(&self) -> String { "java/lang/Exception".to_string() }
    fn print(&self) { println!("Native {} class", self.get_name()); }
    fn new(&self) -> Arc<Mutex<dyn JavaInstance>> {
        Arc::new(Mutex::new(NativeGenericExceptionInstance {
            name: self.get_name(),
            message: "".to_string(),
            stack: Vec::new(),
            parent_class_name: "".to_string()
        }))
    }
}
*/