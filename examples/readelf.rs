use rkmod::elf::ElfContent;
use std::path::PathBuf;

fn main() {
    let path = std::env::args().nth(1).expect("file path required");
    let path = PathBuf::from(path);
    let content = ElfContent::open(&path).expect("failed to open file");
    let compression = content.detect_compression();
    println!("compression: {:?}", compression);
    let content = content.decompress().expect("failed to decompress");
    if !content.check_elf() {
        panic!("module is not an elf file");
    }
    println!("module is an elf file");
    let elf = content.read_elf().expect("failed to parse elf");

    println!("header: {:?}", elf.ehdr);
    if let Some(segments) = elf.segments() {
        for segment in segments {
            println!("segment: {:?}", segment);
        }
    }

    if let (Some(sections), ref strtab) = elf
        .section_headers_with_strtab()
        .expect("failed to parse section headers and strtab")
    {
        for section in sections {
            if section.sh_size == 0 {
                continue;
            }
            let name = strtab
                .map(|strtab| strtab.get(section.sh_name as usize).ok())
                .flatten();
            let name = name
                .map(|name| name.to_string())
                .unwrap_or_else(|| "unknown".to_string());
            println!("section [{}]: {:?}", name, section);
        }
    }
}
