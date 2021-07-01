use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::{get_class, get_debug};
use crate::bytecode_class::{ConstantField, ConstantFloat, ConstantInteger, ConstantLong, ConstantDouble };
use crate::bytecode_class::ConstantString;
use crate::bytecode_class::ConstantStringRef;
use crate::bytecode_class::ConstantClass;
use crate::bytecode_class::ConstantMethod;
use crate::bytecode_class::ConstantNameType;
use crate::bytecode_class::ConstantInvokeDynamic;
use crate::StackFrame;
use crate::bytecode_class::Blob;
use crate::java_class::{JavaClassInstance, MethodCallResult};
use crate::java_class::get_nb_arguments;
use crate::jvm::JavaInstance;
use crate::native_java_classes::{NativeDoubleInstance, NativeGenericExceptionClass};
use crate::native_java_classes::NativeFloatInstance;
use crate::native_java_classes::NativeIntegerInstance;
use crate::native_java_classes::NativeLongInstance;
use crate::native_java_classes::NativeStringInstance;

pub trait ByteCodeInstruction {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction;
    fn print(&self);
    fn set_branch(&mut self, _address_map: &HashMap<usize, usize>) {}
}

pub enum InstrNextAction {
    NEXT,
    RETURN,
    GOTO(usize),
    EXCEPTION(Arc<Mutex<dyn JavaInstance>>)
}

#[macro_export]
macro_rules! exception {
    ( $name:expr, $message:expr ) => {
        InstrNextAction::EXCEPTION(Arc::new(Mutex::new(NativeGenericExceptionClass::new(&$name.to_string(), &$message.to_string()))))
    };
}

////////////////////////////////////////////////////////////////////////////////////
///////////// 0x0

pub struct InstrNop { }
impl ByteCodeInstruction for InstrNop {
    fn execute(&self, _sf: &mut StackFrame) -> InstrNextAction {
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      nop"); }
}

pub struct InstrAConstNull { }
impl ByteCodeInstruction for InstrAConstNull {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.push_null();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      aconst_null"); }
}

pub struct InstrIConst { value: i32 }
impl ByteCodeInstruction for InstrIConst {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.push_int(self.value);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iconst_{}", self.value); }
}

pub struct InstrLConst0 { }
impl ByteCodeInstruction for InstrLConst0 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.push_long(0);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lconst_0"); }
}

pub struct InstrLConst1 { }
impl ByteCodeInstruction for InstrLConst1 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.push_long(1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lconst_1"); }
}

pub struct InstrFConst0 { }
impl ByteCodeInstruction for InstrFConst0 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.push_float(0.0);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fconst_0"); }
}

pub struct InstrFConst1 { }
impl ByteCodeInstruction for InstrFConst1 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.push_float(1.0);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fconst_1"); }
}

pub struct InstrFConst2 { }
impl ByteCodeInstruction for InstrFConst2 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.push_float(2.0);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fconst_2"); }
}

pub struct InstrDConst0 { }
impl ByteCodeInstruction for InstrDConst0 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.push_double(0.0);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dconst_0"); }
}

pub struct InstrDConst1 { }
impl ByteCodeInstruction for InstrDConst1 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.push_double(1.0);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dconst_1"); }
}

///////////// 0x1

pub struct InstrBiPush { value: u8 }
impl ByteCodeInstruction for InstrBiPush {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.push_byte(self.value);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      bipush {}", self.value); }
}

pub struct InstrSiPush { value: i16 }
impl ByteCodeInstruction for InstrSiPush {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.push_short(self.value);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      sipush {}", self.value); }
}

