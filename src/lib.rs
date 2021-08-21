// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
// typecoders - An interpreter that introduces you.
//
// An example code:
//  $%#/*++;%##*++;#/++++;#/*++;#/*+++;-%##*;$@/*+;+++;#/;@
//
// Output:
//  Languages:
//      FlaScript
//      Gretea
//      C
//      C++
//      C++/CLI
//      Python
//
//  Branches:
//      Computer Science
//      Programming languages
//      Programming languages and compilers
//
// github.com/ferhatgec/typecoders
// github.com/ferhatgec/typecode
// github.com/ferhatgec/typecode.py

pub enum Tokens {
    Lang,
    Branch,
    OS,

    Push,
    Push5,
    Push10,
    Push50,
    Push100,
    Print,
    Undef
}

pub struct TypeCode {
    // taken from https://dzone.com/articles/big-list-256-programming
    languages: Vec<String>,

    // taken from https://www.quora.com/What-are-the-branches-of-computer-science
    branches : Vec<String>,

    osystems : Vec<String>,
    is_lang  : bool,
    is_branch: bool,
    is_os    : bool,

    stack    : u64,

    pub info_languages  : Vec<String>,
    pub info_branches   : Vec<String>,
    pub info_osystems   : Vec<String>
}

impl Tokens {
    fn val(ch: char) -> Self {
        use self::Tokens::{*};

        match ch {
            '$' => { Lang },
            '@' => { Branch },
            '?' => { OS },

            '+' => { Push },
            '*' => { Push5 },
            '/' => { Push10 },
            '%' => { Push50 },
            '-' => { Push100 },

            ';' => { Print },
            _   => { unreachable!("Undefined character: '{}'", ch) }
        }
    }
}

