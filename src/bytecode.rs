use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::java_class::{ConstantField, ConstantFloat, ConstantInteger, ConstantLong, ConstantDouble};
use crate::java_class::ConstantString;
use crate::java_class::ConstantStringRef;
use crate::java_class::ConstantClass;
use crate::java_class::ConstantMethod;
use crate::java_class::ConstantNameType;
use crate::java_class::ConstantInvokeDynamic;
use crate::JVM;
use crate::java_class::Blob;
use crate::java_class::BytecodeClass;
use crate::jvm::JavaObject;
use crate::Classes;

pub trait ByteCodeInstruction {
    fn execute(&self, class: &BytecodeClass, jvm: &mut JVM, classes: &Classes) -> InstrNextAction;
    fn print(&self);
    fn set_branch(&mut self, _address_map: &HashMap<usize, usize>) {}
}

pub enum InstrNextAction {
    NEXT,
    RETURN,
    GOTO(usize)
}

////////////////////////////////////////////////////////////////////////////////////
///////////// 0x0

pub struct InstrNop { }
impl ByteCodeInstruction for InstrNop {
    fn execute(&self, _class: &BytecodeClass, _jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      nop"); }
}

pub struct InstrIConst { value: i32 }
impl ByteCodeInstruction for InstrIConst {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::INTEGER(self.value)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iconst_{}", self.value); }
}

pub struct InstrLConst0 { }
impl ByteCodeInstruction for InstrLConst0 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::LONG(0)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lconst_0"); }
}

pub struct InstrLConst1 { }
impl ByteCodeInstruction for InstrLConst1 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::LONG(1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lconst_1"); }
}

pub struct InstrFConst0 { }
impl ByteCodeInstruction for InstrFConst0 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::FLOAT(0.0)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fconst_0"); }
}

pub struct InstrFConst1 { }
impl ByteCodeInstruction for InstrFConst1 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::FLOAT(1.0)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fconst_1"); }
}

pub struct InstrFConst2 { }
impl ByteCodeInstruction for InstrFConst2 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::FLOAT(2.0)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fconst_2"); }
}

pub struct InstrDConst0 { }
impl ByteCodeInstruction for InstrDConst0 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::DOUBLE(0.0)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dconst_0"); }
}

pub struct InstrDConst1 { }
impl ByteCodeInstruction for InstrDConst1 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::DOUBLE(1.0)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dconst_1"); }
}

///////////// 0x1

pub struct InstrBiPush { value: u8 }
impl ByteCodeInstruction for InstrBiPush {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::INTEGER(self.value as i32)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      bipush {}", self.value); }
}