pub struct InstrILoad { variable: u8 }
impl ByteCodeInstruction for InstrILoad {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.variable_to_stack(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iload {}", self.variable); }
}

pub struct InstrLLoad { variable: u8 }
impl ByteCodeInstruction for InstrLLoad {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.variable_to_stack(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lload {}", self.variable); }
}

pub struct InstrFLoad { variable: u8 }
impl ByteCodeInstruction for InstrFLoad {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.variable_to_stack(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fload {}", self.variable); }
}

pub struct InstrDLoad { variable: u8 }
impl ByteCodeInstruction for InstrDLoad {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.variable_to_stack(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dload {}", self.variable); }
}

pub struct InstrALoad { variable: u8 }
impl ByteCodeInstruction for InstrALoad {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.variable_to_stack(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      aload {}", self.variable); }
}

pub struct InstrLdc { value: Arc<Mutex<dyn JavaInstance>> }
impl ByteCodeInstruction for InstrLdc {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.push(self.value.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) {
        print!("      ldc ");
        self.value.lock().unwrap().print();
        println!();
    }
}

pub struct InstrLdcF { value: f32 }
impl ByteCodeInstruction for InstrLdcF {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.push_float(self.value);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ldc {}", self.value); }
}

pub struct InstrILoadN { variable: u8 }
impl ByteCodeInstruction for InstrILoadN {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.variable_to_stack(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iload_{}", self.variable); }
}

pub struct InstrLLoadN { variable: u8 }
impl ByteCodeInstruction for InstrLLoadN {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.variable_to_stack(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lload_{}", self.variable); }
}

///////////// 0x2

pub struct InstrFLoadN { variable: u8 }
impl ByteCodeInstruction for InstrFLoadN {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.variable_to_stack(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fload_{}", self.variable); }
}

pub struct InstrDLoadN { variable: u8 }
impl ByteCodeInstruction for InstrDLoadN {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.variable_to_stack(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dload_{}", self.variable); }
}

pub struct InstrALoadN { variable: u8 }
impl ByteCodeInstruction for InstrALoadN {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.variable_to_stack(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      aload{}", self.variable); }
}

pub struct InstrIALoad {}
impl ByteCodeInstruction for InstrIALoad {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let idx = sf.pop_int();
        let arg = sf.pop_array();
        let array = arg.lock().unwrap();
        let object: &Arc<Mutex<dyn JavaInstance>> = match array.get(idx as usize) {
            Some(obj) => obj,
            _ => panic!("No object in the array at index {}", idx)
        };
        sf.push(object.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iaload"); }
}

pub struct InstrLALoad {}
impl ByteCodeInstruction for InstrLALoad {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let idx = sf.pop_int();
        let arg = sf.pop_array();
        let array = arg.lock().unwrap();
        let object: &Arc<Mutex<dyn JavaInstance>> = match array.get(idx as usize) {
            Some(obj) => obj,
            _ => panic!("No object in the array at index {}", idx)
        };
        sf.push(object.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      laload"); }
}

///////////// 0x3

pub struct InstrFALoad {}
impl ByteCodeInstruction for InstrFALoad {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let idx = sf.pop_int();
        let arg = sf.pop_array();
        let array = arg.lock().unwrap();
        let object: &Arc<Mutex<dyn JavaInstance>> = match array.get(idx as usize) {
            Some(obj) => obj,
            _ => panic!("No object in the array at index {}", idx)
        };
        sf.push(object.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      faload"); }
}

pub struct InstrDALoad {}
impl ByteCodeInstruction for InstrDALoad {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let idx = sf.pop_int();
        let arg = sf.pop_array();
        let array = arg.lock().unwrap();
        let object: &Arc<Mutex<dyn JavaInstance>> = match array.get(idx as usize) {
            Some(obj) => obj,
            _ => panic!("No object in the array at index {}", idx)
        };
        sf.push(object.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      daload"); }
}

pub struct InstrAALoad {}
impl ByteCodeInstruction for InstrAALoad {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let idx = sf.pop_int();
        let arg = sf.pop_array();
        let array = arg.lock().unwrap();
        let object: &Arc<Mutex<dyn JavaInstance>> = match array.get(idx as usize) {
            Some(obj) => obj,
            _ => panic!("No object in the array at index {}", idx)
        };
        sf.push(object.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      aaload"); }
}

pub struct InstrBALoad {}
impl ByteCodeInstruction for InstrBALoad {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let idx = sf.pop_int();
        let arg = sf.pop_array();
        let array = arg.lock().unwrap();
        let object: &Arc<Mutex<dyn JavaInstance>> = match array.get(idx as usize) {
            Some(obj) => obj,
            _ => panic!("No object in the array at index {}", idx)
        };
        sf.push(object.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      baload"); }
}

pub struct InstrCALoad {}
impl ByteCodeInstruction for InstrCALoad {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let idx = sf.pop_int();
        let arg = sf.pop_array();
        let array = arg.lock().unwrap();
        let object: &Arc<Mutex<dyn JavaInstance>> = match array.get(idx as usize) {
            Some(obj) => obj,
            _ => panic!("No object in the array at index {}", idx)
        };
        sf.push(object.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      caload"); }
}

pub struct InstrSALoad {}
impl ByteCodeInstruction for InstrSALoad {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let idx = sf.pop_int();
        let arg = sf.pop_array();
        let array = arg.lock().unwrap();
        let object: &Arc<Mutex<dyn JavaInstance>> = match array.get(idx as usize) {
            Some(obj) => obj,
            _ => panic!("No object in the array at index {}", idx)
        };
        sf.push(object.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      saload"); }
}

pub struct InstrIStore { variable: u8 }
impl ByteCodeInstruction for InstrIStore {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      istore {}", self.variable); }
}

pub struct InstrLStore { variable: u8 }
impl ByteCodeInstruction for InstrLStore {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lstore {}", self.variable); }
}

pub struct InstrFStore { variable: u8 }
impl ByteCodeInstruction for InstrFStore {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fstore {}", self.variable); }
}

pub struct InstrDStore { variable: u8 }
impl ByteCodeInstruction for InstrDStore {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dstore {}", self.variable); }
}

pub struct InstrAStore { variable: u8 }
impl ByteCodeInstruction for InstrAStore {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(self.variable as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      astore {}", self.variable); }
}

pub struct InstrIStore0 {}
impl ByteCodeInstruction for InstrIStore0 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(0);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      istore_0"); }
}

pub struct InstrIStore1 {}
impl ByteCodeInstruction for InstrIStore1 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      istore_1"); }
}

pub struct InstrIStore2 {}
impl ByteCodeInstruction for InstrIStore2 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(2);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      istore_2"); }
}

pub struct InstrIStore3 {}
impl ByteCodeInstruction for InstrIStore3 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(3);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      istore_3"); }
}

///////////// 0x4

pub struct InstrFStore0 {}
impl ByteCodeInstruction for InstrFStore0 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(0);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fstore_0"); }
}

pub struct InstrFStore1 {}
impl ByteCodeInstruction for InstrFStore1 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fstore_1"); }
}

pub struct InstrFStore2 {}
impl ByteCodeInstruction for InstrFStore2 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(2);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fstore_2"); }
}

pub struct InstrFStore3 {}
impl ByteCodeInstruction for InstrFStore3 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(3);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fstore_3"); }
}

pub struct InstrAStore0 {}
impl ByteCodeInstruction for InstrAStore0 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(0);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      astore_0"); }
}

pub struct InstrAStore1 {}
impl ByteCodeInstruction for InstrAStore1 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      astore_1"); }
}

pub struct InstrAStore2 {}
impl ByteCodeInstruction for InstrAStore2 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(2);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      astore_2"); }
}

pub struct InstrAStore3 {}
impl ByteCodeInstruction for InstrAStore3 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.stack_to_variable(3);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      astore_3"); }
}

pub struct InstrIAStore {}
impl ByteCodeInstruction for InstrIAStore {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let object = sf.pop();
        let idx = sf.pop_int();
        let array = sf.pop_array();
        array.lock().unwrap()[idx as usize] = object;
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iastore"); }
}

///////////// 0x5

pub struct InstrLAStore {}
impl ByteCodeInstruction for InstrLAStore {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let object = sf.pop();
        let idx = sf.pop_int();
        let array = sf.pop_array();
        array.lock().unwrap()[idx as usize] = object;
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lastore"); }
}

pub struct InstrFAStore {}
impl ByteCodeInstruction for InstrFAStore {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let object = sf.pop();
        let idx = sf.pop_int();
        let array = sf.pop_array();
        array.lock().unwrap()[idx as usize] = object;
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fastore"); }
}

pub struct InstrDAStore {}
impl ByteCodeInstruction for InstrDAStore {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let object = sf.pop();
        let idx = sf.pop_int();
        let array = sf.pop_array();
        array.lock().unwrap()[idx as usize] = object;
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dastore"); }
}

pub struct InstrAAStore {}
impl ByteCodeInstruction for InstrAAStore {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let object = sf.pop();
        let idx = sf.pop_int();
        let array = sf.pop_array();
        array.lock().unwrap()[idx as usize] = object;
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      aastore"); }
}

pub struct InstrBAStore {}
impl ByteCodeInstruction for InstrBAStore {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let object = sf.pop();
        let idx = sf.pop_int();
        let array = sf.pop_array();
        array.lock().unwrap()[idx as usize] = object;
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      bastore"); }
}

pub struct InstrCAStore {}
impl ByteCodeInstruction for InstrCAStore {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let object = sf.pop();
        let idx = sf.pop_int();
        let array = sf.pop_array();
        array.lock().unwrap()[idx as usize] = object;
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      castore"); }
}

pub struct InstrSAStore {}
impl ByteCodeInstruction for InstrSAStore {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let object = sf.pop();
        let idx = sf.pop_int();
        let array = sf.pop_array();
        array.lock().unwrap()[idx as usize] = object;
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      sastore"); }
}

pub struct InstrPop { }
impl ByteCodeInstruction for InstrPop {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.pop();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      pop"); }
}

pub struct InstrPop2 { }
impl ByteCodeInstruction for InstrPop2 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.pop();
        sf.pop();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      pop"); }
}

pub struct InstrDup { }
impl ByteCodeInstruction for InstrDup {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let arg = sf.pop();
        sf.push(arg.clone());
        sf.push(arg.clone());

        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dup"); }
}

pub struct InstrDupX1 { }
impl ByteCodeInstruction for InstrDupX1 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value1 = sf.pop();
        let value2 = sf.pop();
        sf.push(value1.clone());
        sf.push(value2.clone());
        sf.push(value1.clone());

        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dup_x1"); }
}

pub struct InstrDupX2 { }
impl ByteCodeInstruction for InstrDupX2 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value1 = sf.pop();
        let value2 = sf.pop();
        let value3 = sf.pop();
        sf.push(value1.clone());
        sf.push(value3.clone());
        sf.push(value2.clone());
        sf.push(value1.clone());

        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dup_x2"); }
}

pub struct InstrDup2 { }
impl ByteCodeInstruction for InstrDup2 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value1 = sf.pop();
        let value2 = sf.pop();
        sf.push(value2.clone());
        sf.push(value1.clone());
        sf.push(value2.clone());
        sf.push(value1.clone());

        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dup2"); }
}

pub struct InstrDup2X1 { }
impl ByteCodeInstruction for InstrDup2X1 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value1 = sf.pop();
        let value2 = sf.pop();
        let value3 = sf.pop();
        sf.push(value2.clone());
        sf.push(value1.clone());
        sf.push(value3.clone());
        sf.push(value2.clone());
        sf.push(value1.clone());

        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dup2_x1"); }
}

pub struct InstrDup2X2 { }
impl ByteCodeInstruction for InstrDup2X2 {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value1 = sf.pop();
        let value2 = sf.pop();
        let value3 = sf.pop();
        let value4 = sf.pop();
        sf.push(value2.clone());
        sf.push(value1.clone());
        sf.push(value4.clone());
        sf.push(value3.clone());
        sf.push(value2.clone());
        sf.push(value1.clone());

        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dup2_x2"); }
}

pub struct InstrSwap { }
impl ByteCodeInstruction for InstrSwap {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value1 = sf.pop();
        let value2 = sf.pop();
        sf.push(value1.clone());
        sf.push(value2.clone());

        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      swap"); }
}

///////////// 0x6

pub struct InstrIAdd {}
impl ByteCodeInstruction for InstrIAdd {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_int();
        let nb2 = sf.pop_int();
        sf.push_int(nb2 + nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iadd"); }
}

pub struct InstrLAdd {}
impl ByteCodeInstruction for InstrLAdd {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_long();
        let nb2 = sf.pop_long();
        sf.push_long(nb2 + nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ladd"); }
}

pub struct InstrFAdd {}
impl ByteCodeInstruction for InstrFAdd {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_float();
        let nb2 = sf.pop_float();
        sf.push_float(nb2 + nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fadd"); }
}

pub struct InstrDAdd {}
impl ByteCodeInstruction for InstrDAdd {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_double();
        let nb2 = sf.pop_double();
        sf.push_double(nb2 + nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dadd"); }
}

pub struct InstrISub {}
impl ByteCodeInstruction for InstrISub {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_int();
        let nb2 = sf.pop_int();
        sf.push_int(nb2 - nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      isub"); }
}

pub struct InstrLSub {}
impl ByteCodeInstruction for InstrLSub {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_long();
        let nb2 = sf.pop_long();
        sf.push_long(nb2 - nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lsub"); }
}

pub struct InstrFSub {}
impl ByteCodeInstruction for InstrFSub {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_float();
        let nb2 = sf.pop_float();
        sf.push_float(nb2 - nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fsub"); }
}

pub struct InstrDSub {}
impl ByteCodeInstruction for InstrDSub {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_double();
        let nb2 = sf.pop_double();
        sf.push_double(nb2 - nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dsub"); }
}

pub struct InstrIMul {}
impl ByteCodeInstruction for InstrIMul {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_int();
        let nb2 = sf.pop_int();
        sf.push_int(nb2 * nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      imul"); }
}

pub struct InstrLMul {}
impl ByteCodeInstruction for InstrLMul {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_long();
        let nb2 = sf.pop_long();
        sf.push_long(nb2 * nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lmul"); }
}

pub struct InstrFMul {}
impl ByteCodeInstruction for InstrFMul {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_float();
        let nb2 = sf.pop_float();
        sf.push_float(nb2 * nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fmul"); }
}

pub struct InstrDMul {}
impl ByteCodeInstruction for InstrDMul {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_double();
        let nb2 = sf.pop_double();
        sf.push_double(nb2 * nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dmul"); }
}

pub struct InstrIDiv {}
impl ByteCodeInstruction for InstrIDiv {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_int();
        let nb2 = sf.pop_int();
        if nb1 == 0 { return exception!("java/lang/ArithmeticException", "/ by zero"); }