impl Default for TypeCode {
    fn default() -> Self {
        TypeCode {
            languages     : vec![
                "4th Dimension/4D",
                "ABAP",
                "ABC",
                "ActionScript",
                "Ada",
                "Agilent VEE",
                "Algol",
                "Alice",
                "Angelscript",
                "Apex",
                "APL",
                "AppleScript",
                "Arc",
                "Arduino",
                "ASP",
                "AspectJ",
                "Assembly",
                "ATLAS",
                "Augeas",
                "AutoHotkey",
                "AutoIt",
                "AutoLISP",
                "Automator",
                "Avenue",
                "Awk",
                "Bash",
                "(Visual) Basic",
                "bc",
                "BCPL",
                "BETA",
                "BlitzMax",
                "Boo",
                "Bourne Shell",
                "Bro",
                "C",
                "C Shell",
                "C#",
                "C++",
                "C++/CLI",
                "C-Omega",
                "Caml",
                "Ceylon",
                "CFML",
                "cg",
                "Ch",
                "CHILL",
                "CIL",
                "CL (OS/400)",
                "Clarion",
                "Clean",
                "Clipper",
                "Clojure",
                "CLU",
                "COBOL",
                "Cobra",
                "CoffeeScript",
                "ColdFusion",
                "COMAL",
                "Common Lisp",
                "Coq",
                "cT",
                "Curl",
                "D",
                "Dart",
                "DCL",
                "DCPU-16 ASM",
                "Delphi/Object Pascal",
                "DiBOL",
                "Dylan",
                "E",
                "eC",
                "Ecl",
                "ECMAScript",
                "EGL",
                "Eiffel",
                "Elixir",
                "Emacs Lisp",
                "Erlang",
                "Etoys",
                "Euphoria",
                "EXEC",
                "F#",
                "Factor",
                "Falcon",
                "Fancy",
                "Fantom",
                "Felix",
                "FlaScript",
                "Forth",
                "Fortran",
                "Fortress",
                "(Visual) FoxPro",
                "Gambas",
                "GNU Octave",
                "Go",
                "Google AppsScript",
                "Gosu",
                "Gretea",
                "Groovy",
                "Haskell",
                "haXe",
                "Heron",
                "HPL",
                "HyperTalk",
                "Icon",
                "IDL",
                "Inform",
                "Informix-4GL",
                "INTERCAL",
                "Io",
                "Ioke",
                "J",
                "J#",
                "JADE",
                "Java",
                "Java FX Script",
                "JavaScript",
                "JScript",
                "JScript.NET",
                "Julia",
                "Korn Shell",
                "Kotlin",
                "LabVIEW",
                "Ladder Logic",
                "Lasso",
                "Limbo",
                "Lingo",
                "Lisp",
                "Logo",
                "Logtalk",
                "LotusScript",
                "LPC",
                "Lua",
                "Lustre",
                "M4",
                "MAD",
                "Magic",
                "Magik",
                "Malbolge",
                "MANTIS",
                "Maple",
                "Mathematica",
                "MATLAB",
                "Max/MSP",
                "MAXScript",
                "MEL",
                "Mercury",
                "Mirah",
                "Miva",
                "ML",
                "Monkey",
                "Modula-2",
                "Modula-3",
                "MOO",
                "Moto",
                "MS-DOS Batch",
                "MUMPS",
                "NATURAL",
                "Nemerle",
                "Nim",
                "NQC",
                "NSIS",
                "Nu",
                "NXT-G",
                "Oberon",
                "Object Rexx",
                "Objective-C",
                "Objective-J",
                "OCaml",
                "Occam",
                "ooc",
                "Opa",
                "OpenCL",
                "OpenEdge ABL",
                "OPL",
                "Oz",
                "Paradox",
                "Parrot",
                "Pascal",
                "Perl",
                "PHP",
                "Pike",
                "PILOT",
                "PL/I",
                "PL/SQL",
                "Pliant",
                "PostScript",
                "POV-Ray",
                "PowerBasic",
                "PowerScript",
                "PowerShell",
                "Processing",
                "Prolog",
                "Puppet",
                "Pure Data",
                "Python",
                "Q",
                "R",
                "Racket",
                "REALBasic",
                "REBOL",
                "Revolution",
                "REXX",
                "RPG (OS/400)",
                "Ruby",
                "Rust",
                "S",
                "S-PLUS",
                "SAS",
                "Sather",
                "Scala",
                "Scheme",
                "Scilab",
                "Scratch",
                "sed",
                "Seed7",
                "Self",
                "Shell",
                "SIGNAL",
                "Simula",
                "Simulink",
                "Slate",
                "Smalltalk",
                "Smarty",
                "SPARK",
                "SPSS",
                "SQR",
                "Squeak",
                "Squirrel",
                "Standard ML",
                "Suneido",
                "SuperCollider",
                "TACL",
                "Tcl",
                "Tex",
                "thinBasic",
                "TOM",
                "Transact-SQL",
                "Turing",
                "TypeScript",
                "Vala/Genie",
                "VBScript",
                "Verilog",
                "VHDL",
                "VimL",
                "Visual Basic .NET",
                "WebDNA",
                "Whitespace",
                "X10",
                "xBase",
                "XBase++",
                "Xen",
                "XPL",
                "XSLT",
                "XQuery",
                "yacc",
                "Yorick",
                "Z shell"
            ].iter().map(|&s| s.into()).collect(),
            branches      : vec![
                "Human-computer interaction",
                "Data science",
                "Natural language processing",
                "Programming languages",
                "Software engineering",
                "Architecture and organization",
                "Cyber security",
                "Information management",
                "Networking and communication",
                "Computer graphics",
                "Platform-based development",
                "Graphics and visual computing",
                "Algorithms and complexity",
                "Parallel and distributed computing",
                "Intelligent systems",
                "Security and information assurance",
                "Computer Science",
                "Computer Engineering",
                "Information Systems",
                "New Media",
                "Information Technology (IT)",
                "Information Science",
                "Mathematical foundations.",
                "Algorithms and data structures.",
                "Artificial intelligence.",
                "Communication and security.",
                "Computer architecture.",
                "Computer graphics.",
                "Concurrent, parallel, and distributed systems.",
                "Databases.",
                "Programming languages and compilers",
                "Scientific computing",
                "Software engineering",
                "Theory of computing"
            ].iter().map(|&s| s.into()).collect(),
            osystems      : vec![
                "Arthur",
                "RISC OS",
                "Fire OS",
                "Amiga OS",
                "AMSDOS",
                "macOS",
                "iOS",
                "iPadOS",
                "tvOS",
                "bridgeOS",
                "Atari DOS",
                "BeOS",
                "Unix",
                "BESYS",
                "Plan 9",
                "Inferno",
                "Android",
                "Harmony OS",
                "LiteOS",
                "iRMX",
                "PC DOS",
                "OS/2",
                "Remix OS",
                "KaiOS",
                "LynxOS",
                "Xenix",
                "MS-DOS",
                "DOS/V",
                "Windows",
                    "Windows 1.0",
                    "Windows 2.0",
                    "Windows 3.0",
                    "Windows 3.1x",
                    "Windows 3.2",
                    "Windows 95",
                    "Windows 98",
                    "Windows ME",

                    "Windows NT",
                        "Windows NT 3.1",
                        "Windows NT 4.0",
                        "Windows 2000",
                        "Windows XP",
                        "Windows Server 2003",
                        "Windows Vista",
                        "Windows Phone 7",
                        "Windows 8",
                        "Windows RT",
                        "Windows Phone 8",
                        "Windows 8.1",
                        "Windows Phone 8.1",
                        "Windows 10",
                        "Windows 10 Mobile",
                        "Windows 11",
                "ES",
                "NeXTSTEP",
                "NetWare",
                "UnixWare",
                "Bada",
                "Tizen",
                "One UI",
                "Sun OS",
                "BSD",
                    "FreeBSD",
                        "DragonFlyBSD",
                        "MidnightBSD",
                        "GhostBSD",
                        "TrueOS",
                        "prismBSD",
                    "NetBSD",
                        "OpenBSD",
                            "Bitrig",
                    "Darwin",

                "GNU Hurd",
                "Linux",
                    "RHEL",
                        "Rocky Linux",
                        "Red Hat Linux",
                        "CentOS",
                        "Fedora",
                        "openSUSE",
                            "SUSE Linux Enterprise Desktop",
                            "SUSE Linux Enterprise Server",
                            "SUSE Studio",
                            "GeckoLinux",
                        "Mandrake Linux",
                    "Debian",
                        "MX Linux",
                        "Deepin",
                        "Devuan",
                        "Kali Linux",
                        "Pure OS",
                        "Ubuntu",
                            "Kubuntu",
                            "Lubuntu",
                            "Ubuntu Budgie",
                            "Ubuntu Kylin",
                            "Ubuntu Mate",
                            "Xubuntu",

                            "Bodhi Linux",
                            "elementary OS",
                            "Linux Mint",
                            "Zorin OS",
                            "Pop!_OS",

                    "Arch Linux",
                        "Manjaro",
                        "Artix Linux",
                        "EndeavourOS",
                        "SteamOS",
                    "Gentoo",
                        "Chrome OS",
                        "Chromium OS",
                    "NixOS",
                    "Void Linux",
                    "GuixSD",
                    "Solus",
                "Redox",
                "illumos",
                    "OpenIndiana",

                "FreeDOS",
                "Genode",
                "FFusionOS",
                "Ghost OS",
                "Haiku",
                "ReactOS",
                "TempleOS",
                "Serenity",
                "Visopsys"
            ].iter().map(|&s| s.into()).collect(),
            is_lang       : false,
            is_branch     : false,
            is_os         : false,
            stack         : 0,
            info_languages: vec![],
            info_branches : vec![],
            info_osystems : vec![]
        }
    }
}