pub struct InstrILoad { value: u8 }
impl ByteCodeInstruction for InstrILoad {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(jvm.variables[self.value as usize].clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iload {}", self.value); }
}

pub struct InstrLLoad { value: u8 }
impl ByteCodeInstruction for InstrLLoad {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(jvm.variables[self.value as usize].clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lload {}", self.value); }
}

pub struct InstrFLoad { value: u8 }
impl ByteCodeInstruction for InstrFLoad {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(jvm.variables[self.value as usize].clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fload {}", self.value); }
}

pub struct InstrDLoad { value: u8 }
impl ByteCodeInstruction for InstrDLoad {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(jvm.variables[self.value as usize].clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dload {}", self.value); }
}

pub struct InstrALoad { value: u8 }
impl ByteCodeInstruction for InstrALoad {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(jvm.variables[self.value as usize].clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      aload {}", self.value); }
}

pub struct InstrLdc { value: Rc<JavaObject> }
impl ByteCodeInstruction for InstrLdc {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(self.value.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ldc"); }
}

pub struct InstrLdcF { value: f32 }
impl ByteCodeInstruction for InstrLdcF {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::FLOAT(self.value)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ldc {}", self.value); }
}

pub struct InstrILoadN { variable: u8 }
impl ByteCodeInstruction for InstrILoadN {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(jvm.variables[self.variable as usize].clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iload_{}", self.variable); }
}

pub struct InstrLLoadN { variable: u8 }
impl ByteCodeInstruction for InstrLLoadN {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(jvm.variables[self.variable as usize].clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lload_{}", self.variable); }
}

///////////// 0x2

pub struct InstrFLoadN { variable: u8 }
impl ByteCodeInstruction for InstrFLoadN {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(jvm.variables[self.variable as usize].clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fload_{}", self.variable); }
}

pub struct InstrDLoadN { variable: u8 }
impl ByteCodeInstruction for InstrDLoadN {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(jvm.variables[self.variable as usize].clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dload_{}", self.variable); }
}

pub struct InstrALoadN { variable: u8 }
impl ByteCodeInstruction for InstrALoadN {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(jvm.variables[self.variable as usize].clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      aload{}", self.variable); }
}

///////////// 0x3

pub struct InstrAALoad {}
impl ByteCodeInstruction for InstrAALoad {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let idx = jvm.pop_int();
        let val = jvm.pop();
        let array = match &*val {
            JavaObject::ARRAY(array) => array.borrow(),
            _ => panic!("Unknown object in the stack. Was expecting an array")
        };
        let object: &Rc<JavaObject> = match array.get(idx as usize) {
            Some(obj) => obj,
            _ => panic!("No object in the array at index {}", idx)
        };
        jvm.push(object.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      aaload"); }
}

pub struct InstrIStore { value: u8 }
impl ByteCodeInstruction for InstrIStore {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[self.value as usize] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      istore {}", self.value); }
}

pub struct InstrAStore { value: u8 }
impl ByteCodeInstruction for InstrAStore {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[self.value as usize] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      astore {}", self.value); }
}

pub struct InstrFStore { value: u8 }
impl ByteCodeInstruction for InstrFStore {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[self.value as usize] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fstore {}", self.value); }
}

pub struct InstrIStore0 {}
impl ByteCodeInstruction for InstrIStore0 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[0] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      istore_0"); }
}

pub struct InstrIStore1 {}
impl ByteCodeInstruction for InstrIStore1 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[1] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      istore_1"); }
}

pub struct InstrIStore2 {}
impl ByteCodeInstruction for InstrIStore2 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[2] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      istore_2"); }
}

pub struct InstrIStore3 {}
impl ByteCodeInstruction for InstrIStore3 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[3] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      istore_3"); }
}

///////////// 0x4

pub struct InstrFStore0 {}
impl ByteCodeInstruction for InstrFStore0 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[0] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fstore_0"); }
}

pub struct InstrFStore1 {}
impl ByteCodeInstruction for InstrFStore1 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[1] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fstore_1"); }
}

pub struct InstrFStore2 {}
impl ByteCodeInstruction for InstrFStore2 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[2] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fstore_2"); }
}

pub struct InstrFStore3 {}
impl ByteCodeInstruction for InstrFStore3 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[3] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fstore_3"); }
}

pub struct InstrAStore0 {}
impl ByteCodeInstruction for InstrAStore0 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[0] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      astore_0"); }
}

pub struct InstrAStore1 {}
impl ByteCodeInstruction for InstrAStore1 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[1] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      astore_1"); }
}

pub struct InstrAStore2 {}
impl ByteCodeInstruction for InstrAStore2 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[2] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      astore_2"); }
}

pub struct InstrAStore3 {}
impl ByteCodeInstruction for InstrAStore3 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.variables[3] = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      astore_3"); }
}

///////////// 0x5

pub struct InstrAAStore {}
impl ByteCodeInstruction for InstrAAStore {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let object = jvm.pop();
        let idx = jvm.pop_int();
        let arg = jvm.pop();
        let array = match &*arg {
            JavaObject::ARRAY(array) => array,
            _ => panic!("Excepted array in the stack")
        };
        array.borrow_mut()[idx as usize] = object;
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      aastore"); }
}

