use std::{collections::{HashSet, hash_map::DefaultHasher}, fs::File, hash::{Hash, Hasher}, io::Write};

use crate::bytecode::ByteCode;

pub fn bytecode_to_intel_asm(class_name: &String, bytecode: &ByteCode) {
    let mut assembly = Assembly {
        jumps: HashSet::new(),
        strings: HashSet::new()
    };

    let mut asm_instructions: Vec<String> = Vec::new();

    for instr in bytecode.instructions.iter() {
        asm_instructions.push(instr.convert_to_intel_asm(&mut assembly));
    }

    let mut filename = class_name.clone();
    filename.push_str(".asm");

    match File::create(filename) {
        Err(why) => panic!("Couldn't create {}.asm: {}", class_name, why),
        Ok(mut file) => {
            let mut content: Vec<String> = Vec::new();
            content.push("    global    start\n    section   .text\nstart:\n".to_string());
        
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

            let final_content: String = content.iter().map(|x| x.clone()).collect();

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
    strings: HashSet<String>
}

impl Assembly {
    pub fn add_jump(&mut self, branch: usize) {
        self.jumps.insert(branch);
    }
 
    pub fn add_string(&mut self, string: &String) -> String {
        self.strings.insert(string.clone());
        string_label(&string)
    }
}