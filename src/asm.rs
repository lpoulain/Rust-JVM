use std::{collections::{HashMap, HashSet, hash_map::DefaultHasher}, fs::File, hash::{Hash, Hasher}, io::Write};

use crate::bytecode::ByteCode;

pub enum Arch {
    LinuxX64,
    MacosX64
}

pub fn bytecode_to_asm(class_name: &String, bytecode: &ByteCode, arch: Arch) {
    let mut assembly = Assembly {
        jumps: HashSet::new(),
        strings: HashSet::new(),
        variables: HashMap::new(),
        register_idx: 7
    };

    let mut filename = class_name.clone();
    filename.push_str(".asm");

    match File::create(filename) {
        Err(why) => panic!("Couldn't create {}.asm: {}", class_name, why),
        Ok(mut file) => {
            let mut asm_instructions: Vec<String> = Vec::new();

            for instr in bytecode.instructions.iter() {
                let asm_instruction = match arch {
                    Arch::LinuxX64 => instr.convert_to_linux_intel_asm(&mut assembly),
                    Arch::MacosX64 => instr.convert_to_macos_intel_asm(&mut assembly)
                };
                asm_instructions.push(asm_instruction);
            }
        
            let mut content: Vec<String> = Vec::new();

            match arch {
                Arch::LinuxX64 => { content.push("    global    _start\n    section   .text\n_start:\n".to_string()); },
                Arch::MacosX64 => { content.push("    global    start\n    section   .text\nstart:\n".to_string()); }
            };
        
            let mut instr_idx: usize = 0;
            for instr in asm_instructions.iter() {
                if assembly.jumps.contains(&instr_idx) {
                    content.push(format!("__branch{}:\n", instr_idx));
                }
                content.push(instr.clone());
                content.push("\n".to_string());
                instr_idx += 1;
            }
        
            content.push("\n__string_length:\n    xor       rax, rax\n    xor       rcx, rcx\n __string_len:\n    mov       cl, [rdi]\n    cmp       cl, 0\n    jz __string_len_ok\n".to_string());
            content.push("    inc       rax\n    inc       rdi\n    jmp __string_len\n__string_len_ok:\n    ret\n".to_string());
        
            content.push("\n    section   .data\n".to_string());
            for string in assembly.strings.iter() {
                content.push(format!("{}: db \"{}\", 0\n", string_label(string), string));
            }
            content.push("str_cr: db 0ah, 0\n".to_string());

            let mut final_content: String = content.iter().map(|x| x.clone()).collect();
            // asm optimization: the JVM is passing data between bytecode instructions through the stack
            // bypass this by keeping the value in the rax register
            final_content = final_content.replace("    push rax\n    pop rax\n", "");

            match file.write_all(final_content.as_bytes()) {
                Err(why) => panic!("Couldn't write to {}.asm: {}", class_name, why),
                Ok(_) => {
                    println!("Bytecode class compiled into x64 assembly, Intel notation ({}.asm)", class_name);
                }
            };
        }
    };
}

fn string_label(string: &String) -> String {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    format!("str_{}", hasher.finish())
}

pub struct Assembly {
    jumps: HashSet<usize>,
    strings: HashSet<String>,
    variables: HashMap<u8, u8>,
    register_idx: u8
}

impl Assembly {
    pub fn add_jump(&mut self, branch: usize) {
        self.jumps.insert(branch);
    }
 
    pub fn add_string(&mut self, string: &String) -> String {
        self.strings.insert(string.clone());
        string_label(&string)
    }

    pub fn var_to_reg(&mut self, var_idx: u8) -> u8 {
        match self.variables.get(&var_idx) {
            Some(reg) => *reg,
            None => {
                self.register_idx += 1;
                self.variables.insert(var_idx, self.register_idx);
                self.register_idx
            }
        }
    }
}