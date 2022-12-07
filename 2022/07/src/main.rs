use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

enum Node {
    Dir {
        name: String,
        nodes: Vec<Rc<RefCell<Node>>>,
        parent: Option<Rc<RefCell<Node>>>,
    },
    File {
        name: String,
        size: usize,
        parent: Rc<RefCell<Node>>,
    },
}

impl Node {
    fn parse(line: &str, context: &Context) -> Self {
        let parts: Vec<&str> = line.split(' ').collect();
        if line.starts_with("dir") {
            Self::Dir {
                name: parts[1].to_string(),
                nodes: Vec::new(),
                parent: Some(context.current_dir.clone()),
            }
        } else {
            Self::File {
                name: parts[1].to_string(),
                size: parts[0].parse().unwrap(),
                parent: context.current_dir.clone(),
            }
        }
    }

    fn name(&self) -> String {
        match self {
            Node::Dir {
                name,
                nodes: _,
                parent: _,
            } => name.to_string(),
            Node::File {
                name,
                size: _,
                parent: _,
            } => name.to_string(),
        }
    }

    fn size(&self) -> usize {
        match self {
            Node::Dir {
                name: _,
                nodes,
                parent: _,
            } => nodes.iter().map(|n| n.borrow().size()).sum(),
            Node::File {
                name: _,
                size,
                parent: _,
            } => *size,
        }
    }

    fn get_parent(&self) -> Rc<RefCell<Node>> {
        match self {
            Node::Dir {
                name: _,
                nodes: _,
                parent,
            } => parent.clone().unwrap(),
            Node::File {
                name: _,
                size: _,
                parent,
            } => parent.to_owned(),
        }
    }

    fn collect_all_children(&self) -> Vec<Rc<RefCell<Node>>> {
        match self {
            Node::Dir {
                name: _,
                nodes,
                parent: _,
            } => {
                let mut result = nodes.clone();
                result.extend(
                    nodes
                        .iter()
                        .map(|n| n.borrow().collect_all_children())
                        .reduce(|l, r| {
                            let mut left = l.clone();
                            left.extend(r);
                            left
                        })
                        .unwrap_or(vec![]),
                );
                result
            }
            Node::File {
                name: _,
                size: _,
                parent: _,
            } => {
                vec![]
            }
        }
    }

    fn add_node(&mut self, node: Rc<RefCell<Node>>) {
        if let Node::Dir {
            name: _,
            nodes,
            parent: _,
        } = self
        {
            nodes.push(node)
        }
    }
}

#[derive(Debug)]
enum Command {
    ChangeDirectory(String),
    List(Vec<String>),
}

impl Command {
    fn parse(line: &str, output_lines: Vec<String>) -> Self {
        let parts = line.split(' ').collect::<Vec<&str>>();
        match parts[1] {
            "ls" => Command::List(output_lines.clone()),
            "cd" => Command::ChangeDirectory(parts[2].to_string()),
            _ => panic!("Unknown command: {}", parts[1]),
        }
    }

    fn execute(&self, context: Context) -> Context {
        match self {
            Command::ChangeDirectory(target) => match target.as_str() {
                ".." => Context {
                    current_dir: context.current_dir.borrow().get_parent(),
                },
                _ => match &*context.current_dir.borrow() {
                    Node::Dir {
                        name: _,
                        nodes,
                        parent: _,
                    } => Context {
                        current_dir: nodes
                            .iter()
                            .find(|x| x.borrow().name() == *target)
                            .unwrap()
                            .to_owned(),
                    },
                    Node::File {
                        name: _,
                        size: _,
                        parent: _,
                    } => panic!("This is a file"),
                },
            },
            Command::List(extra_lines) => {
                for line in extra_lines {
                    context
                        .current_dir
                        .as_ref()
                        .borrow_mut()
                        .add_node(Rc::new(RefCell::new(Node::parse(line, &context))));
                }
                context
            }
        }
    }
}

struct Context {
    current_dir: Rc<RefCell<Node>>,
}

fn parse_commands() -> Vec<Command> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut extra_lines = Vec::<String>::new();
    let mut last_line = Option::<String>::None;
    let mut commands = Vec::<Command>::new();
    for line in lines.flatten() {
        if line.starts_with("$") {
            if let Some(last_line) = last_line {
                commands.push(Command::parse(&last_line, extra_lines.clone()))
            }
            last_line = Some(line);
            extra_lines.clear();
        } else {
            extra_lines.push(line);
        }
    }
    commands
}

fn part1() -> usize {
    let commands = parse_commands();

    let root = Rc::<RefCell<Node>>::new(RefCell::new(Node::Dir {
        name: "/".to_string(),
        nodes: Vec::new(),
        parent: None,
    }));

    let original_context = Context {
        current_dir: root.clone(),
    };

    let final_context = commands
        .iter()
        .skip(1)
        .fold(Context { current_dir: root }, |context, command| {
            command.execute(context)
        });

    let sum = original_context
        .current_dir
        .borrow()
        .collect_all_children()
        .iter()
        .map(|node| {
            if let Node::Dir {
                name: _,
                nodes: _,
                parent: _,
            } = *node.borrow()
            {
                let size = node.borrow().size();
                if size <= 100000 {
                    size
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum::<usize>();
    sum
}

fn part2() -> usize {
    let commands = parse_commands();

    let root = Rc::<RefCell<Node>>::new(RefCell::new(Node::Dir {
        name: "/".to_string(),
        nodes: Vec::new(),
        parent: None,
    }));

    let original_context = Context {
        current_dir: root.clone(),
    };

    let final_context = commands
        .iter()
        .skip(1)
        .fold(Context { current_dir: root }, |context, command| {
            command.execute(context)
        });

    let disk_size = 70000000;
    let occupied = original_context.current_dir.borrow().size();
    let free_space = disk_size - occupied;
    let required_to_remove = 30000000 - free_space;

    let x = original_context
        .current_dir
        .borrow()
        .collect_all_children()
        .iter()
        .map(|node| node.borrow().size())
        .filter(|size| *size >= required_to_remove)
        .min()
        .unwrap();
    x
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
