
#![deny(clippy::all)]
#![allow(dead_code,non_snake_case,unused_variables,unused_imports)]
use std::fs::File;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;





//reading in web graph file

fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut graphvec: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("file cannot open");
    let filereader = std::io::BufReader::new(file).lines();
    for line in filereader {  
        let l = line.expect("Error");
        let nodes: Vec<&str> = l.trim().split(' ').collect();
        let node1: usize = nodes[0].parse().unwrap();
        let node2: usize = nodes[1].parse().unwrap();
        graphvec.push((node1, node2));
    }
    return graphvec;
}

fn read_file32(path: &str) -> Vec<(u32, u32)> {
    let mut graphvec: Vec<(u32, u32)> = Vec::new();
    let file = File::open(path).expect("file cannot open");
    let filereader = std::io::BufReader::new(file).lines();
    for line in filereader {  
        let l = line.expect("Error");
        let nodes: Vec<&str> = l.trim().split(' ').collect();
        let node1: u32 = nodes[0].parse().unwrap();
        let node2: u32 = nodes[1].parse().unwrap();
        graphvec.push((node1, node2));
    }
    return graphvec;
}
   


//HashMap creation
fn graphcreate(graphvec: &Vec<(u32, u32)>) -> HashMap<u32, Vec<u32>>{
    let mut graph = HashMap::new();
    for &(a, b) in graphvec.iter() {
        graph.entry(a).or_insert(vec![]).push(b);
    }
    graph
}
    
        




fn adj_list(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut graph_adj_list = HashMap::new();

    for &(node1, node2) in edges {
        graph_adj_list.entry(node1).or_insert(vec![]).push(node1);
        graph_adj_list.entry(node2).or_insert(vec![]).push(node2);
    }

    graph_adj_list
}

//triangles
fn triangles(adj_list: &HashMap<usize, Vec<usize>>) -> usize {
    let mut count = 0;

    for (node1, edges) in adj_list {
        for &node2 in edges {
            for &node3 in edges {
                if node2 < node3 && adj_list[&node2].contains(&node3) {
                    count += 1;
                }
            }
        }
    }

    count / 6
}

fn bfs(graph: &HashMap<u32, Vec<u32>>, start: u32) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start);
    queue.push_back(start);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        println!("{}", node);

        for neighbor in &bfsgraph[node] {
            if !visited.contains(neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
}



fn six_deg(graph: &HashMap<&u32, Vec<&u32>>, start: &u32, end: u32) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut depth = 0;

    visited.insert(start);
    queue.push_back((start, depth));

    while !queue.is_empty() {
        let (node, current_depth) = queue.pop_front().unwrap();
        depth = current_depth;
        if node == end {
            return Some(depth);
        }

        for neighbor in &graph[node] {
            if !visited.contains(neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, depth+1));
            }
        }
    }

    None
}



fn main() {

    let file     = read_file32("googlegraph.txt");
    let graph    = graphcreate(&file);

 //BFS Code
    
    let bfsgraph = bfs(&graph, 0);
    bfs(&graph, "");

 

// Adjacency List
    let edges = read_file("googlegraph.txt");
    let graph_adj_list = adj_list(&edges);
    println!("Adjacency List: {:?}", graph_adj_list); 

//Counting Triangles
    let num_triangles = triangles(&graph_adj_list);

//six degrees of separation
    let graph = HashMap::new();
    let distance = six_deg(&graph, "0", "1000");   
    println!("Distance: {:?}", distance);


}            