        sf.push_int(nb2 / nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      idiv"); }
}

pub struct InstrLDiv {}
impl ByteCodeInstruction for InstrLDiv {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_long();
        let nb2 = sf.pop_long();
        if nb1 == 0 { return exception!("java/lang/ArithmeticException", "/ by zero"); }

        sf.push_long(nb2 / nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ldiv"); }
}

pub struct InstrFDiv {}
impl ByteCodeInstruction for InstrFDiv {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_float();
        let nb2 = sf.pop_float();
        if nb1 == 0.0 { return exception!("java/lang/ArithmeticException", "/ by zero"); }

        sf.push_float(nb2 / nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fdiv"); }
}

pub struct InstrDDiv {}
impl ByteCodeInstruction for InstrDDiv {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_double();
        let nb2 = sf.pop_double();
        if nb1 == 0.0 { return exception!("java/lang/ArithmeticException", "/ by zero"); }

        sf.push_double(nb2 / nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ddiv"); }
}

///////////// 0x7

pub struct InstrIRem {}
impl ByteCodeInstruction for InstrIRem {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_int();
        let nb2 = sf.pop_int();
        if nb1 == 0 { return exception!("java/lang/ArithmeticException", "/ by zero"); }

        sf.push_int(nb2 % nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      irem"); }
}

pub struct InstrLRem {}
impl ByteCodeInstruction for InstrLRem {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_long();
        let nb2 = sf.pop_long();
        if nb1 == 0 { return exception!("java/lang/ArithmeticException", "/ by zero"); }

        sf.push_long(nb2 % nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lrem"); }
}

pub struct InstrFRem {}
impl ByteCodeInstruction for InstrFRem {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_float();
        let nb2 = sf.pop_float();
        if nb1 == 0.0 { return exception!("java/lang/ArithmeticException", "/ by zero"); }

        sf.push_float(nb2 % nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      frem"); }
}

pub struct InstrDRem {}
impl ByteCodeInstruction for InstrDRem {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_double();
        let nb2 = sf.pop_double();
        if nb1 == 0.0 { return exception!("java/lang/ArithmeticException", "/ by zero"); }

        sf.push_double(nb2 % nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      drem"); }
}

pub struct InstrINeg {}
impl ByteCodeInstruction for InstrINeg {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_int();
        sf.push_int(-nb);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ineg"); }
}

pub struct InstrLNeg {}
impl ByteCodeInstruction for InstrLNeg {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_long();
        sf.push_long(-nb);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lneg"); }
}

pub struct InstrFNeg {}
impl ByteCodeInstruction for InstrFNeg {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_float();
        sf.push_float(-nb);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fneg"); }
}

pub struct InstrDNeg {}
impl ByteCodeInstruction for InstrDNeg {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_double();
        sf.push_double(-nb);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dneg"); }
}

pub struct InstrIShl {}
impl ByteCodeInstruction for InstrIShl {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop_int() & 31;
        let value1 = sf.pop_int();

        sf.push_int(value1 << value2);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ishl"); }
}

pub struct InstrLShl {}
impl ByteCodeInstruction for InstrLShl {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop_long() & 63;
        let value1 = sf.pop_long();

        sf.push_long(value1 << value2);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lshl"); }
}

pub struct InstrIShr {}
impl ByteCodeInstruction for InstrIShr {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop_int() & 31;
        let value1 = sf.pop_int();

        sf.push_int(value1 >> value2);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ishr"); }
}

pub struct InstrLShr {}
impl ByteCodeInstruction for InstrLShr {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop_long() & 63;
        let value1 = sf.pop_long();

        sf.push_long(value1 >> value2);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lshr"); }
}

pub struct InstrIUShr {}
impl ByteCodeInstruction for InstrIUShr {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop_int() & 31;
        let value1 = sf.pop_int();

        if value1 >= 0 {
            sf.push_int(value1 >> value2);
        } else {
            sf.push_int(-((-value1) >> value2));
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iushr"); }
}

pub struct InstrLUShr {}
impl ByteCodeInstruction for InstrLUShr {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop_long() & 63;
        let value1 = sf.pop_long();

