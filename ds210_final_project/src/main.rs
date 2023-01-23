
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

    


//HashMap creation
fn graphcreate(graph: &HashMap<&str, Vec<(usize, usize)>>) {
        let mut graph = HashMap::new();
        let file = File::open("googlegraph.txt").unwrap();
        let filereader = std::io::BufReader::new(file).lines();
        let mut graphvec = Vec::new();

        for line in filereader {  
            let l = line.expect("Error");
            let nodes: Vec<&str> = l.trim().split(' ').collect();
            let node1: usize = nodes[0].parse().unwrap();
            let node2: usize = nodes[1].parse().unwrap();
            graphvec.push((node1, node2));
        
        
        let graphvec = graph.entry(node1).or_insert(Vec::new());
        graphvec.push(node2);
        }
    
        
    
    // Print out the graph
    for(node, graphvec)in &graph  {
        println!("{}: ", node);
        for edge in graphvec {
            print!("{} ", edge);
        }
        println!();
       
    }
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

fn bfs(bfsgraph: &HashMap<&str, Vec<&str>>, start: &str) {
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



fn six_deg(sixdgraph: &HashMap<&str, Vec<&str>>, start: &str, end: &str) -> Option<usize> {
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

        for neighbor in &sixdgraph[node] {
            if !visited.contains(neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, depth+1));
            }
        }
    }

    None
}



fn main() {

    let file = File::open("googlegraph.txt");
    let edges = File::open("googlegraph.txt");

 //BFS Code
    let graph = HashMap::new();
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


