use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: hack_assembler <input_file>");
        std::process::exit(1);
    }
    let input_file = &args[1];

    let assembly_code = fs::read_to_string(input_file)?;

    let mut symbol_table = build_symbol_table(&assembly_code);

    let machine_code = assemble(&assembly_code, &mut symbol_table);

    let output_file = input_file.replace(".asm", ".hack");
    let mut file = fs::File::create(output_file)?;
    for line in machine_code {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn build_symbol_table(assembly_code: &str) -> HashMap<String, u16> {
    let mut symbol_table = HashMap::new();
    let mut rom_address = 0;

    let predefined_symbols = [
        ("SP", 0),
        ("LCL", 1),
        ("ARG", 2),
        ("THIS", 3),
        ("THAT", 4),
        ("R0", 0),
        ("R1", 1),
        ("R2", 2),
        ("R3", 3),
        ("R4", 4),
        ("R5", 5),
        ("R6", 6),
        ("R7", 7),
        ("R8", 8),
        ("R9", 9),
        ("R10", 10),
        ("R11", 11),
        ("R12", 12),
        ("R13", 13),
        ("R14", 14),
        ("R15", 15),
        ("SCREEN", 16384),
        ("KBD", 24576),
    ];

    for &(symbol, address) in &predefined_symbols {
        symbol_table.insert(symbol.to_string(), address);
    }

    for line in assembly_code.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if line.starts_with('(') && line.ends_with(')') {
            let label = &line[1..line.len() - 1];
            symbol_table.insert(label.to_string(), rom_address);
        } else {
            rom_address += 1;
        }
    }

    symbol_table
}

fn assemble(assembly_code: &str, symbol_table: &mut HashMap<String, u16>) -> Vec<String> {
    let mut machine_code = Vec::new();
    let mut ram_address = 16;

    for line in assembly_code.lines() {
        let line = line.trim();
        if line.is_empty()
            || line.starts_with("//")
            || (line.starts_with('(') && line.ends_with(')'))
        {
            continue;
        }

        let clean_line = line.split("//").next().unwrap().trim();

        if clean_line.starts_with('@') {
            let symbol = &clean_line[1..];
            let address = if let Ok(number) = symbol.parse::<u16>() {
                number
            } else {
                *symbol_table.entry(symbol.to_string()).or_insert_with(|| {
                    let addr = ram_address;
                    ram_address += 1;
                    addr
                })
            };
            machine_code.push(format!("{:016b}", address));
        } else {
            let parts: Vec<&str> = clean_line.split(|c| c == '=' || c == ';').collect();
            let dest = if clean_line.contains('=') {
                parts[0]
            } else {
                ""
            };
            let comp = if clean_line.contains('=') {
                parts[1]
            } else {
                parts[0]
            };
            let jump = if clean_line.contains(';') {
                parts[parts.len() - 1]
            } else {
                ""
            };

            machine_code.push(format!(
                "111{}{}{}",
                comp_to_binary(comp),
                dest_to_binary(dest),
                jump_to_binary(jump)
            ));
        }
    }

    machine_code
}

fn dest_to_binary(dest: &str) -> &str {
    match dest {
        "M" => "001",
        "D" => "010",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "AMD" => "111",
        _ => "000",
    }
}

fn comp_to_binary(comp: &str) -> &str {
    match comp {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "M" => "1110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "!M" => "1110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "-M" => "1110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "M+1" => "1110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "M-1" => "1110010",
        "D+A" => "0000010",
        "D+M" => "1000010",
        "D-A" => "0010011",
        "D-M" => "1010011",
        "A-D" => "0000111",
        "M-D" => "1000111",
        "D&A" => "0000000",
        "D&M" => "1000000",
        "D|A" => "0010101",
        "D|M" => "1010101",
        _ => "0000000",
    }
}

fn jump_to_binary(jump: &str) -> &str {
    match jump {
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => "000",
    }
}