        if value1 >= 0 {
            sf.push_long(value1 >> value2);
        } else {
            sf.push_long(-((-value1) >> value2));
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lushr"); }
}

pub struct InstrIAnd {}
impl ByteCodeInstruction for InstrIAnd {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_int();
        let nb2 = sf.pop_int();
        sf.push_int(nb2 & nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iand"); }
}

pub struct InstrLAnd {}
impl ByteCodeInstruction for InstrLAnd {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_long();
        let nb2 = sf.pop_long();
        sf.push_long(nb2 & nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      land"); }
}

///////////// 0x8

pub struct InstrIOr {}
impl ByteCodeInstruction for InstrIOr {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_int();
        let nb2 = sf.pop_int();
        sf.push_int(nb2 | nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ior"); }
}

pub struct InstrLOr {}
impl ByteCodeInstruction for InstrLOr {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_long();
        let nb2 = sf.pop_long();
        sf.push_long(nb2 | nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lor"); }
}

pub struct InstrIXor {}
impl ByteCodeInstruction for InstrIXor {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_int();
        let nb2 = sf.pop_int();
        sf.push_int(nb2 ^ nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ixor"); }
}

pub struct InstrLXor {}
impl ByteCodeInstruction for InstrLXor {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb1 = sf.pop_long();
        let nb2 = sf.pop_long();
        sf.push_long(nb2 ^ nb1);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lxor"); }
}

pub struct InstrIInc { idx: u8, count: i8 }
impl ByteCodeInstruction for InstrIInc {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.variable_to_stack(self.idx as usize);
        let nb = sf.pop_int();
        sf.push_int(nb + self.count as i32);
        sf.stack_to_variable(self.idx as usize);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iinc {} {}", self.idx, self.count); }
}

pub struct InstrI2L {}
impl ByteCodeInstruction for InstrI2L {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_int();
        sf.push_long(nb as i64);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      i2l"); }
}

pub struct InstrI2F {}
impl ByteCodeInstruction for InstrI2F {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_int();
        sf.push_float(nb as f32);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      i2f"); }
}

pub struct InstrI2D {}
impl ByteCodeInstruction for InstrI2D {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_int();
        sf.push_double(nb as f64);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      i2d"); }
}

pub struct InstrL2I {}
impl ByteCodeInstruction for InstrL2I {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_long();
        sf.push_int(nb as i32);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      l2i"); }
}

pub struct InstrL2F {}
impl ByteCodeInstruction for InstrL2F {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_long();
        sf.push_float(nb as f32);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      l2f"); }
}

pub struct InstrL2D {}
impl ByteCodeInstruction for InstrL2D {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_long();
        sf.push_double(nb as f64);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      l2d"); }
}

pub struct InstrF2I {}
impl ByteCodeInstruction for InstrF2I {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_float();
        sf.push_int(nb as i32);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      f2i"); }
}

pub struct InstrF2L {}
impl ByteCodeInstruction for InstrF2L {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_float();
        sf.push_long(nb as i64);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      f2l"); }
}

pub struct InstrF2D {}
impl ByteCodeInstruction for InstrF2D {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_float();
        sf.push_double(nb as f64);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      f2d"); }
}

pub struct InstrD2I {}
impl ByteCodeInstruction for InstrD2I {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_double();
        sf.push_int(nb as i32);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      d2i"); }
}

pub struct InstrD2L {}
impl ByteCodeInstruction for InstrD2L {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_double();
        sf.push_long(nb as i64);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      d2l"); }
}

///////////// 0x9

pub struct InstrD2F {}
impl ByteCodeInstruction for InstrD2F {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_double();
        sf.push_float(nb as f32);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      d2f"); }
}

pub struct InstrI2B {}
impl ByteCodeInstruction for InstrI2B {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_int();
        sf.push_byte(nb as u8);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      i2b"); }
}

pub struct InstrI2C {}
impl ByteCodeInstruction for InstrI2C {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_int();
        sf.push_char(nb as u8 as char);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      i2c"); }
}

pub struct InstrI2S {}
impl ByteCodeInstruction for InstrI2S {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb = sf.pop_int();
        sf.push_short(nb as i16);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      i2s"); }
}

pub struct InstrLCmp {}
impl ByteCodeInstruction for InstrLCmp {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb2 = sf.pop_long();
        let nb1 = sf.pop_long();
        let result: i32;
        if nb1 == nb2 {
            result = 0;
        } else if nb1 > nb2 {
            result = 1;
        } else  {
            result = -1;
        }

        sf.push_int(result);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lcmp"); }
}

pub struct InstrFCmpl {}
impl ByteCodeInstruction for InstrFCmpl {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb2 = sf.pop_float();
        let nb1 = sf.pop_float();
        let mut result: i32 = 0;
        if nb1 == f32::NAN || nb2 == f32::NAN {
            result = -1;
        } else if nb1 > nb2 {
            result = 1;
        } else if nb1 < nb2 {
            result = -1;
        }

        sf.push_int(result);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fcmpl"); }
}

pub struct InstrFCmpg {}
impl ByteCodeInstruction for InstrFCmpg {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb2 = sf.pop_float();
        let nb1 = sf.pop_float();
        let mut result: i32 = 0;
        if nb1 == f32::NAN || nb2 == f32::NAN {
            result = 1;
        } else if nb1 > nb2 {
            result = 1;
        } else if nb1 < nb2 {
            result = -1;
        }

        sf.push_int(result);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fcmpg"); }
}

pub struct InstrDCmpl {}
impl ByteCodeInstruction for InstrDCmpl {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb2 = sf.pop_double();
        let nb1 = sf.pop_double();
        let mut result: i32 = 0;
        if nb1 == f64::NAN || nb2 == f64::NAN {
            result = -1;
        } else if nb1 > nb2 {
            result = 1;
        } else if nb1 < nb2 {
            result = -1;
        }

        sf.push_int(result);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dcmpl"); }
}

pub struct InstrDCmpg {}
impl ByteCodeInstruction for InstrDCmpg {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let nb2 = sf.pop_double();
        let nb1 = sf.pop_double();
        let mut result: i32 = 0;
        if nb1 == f64::NAN || nb2 == f64::NAN {
            result = 1;
        } else if nb1 > nb2 {
            result = 1;
        } else if nb1 < nb2 {
            result = -1;
        }

