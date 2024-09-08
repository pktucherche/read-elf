use std::fs::File;
use std::io::{self, Read};
use std::path::Path;



#[repr(C)]
struct Elf64Header {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
struct Elf64SectionHeader {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

#[repr(C)]
struct Elf64SegmentHeader {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}


fn read_elf_file(
    file_path: &Path
) -> io::Result<Vec<u8>> {

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn parse_elf_header(
    data: &[u8]
) -> Elf64Header {

    unsafe {
        std::ptr::read(data.as_ptr() as *const Elf64Header)
    }
}

fn parse_section_headers(
    data: &[u8],
    shoff: usize,
    shnum: usize,
    shentsize: usize
) -> Vec<Elf64SectionHeader> {
 
    let mut section_headers = Vec::new();

    for i in 0..shnum {
        let start = shoff + i * shentsize;
        let section_header = unsafe {
            std::ptr::read(data[start..].as_ptr() as *const Elf64SectionHeader)
        };
        section_headers.push(section_header);
    }

    section_headers
}

fn parse_segment_headers(
    data: &[u8],
    phoff: usize,
    phnum: usize,
    phentsize: usize
) -> Vec<Elf64SegmentHeader> {

    let mut segment_headers = Vec::new();

    for i in 0..phnum {
        let start = phoff + i * phentsize;
        let segment_header = unsafe {
            std::ptr::read(data[start..].as_ptr() as *const Elf64SegmentHeader)
        };
        segment_headers.push(segment_header);
    }

    segment_headers
}

fn extract_section_names(
    data: &[u8],
    section_headers: &[Elf64SectionHeader],
    strtab_section: &Elf64SectionHeader
) -> Vec<String> {

    let strtab_offset = strtab_section.sh_offset as usize;
    let strtab_size = strtab_section.sh_size as usize;

    let strtab_data = &data[strtab_offset..strtab_offset + strtab_size];


    let mut section_name: Vec<String> = Vec::new();

    for section in section_headers.iter() {
        let name_offset = section.sh_name as usize;
        let mut name = Vec::new();
        
        for &byte in &strtab_data[name_offset..] {
            if byte == 0 {
                break;
            }
            name.push(byte);
        }

        section_name.push(
            if name.is_empty() {
                "<invalid name>".to_string()
            } else {
                String::from_utf8(name).unwrap_or_else(|_| "<invalid name>".to_string())
            }
        );
    }

    return section_name;
}

fn get_segment_type_name(
    segment_type: u32
) -> &'static str {

    match segment_type {
        0x00000000 => "NULL",
        0x00000001 => "LOAD",
        0x00000002 => "DYNAMIC",
        0x00000003 => "INTERP",
        0x00000004 => "NOTE",
        0x00000005 => "SHLIB",
        0x00000006 => "PHDR",
        0x00000007 => "TLS",
        _ => "unknow",
    }
}

fn get_segment_flag_name(
    segment_flag: u32
) -> String {

    let read_flag = if segment_flag & 0x4 != 0 { 'r' } else { '-' };
    let write_flag = if segment_flag & 0x2 != 0 { 'w' } else { '-' };
    let exec_flag = if segment_flag & 0x1 != 0 { 'x' } else { '-' };

    format!("{}{}{}", read_flag, write_flag, exec_flag)
}

fn get_section_flag_name(
    section_header: &Elf64SectionHeader,
    segment_headers: &Vec<Elf64SegmentHeader>
) -> String {

    let readonly = if section_header.sh_flags & 0x1 != 0 { "" } else { "READONLY" };
    let alloc = if section_header.sh_flags & 0x2 != 0 { "ALLOC" } else { "" };
    let code = if section_header.sh_flags & 0x4 != 0 { "CODE" } else { "" };

    let mut ranges_segment: Vec<(u64, u64)> = Vec::new();
    for segment_header in segment_headers.iter() {
        ranges_segment.push((segment_header.p_offset, segment_header.p_offset+segment_header.p_filesz))
    }
    ranges_segment = merge_ranges(ranges_segment);

    let mut if_load = false;
    let section_min = section_header.sh_offset;
    let section_max = section_header.sh_offset + section_header.sh_size;
    for &(segment_min, segment_max) in ranges_segment.iter() {
        if section_min >= segment_min && section_max <= segment_max {
            if_load = true;
            break;
        }
    }

    let load = if if_load == true { "LOAD" } else { "" };

    let mut parts = Vec::new();
    if !alloc.is_empty() { parts.push(alloc); }
    if !load.is_empty() { parts.push(load); }
    if !readonly.is_empty() { parts.push(readonly); }
    if !code.is_empty() { parts.push(code); }

    parts.join(", ")
}

fn merge_ranges(mut intervals: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if intervals.is_empty() {
        return intervals;
    }

    intervals.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged: Vec<(u64, u64)> = vec![intervals[0]];

    for i in 1..intervals.len() {
        let (_prev_min, prev_max) = merged.last().copied().unwrap();
        let (curr_min, curr_max) = intervals[i];

        if curr_min <= prev_max {
            let new_max = std::cmp::max(prev_max, curr_max);
            merged.last_mut().unwrap().1 = new_max;
        } else {
            merged.push((curr_min, curr_max));
        }
    }

    merged
}


fn main() {
    let file_path = Path::new("test/exec");
    let file_data = read_elf_file(file_path).expect("Erreur lors de la lecture du fichier ELF");

    let elf_header = parse_elf_header(&file_data);

    let segment_headers = parse_segment_headers(
        &file_data,
        elf_header.e_phoff as usize,
        elf_header.e_phnum as usize,
        elf_header.e_phentsize as usize,
    );

    let section_headers = parse_section_headers(
        &file_data,
        elf_header.e_shoff as usize,
        elf_header.e_shnum as usize,
        elf_header.e_shentsize as usize,
    );

    let shstrtab_index = elf_header.e_shstrndx as usize;
    let shstrtab_section = &section_headers[shstrtab_index];

    let section_names = extract_section_names(&file_data, &section_headers, shstrtab_section);

    println!("Segments:");
    for (i, segment) in segment_headers.iter().enumerate() {
        println!(
            "{}{} {}{} Offset = {:#016x}, Size = {:#016x}, Flags = {}",
            " ".repeat(4 - i.to_string().len()),
            i,
            get_segment_type_name(segment.p_type),
            " ".repeat(14 - get_segment_type_name(segment.p_type).len()),
            segment.p_offset,
            segment.p_filesz,
            get_segment_flag_name(segment.p_flags)
        );
    }

    println!("\nSections:");
    for (i, section) in section_headers.iter().enumerate() {
        let section_name = &section_names[i];
        let max_len_section_name = section_names.iter()
            .map(|s| s.len())
            .max()
            .unwrap_or(0);

        println!(
            "{}{} {}{} Offset = {:#016x}, Size = {:#016x}\n{}{}",
            " ".repeat(4 - i.to_string().len()),
            i,
            section_name,
            " ".repeat(max_len_section_name - section_name.len()),
            section.sh_offset,
            section.sh_size,
            " ".repeat(max_len_section_name + 4 + 2),
            get_section_flag_name(section, &segment_headers)
        );
    }
}