pub struct InstrDup { }
impl ByteCodeInstruction for InstrDup {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let arg = jvm.pop();
        jvm.push(arg.clone());
        jvm.push(arg.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dup"); }
}

///////////// 0x6

pub struct InstrIAdd {}
impl ByteCodeInstruction for InstrIAdd {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_int();
        let nb2 = jvm.pop_int();
        jvm.push(Rc::new(JavaObject::INTEGER(nb2 + nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iadd"); }
}

pub struct InstrLAdd {}
impl ByteCodeInstruction for InstrLAdd {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_long();
        let nb2 = jvm.pop_long();
        jvm.push(Rc::new(JavaObject::LONG(nb2 + nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ladd"); }
}

pub struct InstrFAdd {}
impl ByteCodeInstruction for InstrFAdd {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_float();
        let nb2 = jvm.pop_float();
        jvm.push(Rc::new(JavaObject::FLOAT(nb2 + nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fadd"); }
}

pub struct InstrDAdd {}
impl ByteCodeInstruction for InstrDAdd {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_double();
        let nb2 = jvm.pop_double();
        jvm.push(Rc::new(JavaObject::DOUBLE(nb2 + nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dadd"); }
}

pub struct InstrISub {}
impl ByteCodeInstruction for InstrISub {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_int();
        let nb2 = jvm.pop_int();
        jvm.push(Rc::new(JavaObject::INTEGER(nb2 - nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      isub"); }
}

pub struct InstrLSub {}
impl ByteCodeInstruction for InstrLSub {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_long();
        let nb2 = jvm.pop_long();
        jvm.push(Rc::new(JavaObject::LONG(nb2 - nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lsub"); }
}

pub struct InstrFSub {}
impl ByteCodeInstruction for InstrFSub {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_float();
        let nb2 = jvm.pop_float();
        jvm.push(Rc::new(JavaObject::FLOAT(nb2 - nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fsub"); }
}

pub struct InstrDSub {}
impl ByteCodeInstruction for InstrDSub {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_double();
        let nb2 = jvm.pop_double();
        jvm.push(Rc::new(JavaObject::DOUBLE(nb2 - nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dsub"); }
}

pub struct InstrIMul {}
impl ByteCodeInstruction for InstrIMul {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_int();
        let nb2 = jvm.pop_int();
        jvm.push(Rc::new(JavaObject::INTEGER(nb2 * nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      imul"); }
}

pub struct InstrLMul {}
impl ByteCodeInstruction for InstrLMul {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_long();
        let nb2 = jvm.pop_long();
        jvm.push(Rc::new(JavaObject::LONG(nb2 * nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lmul"); }
}

pub struct InstrFMul {}
impl ByteCodeInstruction for InstrFMul {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_float();
        let nb2 = jvm.pop_float();
        jvm.push(Rc::new(JavaObject::FLOAT(nb2 * nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fmul"); }
}

pub struct InstrDMul {}
impl ByteCodeInstruction for InstrDMul {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_double();
        let nb2 = jvm.pop_double();
        jvm.push(Rc::new(JavaObject::DOUBLE(nb2 * nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dmul"); }
}

pub struct InstrIDiv {}
impl ByteCodeInstruction for InstrIDiv {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_int();
        let nb2 = jvm.pop_int();
        jvm.push(Rc::new(JavaObject::INTEGER(nb2 / nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      idiv"); }
}

pub struct InstrLDiv {}
impl ByteCodeInstruction for InstrLDiv {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_long();
        let nb2 = jvm.pop_long();
        jvm.push(Rc::new(JavaObject::LONG(nb2 / nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ldiv"); }
}

pub struct InstrFDiv {}
impl ByteCodeInstruction for InstrFDiv {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_float();
        let nb2 = jvm.pop_float();
        jvm.push(Rc::new(JavaObject::FLOAT(nb2 / nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fdiv"); }
}

pub struct InstrDDiv {}
impl ByteCodeInstruction for InstrDDiv {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_double();
        let nb2 = jvm.pop_double();
        jvm.push(Rc::new(JavaObject::DOUBLE(nb2 / nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ddiv"); }
}

///////////// 0x7

pub struct InstrIRem {}
impl ByteCodeInstruction for InstrIRem {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_int();
        let nb2 = jvm.pop_int();
        jvm.push(Rc::new(JavaObject::INTEGER(nb2 % nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      irem"); }
}

pub struct InstrLRem {}
impl ByteCodeInstruction for InstrLRem {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_long();
        let nb2 = jvm.pop_long();
        jvm.push(Rc::new(JavaObject::LONG(nb2 % nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lrem"); }
}

pub struct InstrFRem {}
impl ByteCodeInstruction for InstrFRem {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_float();
        let nb2 = jvm.pop_float();
        jvm.push(Rc::new(JavaObject::FLOAT(nb2 % nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      frem"); }
}

pub struct InstrDRem {}
impl ByteCodeInstruction for InstrDRem {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_double();
        let nb2 = jvm.pop_double();
        jvm.push(Rc::new(JavaObject::DOUBLE(nb2 % nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      drem"); }
}

pub struct InstrINeg {}
impl ByteCodeInstruction for InstrINeg {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb = jvm.pop_int();
        jvm.push(Rc::new(JavaObject::INTEGER(-nb)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ineg"); }
}

pub struct InstrLNeg {}
impl ByteCodeInstruction for InstrLNeg {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb = jvm.pop_long();
        jvm.push(Rc::new(JavaObject::LONG(-nb)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lneg"); }
}

pub struct InstrFNeg {}
impl ByteCodeInstruction for InstrFNeg {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb = jvm.pop_float();
        jvm.push(Rc::new(JavaObject::FLOAT(-nb)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fneg"); }
}

pub struct InstrDNeg {}
impl ByteCodeInstruction for InstrDNeg {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb = jvm.pop_double();
        jvm.push(Rc::new(JavaObject::DOUBLE(-nb)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dneg"); }
}

pub struct InstrIAnd {}
impl ByteCodeInstruction for InstrIAnd {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_int();
        let nb2 = jvm.pop_int();
        jvm.push(Rc::new(JavaObject::INTEGER(nb2 & nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iand"); }
}

pub struct InstrLAnd {}
impl ByteCodeInstruction for InstrLAnd {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_long();
        let nb2 = jvm.pop_long();
        jvm.push(Rc::new(JavaObject::LONG(nb2 & nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      land"); }
}

///////////// 0x8

pub struct InstrIOr {}
impl ByteCodeInstruction for InstrIOr {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_int();
        let nb2 = jvm.pop_int();
        jvm.push(Rc::new(JavaObject::INTEGER(nb2 & nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ior"); }
}

pub struct InstrLOr {}
impl ByteCodeInstruction for InstrLOr {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_long();
        let nb2 = jvm.pop_long();
        jvm.push(Rc::new(JavaObject::LONG(nb2 | nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lor"); }
}

pub struct InstrIXor {}
impl ByteCodeInstruction for InstrIXor {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_int();
        let nb2 = jvm.pop_int();
        jvm.push(Rc::new(JavaObject::INTEGER(nb2 ^ nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ixor"); }
}

pub struct InstrLXor {}
impl ByteCodeInstruction for InstrLXor {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_long();
        let nb2 = jvm.pop_long();
        jvm.push(Rc::new(JavaObject::LONG(nb2 ^ nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      lxor"); }
}

pub struct InstrIInc { idx: u8, count: i8 }
impl ByteCodeInstruction for InstrIInc {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let variable = jvm.variables[self.idx as usize].clone();

        let nb = match &*variable {
            JavaObject::INTEGER(int) => *int,
            _ => panic!("iinc expects variable {} to contain an integer", self.idx)
        };

        jvm.variables[self.idx as usize] = Rc::new(JavaObject::INTEGER(nb + self.count as i32));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iinc {} {}", self.idx, self.count); }
}

pub struct InstrI2F {}
impl ByteCodeInstruction for InstrI2F {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb = jvm.pop_int();
        jvm.push(Rc::new(JavaObject::FLOAT(nb as f32)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      i2f"); }
}

pub struct InstrF2D {}
impl ByteCodeInstruction for InstrF2D {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb = jvm.pop_float();
        jvm.push(Rc::new(JavaObject::DOUBLE(nb as f64)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      f2d"); }
}

///////////// 0x9

pub struct InstrD2F {}
impl ByteCodeInstruction for InstrD2F {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb = jvm.pop_double();
        jvm.push(Rc::new(JavaObject::FLOAT(nb as f32)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      2df"); }
}

pub struct InstrFCmpl {}
impl ByteCodeInstruction for InstrFCmpl {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb2 = jvm.pop_float();
        let nb1 = jvm.pop_float();
        let mut result: i32 = 0;
        if nb1 == f32::NAN || nb2 == f32::NAN {
            result = -1;
        } else if nb1 > nb2 {
            result = 1;
        } else if nb1 < nb2 {
            result = -1;
        }

        jvm.push(Rc::new(JavaObject::INTEGER(result)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fcmpl"); }
}

pub struct InstrFCmpg {}
impl ByteCodeInstruction for InstrFCmpg {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb2 = jvm.pop_float();
        let nb1 = jvm.pop_float();
        let mut result: i32 = 0;
        if nb1 == f32::NAN || nb2 == f32::NAN {
            result = 1;
        } else if nb1 > nb2 {
            result = 1;
        } else if nb1 < nb2 {
            result = -1;
        }

        jvm.push(Rc::new(JavaObject::INTEGER(result)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      fcmpg"); }
}

pub struct InstrIfeq { branch: usize }
impl ByteCodeInstruction for InstrIfeq {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let arg = jvm.pop();
        match &*arg {
            JavaObject::INTEGER(int) => {
                if *int == 0 {
                    return InstrNextAction::GOTO(self.branch);
                }
            },
            _ => panic!("ifeq expects an integer in the stack")
        };
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
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let arg = jvm.pop();
        match &*arg {
            JavaObject::INTEGER(int) => {
                if *int != 0 {
                    return InstrNextAction::GOTO(self.branch);
                }
            },
            _ => panic!("ifne expects an integer in the stack")
        };
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
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let arg = jvm.pop();
        match &*arg {
            JavaObject::INTEGER(int) => {
                if *int < 0 {
                    return InstrNextAction::GOTO(self.branch);
                }
            },
            _ => panic!("iflt expects an integer in the stack")
        };
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
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let arg = jvm.pop();
        match &*arg {
            JavaObject::INTEGER(int) => {
                if *int >= 0 {
                    return InstrNextAction::GOTO(self.branch);
                }
            },
            _ => panic!("ifge expects an integer in the stack")
        };
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
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let arg = jvm.pop();
        match &*arg {
            JavaObject::INTEGER(int) => {
                if *int > 0 {
                    return InstrNextAction::GOTO(self.branch);
                }
            },
            _ => panic!("ifgt expects an integer in the stack")
        };
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
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let arg = jvm.pop();
        match &*arg {
            JavaObject::INTEGER(int) => {
                if *int <= 0 {
                    return InstrNextAction::GOTO(self.branch);
                }
            },
            _ => panic!("ifle expects an integer in the stack")
        };
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
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let value2 = jvm.pop_int();
        let value1 = jvm.pop_int();
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
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let value2 = jvm.pop_int();
        let value1 = jvm.pop_int();
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
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let value2 = jvm.pop_int();
        let value1 = jvm.pop_int();
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
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let value2 = jvm.pop_int();
        let value1 = jvm.pop_int();
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
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let value2 = jvm.pop_int();
        let value1 = jvm.pop_int();
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
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let value2 = jvm.pop_int();
        let value1 = jvm.pop_int();
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

pub struct InstrGoto { branch: usize }
impl ByteCodeInstruction for InstrGoto {
    fn execute(&self, _class: &BytecodeClass, _jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
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

pub struct InstrIReturn {}
impl ByteCodeInstruction for InstrIReturn {
    fn execute(&self, _class: &BytecodeClass, _jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        return InstrNextAction::RETURN;
    }
    fn print(&self) { println!("      ireturn"); }
}

///////////// 0xb

pub struct InstrAReturn {}
impl ByteCodeInstruction for InstrAReturn {
    fn execute(&self, _class: &BytecodeClass, _jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        return InstrNextAction::RETURN;
    }
    fn print(&self) { println!("      areturn"); }
}

pub struct InstrReturn {}
impl ByteCodeInstruction for InstrReturn {
    fn execute(&self, _class: &BytecodeClass, _jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        return InstrNextAction::RETURN;
    }
    fn print(&self) { println!("      return"); }
}

pub struct InstrGetStatic { class_name: String, field_name: String, type_desc: String }
impl ByteCodeInstruction for InstrGetStatic {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, classes: &Classes) -> InstrNextAction {
        let class = classes.get_class(&self.class_name);
        jvm.push(Rc::new(class.get_static_object(&self.field_name)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      getstatic {}.{} -> {}", self.class_name, self.field_name, self.type_desc); }
}

pub struct InstrInvokeVirtual { class_name: String, method_name: String, type_desc: String }
impl ByteCodeInstruction for InstrInvokeVirtual {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, classes: &Classes) -> InstrNextAction {
        let class = classes.get_class(&self.class_name);
        class.execute_method(jvm, classes, &self.method_name);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      invokevirtual {}.{}() -> {}", self.class_name, self.method_name, self.type_desc); }
}

pub struct InstrInvokeSpecial { class_name: String, method_name: String, type_desc: String }
impl ByteCodeInstruction for InstrInvokeSpecial {
    fn execute(&self, _class: &BytecodeClass, _jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      invokespecial {}.{}() -> {}", self.class_name, self.method_name, self.type_desc); }
}

pub struct InstrInvokeStatic { class_name: String, method_name: String, type_desc: String }
impl ByteCodeInstruction for InstrInvokeStatic {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, classes: &Classes) -> InstrNextAction {
        let class = classes.get_class(&self.class_name);
        class.execute_static_method(jvm, classes, &self.method_name);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      invokestatic {}.{}() -> {}", self.class_name, self.method_name, self.type_desc); }
}

pub struct InstrInvokeInterface { class_name: String, method_name: String, type_desc: String, count: usize }
impl ByteCodeInstruction for InstrInvokeInterface {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, classes: &Classes) -> InstrNextAction {
        let class = classes.get_class(&self.class_name);
        class.execute_method(jvm, classes, &self.method_name);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      invokeinterface {}.{}() -> {} {}", self.class_name, self.method_name, self.type_desc, self.count); }
}

pub struct InstrInvokeDynamic {
    method_name: String,
    method_type: String,
    bootstrap_method_idx: usize
}
impl ByteCodeInstruction for InstrInvokeDynamic {
    fn execute(&self, class: &BytecodeClass, jvm: &mut JVM, classes: &Classes) -> InstrNextAction {

        let bootstrap = match class.bootstrap_methods.get(self.bootstrap_method_idx) {
            Some(bootstrap) => bootstrap,
            _ => panic!("Unknown bootstrap mehtod {}", self.bootstrap_method_idx)
        };

        jvm.push(Rc::new(JavaObject::STRING(class.name.clone())));
        jvm.push(Rc::new(JavaObject::STRING(self.method_name.clone())));
        jvm.push(Rc::new(JavaObject::STRING(self.method_type.clone())));
        for arg in bootstrap.arguments.iter() {
            jvm.push(Rc::new(JavaObject::INTEGER(*arg as i32)));
        }

        let class = classes.get_class(&bootstrap.class_name);
        class.execute_static_method(jvm, classes, &bootstrap.method_name);

        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      invokedynamic {} {} {}", self.bootstrap_method_idx, self.method_name, self.method_type); }
}

pub struct InstrANewArray { class_name: String }
impl ByteCodeInstruction for InstrANewArray {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let count = jvm.pop_int();
        let mut array: Vec<Rc<JavaObject>> = Vec::with_capacity(count as usize);
        for _i in 0..count {
            array.push(Rc::new(JavaObject::INTEGER(0)));
        }
        jvm.push(Rc::new(JavaObject::ARRAY(RefCell::new(array))));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      anewarray {}", self.class_name); }
}

///////////// 0xc

////////////////////////////////////////////////////////////////////////////////////

pub struct ByteCode {
    pub instructions: Vec<Box<dyn ByteCodeInstruction>>
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
        debug:u8) -> ByteCode {

        let mut instructions: Vec<Box<dyn ByteCodeInstruction>> = Vec::new();
        data.rewind();

        let mut data_offset: usize;
        let mut address_map: HashMap<usize, usize> = HashMap::new();
        let mut instr_idx: usize = 0;

        while data.has_more_data() {
            data_offset = data.get_offset();
            address_map.insert(data_offset, instr_idx);

            let opcode = data.get_u8();
            let instr: Box<dyn ByteCodeInstruction> = match opcode {
                0x00 => Box::new(InstrNop {}),
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
                0x12 => {
                    let idx = data.get_u8() as usize;
                    match constants_string_ref.get(&idx) {
                        Some(string) => Box::new(InstrLdc { value: Rc::new(JavaObject::STRING(string.value.clone())) }),
                        _ => match constants_float.get(&idx) {
                            Some(float) => Box::new(InstrLdc { value: Rc::new(JavaObject::FLOAT(float.value)) }),
                            _ => match constants_integer.get(&idx) {
                                Some(int) => Box::new(InstrLdc { value: Rc::new(JavaObject::INTEGER(int.value)) }),
                                _ =>  panic!("ldc: unknown index {}", idx)
                            }
                        }
                    }
                },
                0x13 => {
                    let idx = data.get_u16size();
                    match constants_string_ref.get(&idx) {
                        Some(string) => Box::new(InstrLdc { value: Rc::new(JavaObject::STRING(string.value.clone())) }),
                        _ => match constants_float.get(&idx) {
                            Some(float) => Box::new(InstrLdc { value: Rc::new(JavaObject::FLOAT(float.value)) }),
                            _ => match constants_integer.get(&idx) {
                                Some(int) => Box::new(InstrLdc { value: Rc::new(JavaObject::INTEGER(int.value)) }),
                                _ =>  panic!("ldc_w: unknown index {}", idx)
                            }
                        }
                    }
                }
                0x14 => {
                    let idx = data.get_u16size();
                    match constants_double.get(&idx) {
                        Some(double) => Box::new(InstrLdc { value: Rc::new(JavaObject::DOUBLE(double.value)) }),
                        _ => match constants_long.get(&idx) {
                            Some(long) => Box::new(InstrLdc { value: Rc::new(JavaObject::LONG(long.value)) }),
                            _ => panic!("ldc2_w: unknown index {}", idx)
                        }
                    }
                },
                0x15 => Box::new(InstrILoad { value: data.get_u8() }),
                0x16 => Box::new(InstrLLoad { value: data.get_u8() }),
                0x17 => Box::new(InstrFLoad { value: data.get_u8() }),
                0x18 => Box::new(InstrDLoad { value: data.get_u8() }),
                0x19 => Box::new(InstrALoad { value: data.get_u8() }),
                0x1a => Box::new(InstrILoadN { variable: 0 }),
                0x1b => Box::new(InstrILoadN { variable: 1 }),
                0x1c => Box::new(InstrILoadN { variable: 2 }),
                0x1d => Box::new(InstrILoadN { variable: 3 }),
                0x1e => Box::new(InstrLLoadN { variable: 0 }),
                0x1f => Box::new(InstrLLoadN { variable: 1 }),
                0x20 => Box::new(InstrLLoadN { variable: 2 }),
                0x21 => Box::new(InstrLLoadN { variable: 3 }),
                0x22 => Box::new(InstrFLoadN { variable: 0 }),
                0x23 => Box::new(InstrFLoadN { variable: 1 }),
                0x24 => Box::new(InstrFLoadN { variable: 2 }),
                0x25 => Box::new(InstrFLoadN { variable: 3 }),
                0x26 => Box::new(InstrDLoadN { variable: 0 }),
                0x27 => Box::new(InstrDLoadN { variable: 1 }),
                0x28 => Box::new(InstrDLoadN { variable: 2 }),
                0x29 => Box::new(InstrDLoadN { variable: 3 }),
                0x2a => Box::new(InstrALoadN { variable: 0 }),
                0x2b => Box::new(InstrALoadN { variable: 1 }),
                0x2c => Box::new(InstrALoadN { variable: 2 }),
                0x2d => Box::new(InstrALoadN { variable: 3 }),
                0x32 => Box::new(InstrAALoad {}),
                0x36 => Box::new(InstrIStore { value: data.get_u8() }),
                0x38 => Box::new(InstrFStore { value: data.get_u8() }),
                0x3a => Box::new(InstrAStore { value: data.get_u8() }),
                0x3b => Box::new(InstrIStore0 {}),
                0x3c => Box::new(InstrIStore1 {}),
                0x3d => Box::new(InstrIStore2 {}),
                0x3e => Box::new(InstrIStore3 {}),
                0x43 => Box::new(InstrFStore0 {}),
                0x44 => Box::new(InstrFStore1 {}),
                0x45 => Box::new(InstrFStore2 {}),
                0x46 => Box::new(InstrFStore3 {}),
                0x4b => Box::new(InstrAStore0 {}),
                0x4c => Box::new(InstrAStore1 {}),
                0x4d => Box::new(InstrAStore2 {}),
                0x4e => Box::new(InstrAStore3 {}),
                0x53 => Box::new(InstrAAStore {}),
                0x59 => Box::new(InstrDup {}),
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
                0x7e => Box::new(InstrIAnd {}),
                0x7f => Box::new(InstrLAnd {}),
                0x80 => Box::new(InstrIOr {}),
                0x81 => Box::new(InstrLOr {}),
                0x82 => Box::new(InstrIXor {}),
                0x83 => Box::new(InstrLXor {}),
                0x84 => Box::new(InstrIInc { idx: data.get_u8(), count: data.get_i8() }),
                0x86 => Box::new(InstrI2F {}),
                0x8d => Box::new(InstrF2D {}),
                0x90 => Box::new(InstrD2F {}),
                0x95 => Box::new(InstrFCmpl {}),
                0x96 => Box::new(InstrFCmpg {}),
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
                0xa7 => Box::new(InstrGoto { branch: (data_offset as i16 + data.get_i16()) as usize }),
                0xac => Box::new(InstrIReturn {}),
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
                0xb6 => match constants_method.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrInvokeVirtual {
                        class_name: method.class_name.clone(),
                        method_name: method.method_name.clone(),
                        type_desc: method.type_name.clone()
                    }),
                    _ => panic!("Unknown method")
                },
                0xb7 => match constants_method.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrInvokeSpecial {
                        class_name: method.class_name.clone(),
                        method_name: method.method_name.clone(),
                        type_desc: method.type_name.clone()
                    }),
                    _ => panic!("Unknown method")
                },
                0xb8 => match constants_method.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrInvokeStatic {
                        class_name: method.class_name.clone(),
                        method_name: method.method_name.clone(),
                        type_desc: method.type_name.clone()
                    }),
                    _ => panic!("Unknown method")
                },
                0xb9 => match constants_method.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrInvokeInterface {
                        count: data.get_u16size(),
                        class_name: method.class_name.clone(),
                        method_name: method.method_name.clone(),
                        type_desc: method.type_name.clone()
                    }),
                    _ => panic!("Unknown interface")
                },
                0xba => {
                    let type_name = data.get_u16size();
                    data.get_u16size();

                    match constants_dynamic.get(&type_name) {
                        Some(dynamic) => Box::new(InstrInvokeDynamic {
                            bootstrap_method_idx: dynamic.idx_bootstrap_method,
                            method_name: dynamic.method_name.clone(),
                            method_type: dynamic.type_name.clone()
                        }),
                        _ => panic!("Unknown name/type {}", type_name)
                    }
                },
                0xbd => match constants_class.get(&data.get_u16size()) {
                    Some(class) => Box::new(InstrANewArray {
                        class_name: class.name.clone()
                    }),
                    _ => panic!("Unknown class")
                },
                _ => panic!("Unknown opcode {:#02x}", opcode)
            };

            if debug >= 2 { instr.print(); }
            instructions.push(instr);

            instr_idx += 1;
        }

        for instr in instructions.iter_mut() {
            instr.set_branch(&address_map);
        }

        ByteCode {
            instructions: instructions
        }
    }
}
