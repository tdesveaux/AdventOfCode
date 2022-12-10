use std::{fs::File, io::{self, BufRead}, cell::RefCell, collections::HashMap, rc::{Weak, Rc}};
use clap::Parser;
use itertools::Itertools;

#[derive(clap::ValueEnum, Clone)]
enum Mode {
    Part01,
    Part02,
}

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,

    #[arg(value_enum, default_value_t=Mode::Part02)]
    mode: Mode,
}

#[derive(Debug)]
struct FsEntry {
    parent: Option<Weak<RefCell<FsEntry>>>,
    children: HashMap<String, Rc<RefCell<FsEntry>>>,
    size: Option<usize>
}

impl FsEntry {

    fn create(parent: Option<Weak<RefCell<FsEntry>>>) -> Rc<RefCell<FsEntry>> {
        Rc::new(RefCell::new(FsEntry { parent: parent, ..Default::default() }))
    }
}

impl Default for FsEntry {
    fn default() -> FsEntry {
        FsEntry {
            parent: None,
            children: HashMap::new(),
            size: None,
        }
    }
}

fn get_or_add_child(entry: &Rc<RefCell<FsEntry>>, child_name: &str) -> Rc<RefCell<FsEntry>> {
    {
        let children = &mut entry.as_ref().borrow_mut().children;

        if !children.contains_key(child_name) {
            children
                .insert(child_name.to_owned(), FsEntry::create(Some(Rc::downgrade(&entry))));
        }
    }
    entry.as_ref().borrow().children.get(child_name).unwrap().clone()
}

fn parse_filesystem(buf_reader: &mut io::BufReader<File>) -> Rc<RefCell<FsEntry>> {
    let command_marker = "$ ";

    let root = FsEntry::create(None);
    let mut current_path = root.clone();

    let mut line_reader = buf_reader.lines().map(Result::unwrap).peekable();

    if line_reader.next() != Some("$ cd /".to_string()) {
        panic!("life is hell");
    }

    while let Some(peeked) = line_reader.peek() {
        if !peeked.starts_with(command_marker) {
            panic!("Not a command, unexpected: {}", peeked);
        }

        let commandline = &line_reader.next().unwrap()[command_marker.len()..];
        let mut args = commandline.split(' ');
        let command = args.next().unwrap();
        match command {
            "cd" => {
                match args.next().unwrap() {
                    ".." => {
                        let parent = current_path
                                        .as_ref()
                                        .borrow()
                                        .parent
                                        .as_ref()
                                        .unwrap()
                                        .clone();
                        current_path = parent.upgrade().unwrap(); // ok...
                    },
                    "/" => { 
                        current_path = root.clone();
                    },
                    path => {
                        let new_child = get_or_add_child(&current_path, path);
                        current_path = new_child.clone();
                    }
                };
            },
            "ls" => {
                loop {
                    match line_reader.peek() {
                        Some(peeked) => {
                            if peeked.starts_with(command_marker) {
                                break;
                            }
                        },
                        None => break
                    };
                    let out_line = line_reader.next().unwrap();
                    let (size_or_dir, name) = out_line.split(" ").next_tuple().unwrap();
                    match size_or_dir {
                        "dir" => (),
                        real_size => {
                            let new_child_ref = get_or_add_child(&current_path, name);
                            let new_child = &mut new_child_ref.as_ref().borrow_mut();
                            new_child.size = Some(real_size.parse::<usize>().unwrap());
                        }
                    }
                }
            },
            _ => panic!("Unknown command '{}' ({:?})", command, args.join("|"))
        }
    }

    root.clone()
}

fn get_size_directories_of_at_most(dir: &Rc<RefCell<FsEntry>>, size_limit: usize) -> (usize, usize) {
    let (mut total_dir_size, mut total_matched_size) = (0, 0);
    for (_name, child) in &dir.as_ref().borrow().children {
        let (total_child_size, matched_size) = match child.as_ref().borrow().size {
            // File
            Some(size) => (size, 0),
            // Dir
            None => get_size_directories_of_at_most(&child, size_limit)
        };

        total_dir_size += total_child_size;
        total_matched_size += matched_size;
    }

    if total_dir_size <= size_limit {
        total_matched_size += total_dir_size;
    }

    (total_dir_size, total_matched_size)
}

fn part_01(root: &Rc<RefCell<FsEntry>>) {
    // get directories of at most 100 000 size
    let (total_size, matched_size) = get_size_directories_of_at_most(&root, 100000);
    println!("Total size: {}", total_size);
    println!("Matched size: {}", matched_size);
}

fn part_02(_root: &Rc<RefCell<FsEntry>>) {
}

fn main() {
    let args = Cli::parse();

    let file_path = args.path.as_path();

    if let Err(error) = file_path.try_exists() {
        panic!("File {} not found: {:?}", file_path.display(), error)
    }

    let fp = File::open(file_path).unwrap();
    let mut buf = io::BufReader::new(fp);

    let root = parse_filesystem(&mut buf);

    match args.mode {
        Mode::Part01 => part_01(&root),
        Mode::Part02 => part_02(&root),
    };
}
