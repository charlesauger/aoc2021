use crate::helpers::read_lines;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Nodes {
    caves: HashMap<String, Node>,
    total_paths: usize,
}

#[derive(Debug)]
struct Node {
    name: String,
    small: bool,
    edges: Vec<String>,
}

impl Node {
    fn InsertEdge(&mut self, name: String) {
        self.edges.push(name);
    }
}

impl Nodes {
    fn depth_first_search(&mut self, current_node: String, curr_path: &mut Vec<String>, visited: &mut HashSet<String>) {
        visited.insert(current_node.clone());
        curr_path.push(current_node.clone());

        if current_node == "end" {
            self.total_paths += 1;

            for s in curr_path {
                if s == "end" {
                    print!("{}", s);
                } else {
                    print!("{},", s);
                }
            }
            println!();
            return;
        }

        let curr_cave = self.caves.get(&current_node).unwrap().clone();
        for s in curr_cave.edges.clone() {
            if self.caves.get(&s).unwrap().small {
                if visited.contains(&s) {
                    // Do not take this path
                } else {
                    self.depth_first_search(s.clone(), &mut curr_path.clone(), &mut visited.clone());
                }
            } else {
                self.depth_first_search(s.clone(), &mut curr_path.clone(), &mut visited.clone());
            }
        }
    }

    fn depth_first_search_part_2(&mut self, current_node: String, curr_path: &mut Vec<String>, visited: &mut HashSet<String>, visited_small_twice: &bool) {
        visited.insert(current_node.clone());
        curr_path.push(current_node.clone());

        if current_node == "end" {
            self.total_paths += 1;

            for s in curr_path {
                if s == "end" {
                    // print!("{}", s);
                } else {
                    // print!("{},", s);
                }
            }
            // println!();
            return;
        }

        let curr_cave = self.caves.get(&current_node).unwrap().clone();
        for s in curr_cave.edges.clone() {
            if self.caves.get(&s).unwrap().small {
                if visited.contains(&s) {
                    if !visited_small_twice && s != "start" {
                        // Allow visiting one small cave twice
                        self.depth_first_search_part_2(s.clone(), &mut curr_path.clone(), &mut visited.clone(), &true)
                    } else {
                        // Do not take this path
                    }
                } else {
                    self.depth_first_search_part_2(s.clone(), &mut curr_path.clone(), &mut visited.clone(), visited_small_twice);
                }
            } else {
                self.depth_first_search_part_2(s.clone(), &mut curr_path.clone(), &mut visited.clone(), visited_small_twice);
            }
        }
    }
}

pub fn day_12_part_1() {
    let mut caves = HashMap::<String, Node>::new();

    if let Ok(file_lines) = read_lines("resources/day_12.txt") {
        
        for file_line in file_lines {
            let line = file_line.unwrap();

            let mut split = line.split("-");

            let first_cave_name = split.next().unwrap().trim();
            let second_cave_name = split.next().unwrap().trim();

            if !caves.contains_key(first_cave_name) {
                let small_cave = first_cave_name.chars().any(|c| matches!(c, 'a'..='z'));
                caves.insert( first_cave_name.to_string(), Node { name: first_cave_name.to_string(), small: small_cave, edges: Vec::<String>::new()});
            }

            if !caves.contains_key(second_cave_name) {
                let small_cave = second_cave_name.chars().any(|c| matches!(c, 'a'..='z'));
                caves.insert( second_cave_name.to_string(), Node { name: second_cave_name.to_string(), small: small_cave, edges: Vec::<String>::new()});
            }

            // let mut first_cave_node = caves.get(first_cave).unwrap();
            // let mut second_cave_node = caves.get(second_cave).unwrap();

            // first_cave_node.InsertEdge(second_cave.to_string());
            // second_cave_node.InsertEdge(first_cave.to_string());

            let first_cave_node = caves.get_mut(first_cave_name).unwrap();
            first_cave_node.InsertEdge(second_cave_name.to_string());

            let second_cave_node = caves.get_mut(second_cave_name).unwrap();
            second_cave_node.InsertEdge(first_cave_name.to_string());
        }

        println!("{:?}", caves);
    }

    let mut nodes = Nodes{caves: caves, total_paths: 0};

    nodes.depth_first_search("start".to_string(), &mut Vec::<String>::new(), &mut HashSet::<String>::new());

    println!("Total paths visiting small caves at most once = {}", nodes.total_paths);
}

pub fn day_12_part_2() {
    let mut caves = HashMap::<String, Node>::new();

    if let Ok(file_lines) = read_lines("resources/day_12.txt") {
        
        for file_line in file_lines {
            let line = file_line.unwrap();

            let mut split = line.split("-");

            let first_cave_name = split.next().unwrap().trim();
            let second_cave_name = split.next().unwrap().trim();

            if !caves.contains_key(first_cave_name) {
                let small_cave = first_cave_name.chars().any(|c| matches!(c, 'a'..='z'));
                caves.insert( first_cave_name.to_string(), Node { name: first_cave_name.to_string(), small: small_cave, edges: Vec::<String>::new()});
            }

            if !caves.contains_key(second_cave_name) {
                let small_cave = second_cave_name.chars().any(|c| matches!(c, 'a'..='z'));
                caves.insert( second_cave_name.to_string(), Node { name: second_cave_name.to_string(), small: small_cave, edges: Vec::<String>::new()});
            }

            // let mut first_cave_node = caves.get(first_cave).unwrap();
            // let mut second_cave_node = caves.get(second_cave).unwrap();

            // first_cave_node.InsertEdge(second_cave.to_string());
            // second_cave_node.InsertEdge(first_cave.to_string());

            let first_cave_node = caves.get_mut(first_cave_name).unwrap();
            first_cave_node.InsertEdge(second_cave_name.to_string());

            let second_cave_node = caves.get_mut(second_cave_name).unwrap();
            second_cave_node.InsertEdge(first_cave_name.to_string());
        }

        println!("{:?}", caves);
    }

    let mut nodes = Nodes{caves: caves, total_paths: 0};

    nodes.depth_first_search_part_2("start".to_string(), &mut Vec::<String>::new(), &mut HashSet::<String>::new(), &false);

    println!("Total paths visiting small caves at most once = {}", nodes.total_paths);
}
