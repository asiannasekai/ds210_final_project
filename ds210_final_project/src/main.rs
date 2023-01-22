#![deny(clippy::all)]
#[allow(dead_code,non_snake_case)]
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;


//Read File ( ListofEdges ) u32
fn read_file(path: &str) -> Vec<(u32, u32)> {
    let mut result: Vec<(u32, u32)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<u32>().unwrap();
        let y = v[1].parse::<u32>().unwrap();
        result.push((x, y));
    }
    return result;
}
// usize
fn read_file2(path: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<usize>().unwrap();
        let y = v[1].parse::<usize>().unwrap();
        result.push((x, y));
    }
    return result;
}
//HashMap creation
fn create_graph(edge: &Vec<(u32, u32)>) -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    for &(a, b) in edge.iter() {
        graph.entry(a).or_insert(vec![]).push(b);
    }
    graph
}

fn bfs(graph: &HashMap<u32, Vec<u32>>, start: u32) -> HashMap<u32, u32> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    distances.insert(start, 0);

    while !queue.is_empty() {
        let vertex = queue.pop_front().unwrap();
        let distance = *distances.get(&vertex).unwrap();

        for neighbor in graph.get(&vertex).unwrap() {
            if !distances.contains_key(neighbor) {
                distances.insert(*neighbor, distance + 1);
                queue.push_back(*neighbor);
            }
        }
    }

    distances
}
fn six_deg(edges: &Vec<(usize, usize)>, start: usize) -> Option<usize> {
    let mut _graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(a, b) in edges {
        _graph.entry(a).or_default().push(b);
        _graph.entry(b).or_default().push(a);
    }
    let mut queue = VecDeque::new();
    let level = 0;
    let mut visited = vec![false; _graph.len()];
    queue.push_back((start, level));
    visited[start] = true;
    while let Some((node, level)) = queue.pop_front() {
        for &neighbor in _graph.get(&node).unwrap() {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back((neighbor, level + 1));
                if level + 1 > 5 {
                    return Some(neighbor);
                }
            }
        }
    }
    None
}

fn main() {
    let edges: Vec<(u32, u32)> = read_file("amazon.txt");
    let _edges: Vec<(usize, usize)> = read_file2("amazon.txt");
    let _mygraph = create_graph(&edges);
    let distance_bds = bfs(&_mygraph,0);
    let six_d = six_deg(&_edges,0);
    println!("{:?}",distance_bds);
    println!("{:?}",six_d);
}