impl TypeCode {
    fn init(&mut self, file_data: String) {
        use Tokens::{*};

        for ch in file_data.chars() {
            match Tokens::val(ch) {
                Lang => {
                    self.is_lang = !self.is_lang;
                },
                Branch => {
                    self.is_branch = !self.is_branch;
                },
                OS => {
                    self.is_os = !self.is_os;
                },

                Push => {
                    self.stack += 1;
                },
                Push5 => {
                    self.stack += 5;
                },
                Push10 => {
                    self.stack += 10;
                },
                Push50 => {
                    self.stack += 50;
                },
                Push100 => {
                    self.stack += 100;
                },

                Print => {
                    if self.is_branch && self.stack < self.branches.len() as u64 {
                        self.info_branches.push(self.branches[self.stack as usize].clone());
                    } else if self.is_lang && self.stack < self.languages.len() as u64 {
                        println!("{} | {}", self.stack, self.languages[0 as usize].clone());
                        self.info_languages.push(self.languages[self.stack as usize].clone());
                    } else if self.is_os && self.stack < self.osystems.len() as u64 {
                        self.info_osystems.push(self.osystems[self.stack as usize].clone());
                    }

                    self.stack = 0;
                },
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TypeCode;
    #[test]
    fn hmm() {
        let mut init = TypeCode::default();
        let val = String::from("$%///*++;%////*++;///++++;///*++;///*+++;-%////*;$@/*+;+++;///;@?-++;%/*++;?");
        init.init(val);


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
}