        sf.push_int(result);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dcmpg"); }
}

pub struct InstrIfeq { branch: usize }
impl ByteCodeInstruction for InstrIfeq {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let arg = sf.pop_int();
        if arg == 0 {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ifeq {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrIfne { branch: usize }
impl ByteCodeInstruction for InstrIfne {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let arg = sf.pop_int();
        if arg != 0 {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ifne {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrIflt { branch: usize }
impl ByteCodeInstruction for InstrIflt {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let arg = sf.pop_int();
        if arg < 0 {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iflt {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrIfge { branch: usize }
impl ByteCodeInstruction for InstrIfge {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let arg = sf.pop_int();
        if arg >= 0 {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ifge {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrIfgt { branch: usize }
impl ByteCodeInstruction for InstrIfgt {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let arg = sf.pop_int();
        if arg > 0 {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ifgt {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrIfle { branch: usize }
impl ByteCodeInstruction for InstrIfle {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let arg = sf.pop_int();
        if arg <= 0 {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ifle {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

///////////// 0xa

pub struct InstrIfICmpEq { branch: usize }
impl ByteCodeInstruction for InstrIfICmpEq {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop_int();
        let value1 = sf.pop_int();
        if value1 == value2 {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      if_icmpeq {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrIfICmpNe { branch: usize }
impl ByteCodeInstruction for InstrIfICmpNe {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop_int();
        let value1 = sf.pop_int();
        if value1 != value2 {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      if_icmpne {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrIfICmpLt { branch: usize }
impl ByteCodeInstruction for InstrIfICmpLt {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop_int();
        let value1 = sf.pop_int();
        if value1 < value2 {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      if_icmplt {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrIfICmpGe { branch: usize }
impl ByteCodeInstruction for InstrIfICmpGe {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop_int();
        let value1 = sf.pop_int();
        if value1 >= value2 {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      if_icmpge {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrIfICmpGt { branch: usize }
impl ByteCodeInstruction for InstrIfICmpGt {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop_int();
        let value1 = sf.pop_int();
        if value1 > value2 {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      if_icmpgt {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrIfICmpLe { branch: usize }
impl ByteCodeInstruction for InstrIfICmpLe {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop_int();
        let value1 = sf.pop_int();
        if value1 <= value2 {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      if_icmple {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrIfACmpEq { branch: usize }
impl ByteCodeInstruction for InstrIfACmpEq {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop();
        let value1 = sf.pop();
        if ::std::ptr::eq(&value1, &value2) {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      if_acmpeq {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrIfACmpNe { branch: usize }
impl ByteCodeInstruction for InstrIfACmpNe {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value2 = sf.pop();
        let value1 = sf.pop();
        if !::std::ptr::eq(&value1, &value2) {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      if_acmpne {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrGoto { branch: usize }
impl ByteCodeInstruction for InstrGoto {
    fn execute(&self, _sf: &mut StackFrame) -> InstrNextAction {
        return InstrNextAction::GOTO(self.branch);
    }
    fn print(&self) { println!("      goto {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrTableSwitch { default: usize, low: usize, table: Vec<usize> }
impl ByteCodeInstruction for InstrTableSwitch {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let idx = sf.pop_int();
        
        let offset = (idx - self.low as i32) as usize;
        match self.table.get(offset) {
            Some(goto) => return InstrNextAction::GOTO(*goto),
            _ => return InstrNextAction::GOTO(self.default)
        };
    }

    fn print(&self) {
        print!("      tableswitch");
        let mut nb = self.low;
        for jump in self.table.iter() {
            print!("  {}=>{}", nb, *jump);
            nb += 1;
        }
        println!("  default=>{}", self.default);
    }

    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        let mut new_table: Vec<usize> = Vec::new();
        for goto in self.table.iter() {
            match address_map.get(goto) {
                Some(new_address) => new_table.push(*new_address),
                _ => panic!("Unknown branch position {}", goto)
            };
        }
        self.table = new_table;

        match address_map.get(&self.default) {
            Some(new_address) => self.default = *new_address,
            _ => panic!("Unknown branch position {}", self.default)
        };
    }
}

pub struct InstrLookupSwitch { default: usize, lookup: HashMap<i32, usize> }
impl ByteCodeInstruction for InstrLookupSwitch {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let idx = sf.pop_int();
        
        match self.lookup.get(&idx) {
            Some(goto) => return InstrNextAction::GOTO(*goto),
            _ => return InstrNextAction::GOTO(self.default)
        };
    }

    fn print(&self) {
        print!("      lookupswitch");
        for (value, goto) in self.lookup.iter() {
            print!("  {}=>{}", value, goto);
        }
        println!("  default=>{}", self.default);
    }

    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        let mut new_lookup: HashMap<i32, usize> = HashMap::new();
        for (value, goto) in self.lookup.iter() {
            match address_map.get(goto) {
                Some(new_address) => new_lookup.insert(*value, *new_address),
                _ => panic!("Unknown branch position {}", goto)
            };
        }
        self.lookup = new_lookup;

        match address_map.get(&self.default) {
            Some(new_address) => self.default = *new_address,
            _ => panic!("Unknown branch position {}", self.default)
        };
    }
}

pub struct InstrIReturn {}
impl ByteCodeInstruction for InstrIReturn {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.set_return_arg_flag();
        return InstrNextAction::RETURN;
    }
    fn print(&self) { println!("      ireturn"); }
}

pub struct InstrLReturn {}
impl ByteCodeInstruction for InstrLReturn {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.set_return_arg_flag();
        return InstrNextAction::RETURN;
    }
    fn print(&self) { println!("      lreturn"); }
}

pub struct InstrFReturn {}
impl ByteCodeInstruction for InstrFReturn {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.set_return_arg_flag();
        return InstrNextAction::RETURN;
    }
    fn print(&self) { println!("      freturn"); }
}

pub struct InstrDReturn {}
impl ByteCodeInstruction for InstrDReturn {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.set_return_arg_flag();
        return InstrNextAction::RETURN;
    }
    fn print(&self) { println!("      dreturn"); }
}

///////////// 0xb

pub struct InstrAReturn {}
impl ByteCodeInstruction for InstrAReturn {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        sf.set_return_arg_flag();
        return InstrNextAction::RETURN;
    }
    fn print(&self) { println!("      areturn"); }
}

pub struct InstrReturn {}
impl ByteCodeInstruction for InstrReturn {
    fn execute(&self, _sf: &mut StackFrame) -> InstrNextAction {
        return InstrNextAction::RETURN;
    }
    fn print(&self) { println!("      return"); }
}

pub struct InstrGetStatic { class_name: String, field_name: String, type_desc: String }
impl ByteCodeInstruction for InstrGetStatic {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let class = get_class(&self.class_name);
        sf.push(class.get_static_object(&self.field_name));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      getstatic {}.{} -> {}", self.class_name, self.field_name, self.type_desc); }
}
pub struct InstrPutStatic { class_name: String, field_name: String, type_desc: String }
impl ByteCodeInstruction for InstrPutStatic {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let class = get_class(&self.class_name);
        class.put_static_object(&self.field_name, sf.pop());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      putstatic {}.{} <- {}", self.class_name, self.field_name, self.type_desc); }
}

pub struct InstrGetField { class_name: String, field_name: String }
impl ByteCodeInstruction for InstrGetField {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let instance = sf.pop();
        let field = instance.lock().unwrap().get_field(&self.field_name);
        sf.push(field);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      getfield {}.{}", self.class_name, self.field_name); }
}

pub struct InstrPutField { class_name: String, field_name: String }
impl ByteCodeInstruction for InstrPutField {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let value = sf.pop();
        let instance = sf.pop();
        instance.lock().unwrap().set_field(&self.field_name, value);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      putfield {}.{}", self.class_name, self.field_name); }
}

pub struct InstrInvokeVirtual { class_name: String, method_name: String, type_desc: String, nb_args: usize }
impl ByteCodeInstruction for InstrInvokeVirtual {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let mut args: Vec<Arc<Mutex<dyn JavaInstance>>> = Vec::new();
        for _ in 0..self.nb_args {
            args.push(sf.pop().clone());
        }
        let this = sf.pop();
        let class = get_class(&self.class_name);
        class.execute_method(sf, &self.method_name, this, args);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      invokevirtual {}.{}{}(<{} arguments>)", self.class_name, self.method_name, self.type_desc, self.nb_args); }
}

pub struct InstrInvokeSpecial { class_name: String, method_name: String, type_desc: String, nb_args: usize }
impl ByteCodeInstruction for InstrInvokeSpecial {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        if get_debug() >= 1 { sf.print_stack(); }

        let mut args: Vec<Arc<Mutex<dyn JavaInstance>>> = Vec::new();
        for _ in 0..self.nb_args {
            args.push(sf.pop().clone());
        }
        let this = sf.pop();
        let class = get_class(&self.class_name);
        class.execute_method(sf,  &self.method_name, this, args);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      invokespecial {}.{}{}(<{} arguments>)", self.class_name, self.method_name, self.type_desc, self.nb_args); }
}

pub struct InstrInvokeStatic { class_name: String, method_name: String, type_desc: String, nb_args: usize }
impl ByteCodeInstruction for InstrInvokeStatic {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let class = get_class(&self.class_name);
        let result = class.execute_static_method(sf, &self.method_name, self.nb_args);

        match result {
            MethodCallResult::SUCCESS => InstrNextAction::NEXT,
            MethodCallResult::EXCEPTION(e) => InstrNextAction::EXCEPTION(e)
        }
    }
    fn print(&self) { println!("      invokestatic {}.{}{}(<{} arguments>)", self.class_name, self.method_name, self.type_desc, self.nb_args); }
}

pub struct InstrInvokeInterface { class_name: String, method_name: String, type_desc: String, count: usize, nb_args: usize }
impl ByteCodeInstruction for InstrInvokeInterface {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {

        let mut args: Vec<Arc<Mutex<dyn JavaInstance>>> = Vec::new();
        for _ in 0..self.nb_args {
            args.push(sf.pop().clone());
        }
        let this = sf.pop();
        let class = get_class(&self.class_name);
        class.execute_method(sf, &self.method_name, this, args);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      invokeinterface {}.{}{}(<{} arguments>) {}", self.class_name, self.method_name, self.type_desc, self.nb_args, self.count); }
}

pub struct InstrInvokeDynamic {
    method_name: String,
    method_type: String,
    method_nb_args: usize,
    bootstrap_method_idx: usize,
    class_name: String
}
impl ByteCodeInstruction for InstrInvokeDynamic {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let class = get_class(&self.class_name);
        let the_class = class;
        let bootstrap = match the_class.get_bootstrap_method(self.bootstrap_method_idx) {
            Some(bootstrap) => bootstrap,
            _ => panic!("Unknown bootstrap method {}", self.bootstrap_method_idx)
        };

        sf.push_string(self.class_name.clone());
        sf.push_string(self.method_name.clone());
        sf.push_string(self.method_type.clone());

        let mut type_desc = String::from("(III");

        for arg in bootstrap.arguments.iter() {
            sf.push_int(*arg as i32);
            type_desc.push_str("I");
        }
        type_desc.push_str(")V");

        let class = get_class(&bootstrap.class_name);
        let result = class.execute_static_method(sf, &bootstrap.method_name, self.method_nb_args);

        match result {
            MethodCallResult::SUCCESS => InstrNextAction::NEXT,
            MethodCallResult::EXCEPTION(e) => InstrNextAction::EXCEPTION(e)
        }
    }
    fn print(&self) { println!("      invokedynamic {} {}{}", self.bootstrap_method_idx, self.method_name, self.method_type); }
}

pub struct InstrNew { class_name: String }
impl ByteCodeInstruction for InstrNew {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let class = get_class(&self.class_name);
        sf.push(class.new());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      new {}", self.class_name); }
}

pub struct InstrNewArray { atype: u8 }
impl ByteCodeInstruction for InstrNewArray {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let count = sf.pop_int();
        let mut array: Vec<Arc<Mutex<dyn JavaInstance>>> = Vec::with_capacity(count as usize);
        for _i in 0..count {
            array.push(Arc::new(Mutex::new(NativeIntegerInstance::new(0))));
        }
        sf.push_array(Arc::new(Mutex::new(array)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      newarray {}", self.atype); }
}

pub struct InstrANewArray { class_name: String }
impl ByteCodeInstruction for InstrANewArray {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let count = sf.pop_int();
        let mut array: Vec<Arc<Mutex<dyn JavaInstance>>> = Vec::with_capacity(count as usize);
        for _i in 0..count {
            array.push(Arc::new(Mutex::new(NativeIntegerInstance::new(0))));
        }
        sf.push_array(Arc::new(Mutex::new(array)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      anewarray {}", self.class_name); }
}

pub struct InstrArrayLength { }
impl ByteCodeInstruction for InstrArrayLength {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let array = sf.pop_array();
        sf.push_int(array.lock().unwrap().len() as i32);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      arraylength"); }
}

pub struct InstrAThrow { }
impl ByteCodeInstruction for InstrAThrow {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let exception = sf.pop();
        return InstrNextAction::EXCEPTION(exception.clone());
    }
    fn print(&self) { println!("      arraylength"); }
}

///////////// 0xc

pub struct InstrCheckCast { class_name: String }
impl ByteCodeInstruction for InstrCheckCast {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let arg = sf.pop();
        let is_cast_ok = {
            let object = arg.lock().unwrap();
            object.get_class_name().eq(&self.class_name) || object.supports_interface(&self.class_name)
        };
        if is_cast_ok {
            sf.push(arg);
            return InstrNextAction::NEXT;
        } else {
            return exception!("java/lang/ClassCastException", "cannot be cast");
        }
    }
    fn print(&self) { println!("      checkcast"); }
}

pub struct InstrInstanceOf { class_name: String }
impl ByteCodeInstruction for InstrInstanceOf {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let arg = sf.pop();
        sf.push_bool(arg.lock().unwrap().get_class_name().eq(&self.class_name));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      instanceof"); }
}

pub struct InstrIfNull { branch: usize }
impl ByteCodeInstruction for InstrIfNull {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let isnull = sf.pop_isnull();
        if isnull {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ifnull {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrIfNotNull { branch: usize }
impl ByteCodeInstruction for InstrIfNotNull {
    fn execute(&self, sf: &mut StackFrame) -> InstrNextAction {
        let isnull = sf.pop_isnull();
        if !isnull {
            return InstrNextAction::GOTO(self.branch);
        }
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ifnotnull {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////

pub struct Exception {
    pub start_pc: usize,
    pub end_pc: usize,
    pub handler_pc: usize,
    pub name: String
}

impl Exception {
    pub fn catches(&self, exception: Arc<Mutex<dyn JavaInstance>>, instr_idx: usize) -> Option<usize> {
        if self.start_pc > instr_idx || self.end_pc < instr_idx { return None; }

        let mut class_name= exception.lock().unwrap().get_class_name();

        while !class_name.eq("") {
            if class_name.eq(&self.name) {
                return Some(self.handler_pc);
            }

            let class = get_class(&class_name);
            class_name = class.get_parent();
        }

        return None;
    }
}

pub struct ByteCode {
    pub instructions: Vec<Box<dyn ByteCodeInstruction>>,
    pub exceptions: Vec<Exception>,
    address_map: HashMap<usize, usize>,
    pub line_number_table: Vec<(usize, usize)>
}

impl ByteCode {
    pub fn new(data: &mut Blob, constants_class: &HashMap<usize, ConstantClass>,
        _constants_string: &HashMap<usize, ConstantString>,
        constants_string_ref: &HashMap<usize, ConstantStringRef>,
        constants_method: &HashMap<usize, ConstantMethod>,
        constants_field: &HashMap<usize, ConstantField>,
        _constants_name_type: &HashMap<usize, ConstantNameType>,
        constants_dynamic: &HashMap<usize, ConstantInvokeDynamic>,
        constants_integer: &HashMap<usize, ConstantInteger>,
        constants_long: &HashMap<usize, ConstantLong>,
        constants_float: &HashMap<usize, ConstantFloat>,
        constants_double: &HashMap<usize, ConstantDouble>,
        class_name: &String) -> ByteCode {

        let mut instructions: Vec<Box<dyn ByteCodeInstruction>> = Vec::new();
        data.rewind();

        let mut data_offset: usize;
        let mut address_map: HashMap<usize, usize> = HashMap::new();
        let line_number_table: Vec<(usize, usize)> = Vec::new();
        let mut instr_idx: usize = 0;

        while data.has_more_data() {
            data_offset = data.get_offset();
            address_map.insert(data_offset, instr_idx);

            let opcode = data.get_u8();
            let instr: Box<dyn ByteCodeInstruction> = match opcode {
                0x00 => Box::new(InstrNop {}),
                0x01 => Box::new(InstrAConstNull {}),
                0x02 => Box::new(InstrIConst { value:-1 }),
                0x03 => Box::new(InstrIConst { value:0 }),
                0x04 => Box::new(InstrIConst { value:1 }),
                0x05 => Box::new(InstrIConst { value:2 }),
                0x06 => Box::new(InstrIConst { value:3 }),
                0x07 => Box::new(InstrIConst { value:4 }),
                0x08 => Box::new(InstrIConst { value:5 }),
                0x09 => Box::new(InstrLConst0 {}),
                0x0a => Box::new(InstrLConst1 {}),
                0x0b => Box::new(InstrFConst0 {}),
                0x0c => Box::new(InstrFConst1 {}),
                0x0d => Box::new(InstrFConst2 {}),
                0x0e => Box::new(InstrDConst0 {}),
                0x0f => Box::new(InstrDConst1 {}),
                0x10 => Box::new(InstrBiPush { value: data.get_u8() }),
                0x11 => Box::new(InstrSiPush { value: data.get_i16() }),
                0x12 => {
                    let idx = data.get_u8() as usize;
                    match constants_string_ref.get(&idx) {
                        Some(string) => Box::new(InstrLdc { value: Arc::new(Mutex::new(NativeStringInstance::new(string.value.clone()))) }),
                        _ => match constants_float.get(&idx) {
                            Some(float) => Box::new(InstrLdc { value: Arc::new(Mutex::new(NativeFloatInstance::new(float.value))) }),
                            _ => match constants_integer.get(&idx) {
                                Some(int) => Box::new(InstrLdc { value: Arc::new(Mutex::new(NativeIntegerInstance::new(int.value))) }),
                                _ =>  match constants_class.get(&idx) {
                                    Some(class) => Box::new(InstrLdc { value: Arc::new(Mutex::new(JavaClassInstance::new(class.name.clone()))) }),
                                    _ => panic!("ldc: unknown index {}", idx)
                                }
                            }
                        }
                    }
                },
                0x13 => {
                    let idx = data.get_u16size();
                    match constants_string_ref.get(&idx) {
                        Some(string) => Box::new(InstrLdc { value: Arc::new(Mutex::new(NativeStringInstance::new(string.value.clone()))) }),
                        _ => match constants_float.get(&idx) {
                            Some(float) => Box::new(InstrLdc { value: Arc::new(Mutex::new(NativeFloatInstance::new(float.value))) }),
                            _ => match constants_integer.get(&idx) {
                                Some(int) => Box::new(InstrLdc { value: Arc::new(Mutex::new(NativeIntegerInstance::new(int.value))) }),
                                _ =>  panic!("ldc_w: unknown index {}", idx)
                            }
                        }
                    }
                }
                0x14 => {
                    let idx = data.get_u16size();
                    match constants_double.get(&idx) {
                        Some(double) => Box::new(InstrLdc { value: Arc::new(Mutex::new(NativeDoubleInstance::new(double.value))) }),
                        _ => match constants_long.get(&idx) {
                            Some(long) => Box::new(InstrLdc { value: Arc::new(Mutex::new(NativeLongInstance::new(long.value))) }),
                            _ => panic!("ldc2_w: unknown index {}", idx)
                        }
                    }
                },
                0x15 => Box::new(InstrILoad { variable: data.get_u8() }),
                0x16 => Box::new(InstrLLoad { variable: data.get_u8() }),
                0x17 => Box::new(InstrFLoad { variable: data.get_u8() }),
                0x18 => Box::new(InstrDLoad { variable: data.get_u8() }),
                0x19 => Box::new(InstrALoad { variable: data.get_u8() }),
                0x1a => Box::new(InstrILoad { variable: 0 }),
                0x1b => Box::new(InstrILoad { variable: 1 }),
                0x1c => Box::new(InstrILoad { variable: 2 }),
                0x1d => Box::new(InstrILoad { variable: 3 }),
                0x1e => Box::new(InstrLLoad { variable: 0 }),
                0x1f => Box::new(InstrLLoad { variable: 1 }),
                0x20 => Box::new(InstrLLoad { variable: 2 }),
                0x21 => Box::new(InstrLLoad { variable: 3 }),
                0x22 => Box::new(InstrFLoad { variable: 0 }),
                0x23 => Box::new(InstrFLoad { variable: 1 }),
                0x24 => Box::new(InstrFLoad { variable: 2 }),
                0x25 => Box::new(InstrFLoad { variable: 3 }),
                0x26 => Box::new(InstrDLoad { variable: 0 }),
                0x27 => Box::new(InstrDLoad { variable: 1 }),
                0x28 => Box::new(InstrDLoad { variable: 2 }),
                0x29 => Box::new(InstrDLoad { variable: 3 }),
                0x2a => Box::new(InstrALoad { variable: 0 }),
                0x2b => Box::new(InstrALoad { variable: 1 }),
                0x2c => Box::new(InstrALoad { variable: 2 }),
                0x2d => Box::new(InstrALoad { variable: 3 }),
                0x2e => Box::new(InstrIALoad {}),
                0x2f => Box::new(InstrLALoad {}),
                0x30 => Box::new(InstrFALoad {}),
                0x31 => Box::new(InstrDALoad {}),
                0x32 => Box::new(InstrAALoad {}),
                0x33 => Box::new(InstrBALoad {}),
                0x34 => Box::new(InstrCALoad {}),
                0x35 => Box::new(InstrSALoad {}),
                0x36 => Box::new(InstrIStore { variable: data.get_u8() }),
                0x37 => Box::new(InstrLStore { variable: data.get_u8() }),
                0x38 => Box::new(InstrFStore { variable: data.get_u8() }),
                0x39 => Box::new(InstrDStore { variable: data.get_u8() }),
                0x3a => Box::new(InstrAStore { variable: data.get_u8() }),
                0x3b => Box::new(InstrIStore { variable: 0 }),
                0x3c => Box::new(InstrIStore { variable: 1 }),
                0x3d => Box::new(InstrIStore { variable: 2 }),
                0x3e => Box::new(InstrIStore { variable: 3 }),
                0x3f => Box::new(InstrLStore { variable: 0 }),
                0x40 => Box::new(InstrLStore { variable: 1 }),
                0x41 => Box::new(InstrLStore { variable: 2 }),
                0x42 => Box::new(InstrLStore { variable: 3 }),
                0x43 => Box::new(InstrFStore { variable: 0 }),
                0x44 => Box::new(InstrFStore { variable: 1 }),
                0x45 => Box::new(InstrFStore { variable: 2 }),
                0x46 => Box::new(InstrFStore { variable: 3 }),
                0x47 => Box::new(InstrDStore { variable: 0 }),
                0x48 => Box::new(InstrDStore { variable: 1 }),
                0x49 => Box::new(InstrDStore { variable: 2 }),
                0x4a => Box::new(InstrDStore { variable: 3 }),
                0x4b => Box::new(InstrAStore { variable: 0 }),
                0x4c => Box::new(InstrAStore { variable: 1 }),
                0x4d => Box::new(InstrAStore { variable: 2 }),
                0x4e => Box::new(InstrAStore { variable: 3 }),
                0x4f => Box::new(InstrIAStore {}),
                0x50 => Box::new(InstrLAStore {}),
                0x51 => Box::new(InstrFAStore {}),
                0x52 => Box::new(InstrDAStore {}),
                0x53 => Box::new(InstrAAStore {}),
                0x54 => Box::new(InstrBAStore {}),
                0x55 => Box::new(InstrCAStore {}),
                0x56 => Box::new(InstrSAStore {}),
                0x57 => Box::new(InstrPop {}),
                0x58 => Box::new(InstrPop2 {}),
                0x59 => Box::new(InstrDup {}),
                0x5a => Box::new(InstrDupX1 {}),
                0x5b => Box::new(InstrDupX2 {}),
                0x5c => Box::new(InstrDup2 {}),
                0x5d => Box::new(InstrDup2X1 {}),
                0x5e => Box::new(InstrDup2X2 {}),
                0x5f => Box::new(InstrSwap {}),
                0x60 => Box::new(InstrIAdd {}),
                0x61 => Box::new(InstrLAdd {}),
                0x62 => Box::new(InstrFAdd {}),
                0x63 => Box::new(InstrDAdd {}),
                0x64 => Box::new(InstrISub {}),
                0x65 => Box::new(InstrLSub {}),
                0x66 => Box::new(InstrFSub {}),
                0x67 => Box::new(InstrDSub {}),
                0x68 => Box::new(InstrIMul {}),
                0x69 => Box::new(InstrLMul {}),
                0x6a => Box::new(InstrFMul {}),
                0x6b => Box::new(InstrDMul {}),
                0x6c => Box::new(InstrIDiv {}),
                0x6d => Box::new(InstrLDiv {}),
                0x6e => Box::new(InstrFDiv {}),
                0x6f => Box::new(InstrDDiv {}),
                0x70 => Box::new(InstrIRem {}),
                0x71 => Box::new(InstrLRem {}),
                0x72 => Box::new(InstrFRem {}),
                0x73 => Box::new(InstrDRem {}),
                0x74 => Box::new(InstrINeg {}),
                0x75 => Box::new(InstrLNeg {}),
                0x76 => Box::new(InstrFNeg {}),
                0x77 => Box::new(InstrDNeg {}),
                0x78 => Box::new(InstrIShl {}),
                0x79 => Box::new(InstrLShl {}),
                0x7a => Box::new(InstrIShr {}),
                0x7b => Box::new(InstrLShr {}),
                0x7c => Box::new(InstrIUShr {}),
                0x7d => Box::new(InstrLUShr {}),
                0x7e => Box::new(InstrIAnd {}),
                0x7f => Box::new(InstrLAnd {}),
                0x80 => Box::new(InstrIOr {}),
                0x81 => Box::new(InstrLOr {}),
                0x82 => Box::new(InstrIXor {}),
                0x83 => Box::new(InstrLXor {}),
                0x84 => Box::new(InstrIInc { idx: data.get_u8(), count: data.get_i8() }),
                0x85 => Box::new(InstrI2L {}),
                0x86 => Box::new(InstrI2F {}),
                0x87 => Box::new(InstrI2D {}),
                0x88 => Box::new(InstrL2I {}),
                0x89 => Box::new(InstrL2F {}),
                0x8a => Box::new(InstrL2D {}),
                0x8b => Box::new(InstrF2I {}),
                0x8c => Box::new(InstrF2L {}),
                0x8d => Box::new(InstrF2D {}),
                0x8e => Box::new(InstrD2I {}),
                0x8f => Box::new(InstrD2L {}),
                0x90 => Box::new(InstrD2F {}),
                0x91 => Box::new(InstrI2B {}),
                0x92 => Box::new(InstrI2C {}),
                0x93 => Box::new(InstrI2C {}),
                0x94 => Box::new(InstrLCmp {}),
                0x95 => Box::new(InstrFCmpl {}),
                0x96 => Box::new(InstrFCmpg {}),
                0x97 => Box::new(InstrDCmpl {}),
                0x98 => Box::new(InstrDCmpg {}),
                0x99 => Box::new(InstrIfeq { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0x9a => Box::new(InstrIfne { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0x9b => Box::new(InstrIflt { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0x9c => Box::new(InstrIfge { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0x9d => Box::new(InstrIfgt { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0x9e => Box::new(InstrIfle { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0x9f => Box::new(InstrIfICmpEq { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0xa0 => Box::new(InstrIfICmpNe { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0xa1 => Box::new(InstrIfICmpLt { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0xa2 => Box::new(InstrIfICmpGe { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0xa3 => Box::new(InstrIfICmpGt { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0xa4 => Box::new(InstrIfICmpLe { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0xa5 => Box::new(InstrIfACmpEq { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0xa6 => Box::new(InstrIfACmpNe { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0xa7 => Box::new(InstrGoto { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0xaa => {
                    let offset = data_offset;
                    for _ in 0..((4 - (offset + 1) % 4) % 4) {
                        data.get_u8();
                    }
                    let default = offset + data.get_u32size();
                    let low = data.get_u32size();
                    let high = data.get_u32size();
                    let nb_jumps = high - low + 1;
                    let mut jumps: Vec<usize> = Vec::new();
                    for _ in 0..nb_jumps {
                        let jump = data.get_u32size() + offset;
                        jumps.push(jump);
                    }
                    Box::new(InstrTableSwitch { default, low, table: jumps })
                },
                0xab => {
                    let offset = data_offset;
                    for _ in 0..((4 - (offset + 1) % 4) % 4) {
                        data.get_u8();
                    }
                    let default = offset + data.get_u32size();
                    let nb_pairs = data.get_u32size();
                    let mut pairs: HashMap<i32, usize> = HashMap::new();
                    for _ in 0..nb_pairs {
                        let value = data.get_u32size();
                        let goto = offset + data.get_u32size();
                        pairs.insert(value as i32, goto);
                    }
                    Box::new(InstrLookupSwitch { default, lookup: pairs })
                },
                0xac => Box::new(InstrIReturn {}),
                0xad => Box::new(InstrLReturn {}),
                0xae => Box::new(InstrFReturn {}),
                0xaf => Box::new(InstrDReturn {}),
                0xb0 => Box::new(InstrAReturn {}),
                0xb1 => Box::new(InstrReturn {}),
                0xb2 => match constants_field.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrGetStatic {
                        class_name: method.class_name.clone(),
                        field_name: method.field_name.clone(),
                        type_desc: method.type_name.clone()
                    }),
                    _ => panic!("Unknown field")
                },
                0xb3 => match constants_field.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrPutStatic {
                        class_name: method.class_name.clone(),
                        field_name: method.field_name.clone(),
                        type_desc: method.type_name.clone()
                    }),
                    _ => panic!("Unknown field")
                },
                0xb4 => match constants_field.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrGetField {
                        class_name: method.class_name.clone(),
                        field_name: method.field_name.clone()
                    }),
                    _ => panic!("Unknown field")
                },
                0xb5 => match constants_field.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrPutField {
                        class_name: method.class_name.clone(),
                        field_name: method.field_name.clone()
                    }),
                    _ => panic!("Unknown field")
                },
                0xb6 => match constants_method.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrInvokeVirtual {
                        class_name: method.class_name.clone(),
                        method_name: method.method_name.clone(),
                        type_desc: method.type_name.clone(),
                        nb_args: get_nb_arguments(&method.type_name)
                    }),
                    _ => panic!("Unknown method")
                },
                0xb7 => match constants_method.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrInvokeSpecial {
                        class_name: method.class_name.clone(),
                        method_name: method.method_name.clone(),
                        type_desc: method.type_name.clone(),
                        nb_args: get_nb_arguments(&method.type_name)
                    }),
                    _ => panic!("Unknown method")
                },
                0xb8 => match constants_method.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrInvokeStatic {
                        class_name: method.class_name.clone(),
                        method_name: method.method_name.clone(),
                        type_desc: method.type_name.clone(),
                        nb_args: get_nb_arguments(&method.type_name)
                    }),
                    _ => panic!("Unknown method")
                },
                0xb9 => match constants_method.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrInvokeInterface {
                        count: data.get_u16size(),
                        class_name: method.class_name.clone(),
                        method_name: method.method_name.clone(),
                        type_desc: method.type_name.clone(),
                        nb_args: get_nb_arguments(&method.type_name)
                    }),
                    _ => panic!("Unknown interface")
                },
                0xba => {
                    let type_name = data.get_u16size();
                    data.get_u16size();

                    match constants_dynamic.get(&type_name) {
                        Some(dynamic) => Box::new(InstrInvokeDynamic {
                            bootstrap_method_idx: dynamic.idx_bootstrap_method,
                            class_name: class_name.clone(),
                            method_name: dynamic.method_name.clone(),
                            method_type: dynamic.type_name.clone(),
                            method_nb_args: get_nb_arguments(&dynamic.type_name)
                        }),
                        _ => panic!("Unknown name/type {}", type_name)
                    }
                },
                0xbb => match constants_class.get(&data.get_u16size()) {
                    Some(class) => Box::new(InstrNew {
                        class_name: class.name.clone()
                    }),
                    _ => panic!("Unknown class")
                },
                0xbc => Box::new(InstrNewArray { atype: data.get_u8() }),
                0xbd => match constants_class.get(&data.get_u16size()) {
                    Some(class) => Box::new(InstrANewArray {
                        class_name: class.name.clone()
                    }),
                    _ => panic!("Unknown class")
                },
                0xbe => Box::new(InstrArrayLength {}),
                0xbf => Box::new(InstrAThrow {}),
                0xc0 => {
                    let idx = data.get_u16size();
                    match constants_class.get(&idx) {
                        Some(class) => {
                            let class_name = if class.name.starts_with("[") {
                                "java/util/Arrays".to_string()
                            } else {
                                class.name.clone()
                            };
                            Box::new(InstrCheckCast { class_name: class_name })
                        },
                        _ => panic!("Unknown class at index {}", idx)
                    }
                },
                0xc1 => {
                    let idx = data.get_u16size();
                    match constants_class.get(&idx) {
                        Some(class) => Box::new(InstrInstanceOf { class_name: class.name.clone() }),
                        _ => panic!("Unknown class at index {}", idx)
                    }
                },
//                0xc2 => monitorenter
//                0xc3 => monitorexit
//                0xc4 => wide
//                0xc5 => multianewarray
                0xc6 => Box::new(InstrIfNull { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0xc7 => Box::new(InstrIfNotNull { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0xc8 => Box::new(InstrGoto { branch: (data_offset as i32 + data.get_i32()) as usize }),
                _ => panic!("Unknown opcode {:#02x}", opcode)
            };

            if get_debug() >= 2 { instr.print(); }
            instructions.push(instr);

            instr_idx += 1;
        }

        for instr in instructions.iter_mut() {
            instr.set_branch(&address_map);
        }

        ByteCode {
            instructions,
            exceptions: Vec::new(),
            address_map,
            line_number_table
        }
    }

    pub fn add_exception(&mut self, start_pc: usize, end_pc: usize, handler_pc: usize, name: &String) {
        self.exceptions.push(Exception {
            start_pc: *self.address_map.get(&start_pc).unwrap(),
            end_pc: *self.address_map.get(&end_pc).unwrap(),
            handler_pc: *self.address_map.get(&handler_pc).unwrap(),
            name: name.clone()
        });

        if get_debug() >= 2 {
            println!("    Method exception [{}..{}] -> {}, type={}",
                *self.address_map.get(&start_pc).unwrap(),
                *self.address_map.get(&end_pc).unwrap(),
                *self.address_map.get(&handler_pc).unwrap(),
                name);
        }
    }

    pub fn add_line_number(&mut self, line: usize, bytecode_offset: usize) {
        let instruction = match self.address_map.get(&bytecode_offset) {
            Some(instr) => *instr,
            None => panic!("Cannot find instruction at bytecode offset {}", bytecode_offset)
        };
        if get_debug() >= 2 { println!("        Instr {} => Line {}", instruction, line); }

        self.line_number_table.push((instruction, line));
    }
}
