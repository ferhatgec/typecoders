// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//

fn main() {
    use typecoders::{TypeCode};

    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("typecode - create unique code that introduce yourself\n\n\
                 Usage:\n\
                 {} {{typecode}}", args.last().unwrap());

        std::process::exit(1);
    }

    let mut init = TypeCode::default();
    init.init(std::env::args().last().unwrap());

    println!("\x1b[0;93mLanguages\x1b[0m:");

    for lang in init.info_languages {
        println!("  {}", lang);
    } println!("\n\x1b[0;92mBranches\x1b[0m:");

    for branch in init.info_branches {
        println!("  {}", branch);
    } println!("\n\x1b[0;92mOperating Systems\x1b[0m:");

    for os in init.info_osystems {
        println!("  {}", os);
    }
}