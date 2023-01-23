
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
        let linemsg = line.expect("Error");
        let nodes: Vec<&str> = linemsg.trim().split(' ').collect();
        let node1: usize = nodes[0].parse::<usize>().unwrap();
        let node2: usize = nodes[1].parse::<usize>().unwrap();
        graphvec.push((node1, node2));
    }
    return graphvec;
}

fn read_file32(path: &str) -> Vec<(u32, u32)> {
    let mut graphvec32: Vec<(u32, u32)> = Vec::new();
    let file = File::open(path).expect("file cannot open");
    let filereader = std::io::BufReader::new(file).lines();
    for line in filereader {  
        let linemsg = line.expect("Error");
        let nodes: Vec<&str> = linemsg.trim().split(' ').collect();
        let node1= nodes[0].parse::<u32>().unwrap();
        let node2= nodes[1].parse::<u32>().unwrap();
        graphvec32.push((node1, node2));
    }
    return graphvec32;
}


//Breadth search function that takes in a graph and returns a hashmap with the distances
fn bfs(graph: &HashMap<u32, Vec<u32>>, start: u32) -> HashMap<u32, u32> {
    let mut search = HashMap::new();
    let mut graphqueue = VecDeque::new();

    graphqueue.push_back(start);
    search.insert(start, 0);

    while !graphqueue.is_empty() {
        let vertex = graphqueue.pop_front().unwrap();
        let distance = *search.get(&vertex).unwrap();

        for neighbor in graph.get(&vertex).unwrap() {
            if !search.contains_key(neighbor) {
                search.insert(*neighbor, distance + 1);
               graphqueue.push_back(*neighbor);
            }
        }
    }
    search
}
   


//HashMap creation
fn graphcreate(graphvec: &Vec<(u32, u32)>) -> HashMap<u32, Vec<u32>>{
    let mut graph = HashMap::new();
    for &(a, b) in graphvec.iter() {
        graph.entry(a).or_insert(vec![]).push(b);
    }
    graph
}

fn graphcreate2(graphvec: &Vec<(usize, usize)>) -> HashMap<usize, Vec<usize>>{
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




   

            
            
        


   




fn main() {

    let file     = read_file("googlegraph.txt");
    let file32= read_file32("googlegraph.txt");
    let graph32  = graphcreate(&file32);
    let graph    = graphcreate2(&file);
    let distance_bds = bfs(&graph32,0);

    println!("Graph: {:?}", graph32);
    println!("Graph32: {:?}", graph32);



    



 

// Adjacency List

    let graph_adj_list = adj_list(&file);
    println!("Adjacency List: {:?}", graph_adj_list); 

//Counting Triangles
    let triangle_count = triangles(&graph_adj_list);
    println!("Triangles: {:?}", triangle_count);

//6 degree of separation

  


}            


