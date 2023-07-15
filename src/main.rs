use regex::Regex;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::time::SystemTime;

struct Edge {
    src: u32,
    dst: u32,
    weight: i32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

impl Eq for Edge {
    fn assert_receiver_is_total_eq(&self) {
        // do nothing
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.weight.cmp(&other.weight);
    }
}

impl std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Edge")
            .field("src", &self.src)
            .field("dst", &self.dst)
            .field("peso", &self.weight)
            .finish()
    }
}

fn prim(adjacency_matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut edges_graph = Vec::new();

    for i in 0..adjacency_matrix.len() {
        for j in (i + 1)..adjacency_matrix.len() {
            if adjacency_matrix[i][j] != 0 {
                edges_graph.push(Reverse(Edge {
                    src: i as u32,
                    dst: j as u32,
                    weight: adjacency_matrix[i][j],
                }));
            }
        }
    }

    // Sort edges by weight
    edges_graph.sort();

    let vertices_num = adjacency_matrix.len();
    let mut non_visited: Vec<bool> = Vec::new();
    let mut mst: Vec<Edge> = Vec::new();

    for edge in edges_graph.iter() {
        println!(
            "src: {} dst: {} peso: {}",
            edge.0.src, edge.0.dst, edge.0.weight
        );
    }
    println!("\nVertices: {}\n", vertices_num);

    // Initialize non-visited vertices
    for _ in 0..vertices_num {
        non_visited.push(true);
    }

    let first_edge = edges_graph.pop().unwrap().0;

    non_visited[first_edge.src as usize] = false;
    non_visited[first_edge.dst as usize] = false;

    mst.push(first_edge);

    for i in 0..non_visited.len() {
        println!("Non-visited {}: {}", i, non_visited[i]);
    }

    while mst.len() < vertices_num - 1 {
        let edge_to_work = edges_graph.pop().unwrap().0;

        if !(non_visited[edge_to_work.src as usize] && non_visited[edge_to_work.dst as usize])
            && (non_visited[edge_to_work.src as usize] || non_visited[edge_to_work.dst as usize])
        {
            non_visited[edge_to_work.src as usize] = false;
            non_visited[edge_to_work.dst as usize] = false;

            mst.push(edge_to_work);

            edges_graph.sort();
        } else {
            edges_graph.push(Reverse(edge_to_work));
        }
    }

    println!("\n------------MST----------");
    for edge in mst {
        println!("src: {} dst: {} peso: {}", edge.src, edge.dst, edge.weight);
    }

    let temp: Vec<Vec<i32>> = Vec::new();
    temp
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Você deve inserir um path para um arquivo de entrada e um valor de debug!");
        exit(exitcode::USAGE);
    }

    // Pega o argumento de arquivo
    let filepath = &args[1];

    println!("Arquivo: {}", filepath);
    let f = File::open(filepath);

    let mut f = match f {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Arquivo não encontrado!");
            exit(exitcode::IOERR);
        }
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Erro ao ler o arquivo!");

    println!("Conteudo:\n{}\n", contents);

    // Pegando os valores da matriz com regex
    let regex = Regex::new(r"\d+").unwrap();
    let mut adjacency_matrix: Vec<Vec<i32>> = Vec::new();

    for line in contents.lines() {
        let temp = regex
            .find_iter(line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();

        adjacency_matrix.push(temp);
    }

    // Calculando tamanho da matriz
    let tam_matriz: u32 = adjacency_matrix.len() as u32;

    prim(adjacency_matrix);
}
