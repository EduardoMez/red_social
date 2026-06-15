mod grafo;  
mod datos;  
mod bfs;     
mod dfs;     

use datos::construir_red;
use bfs::{bfs, mostrar_bfs};
use dfs::{detectar_ciclos, mostrar_dfs};

fn main() {
    println!("       RED SOCIAL — Grafos con Rust       ");
    println!("\n");

    //Construir la red social 
    let red = construir_red();

    //Mostrar personas y amistades 
    println!(" Personas que hay en la red:");
    let mut nombres: Vec<&String> = red.nodos.keys().collect();
    nombres.sort();
    for nombre in &nombres {
        let idx = red.nodos[*nombre];
        let amigos: Vec<String> = red
            .grafo
            .neighbors(idx)
            .map(|n| red.grafo[n].clone())
            .collect();
        println!("   {:<8} → amigos: {}", nombre, amigos.join(", "));
    }

    //BFS: ruta con menos conexiones
    println!("\n------------------------------------------");
    println!(" BFS — Ruta más corta (menos conexiones)");
    println!("------------------------------------------");

    let casos_bfs = [
        ("Ana",   "Julia"),
        ("Bruno", "Iris"),
        ("Fran",  "Diana"),
        ("Hugo",  "Carlos"),
    ];
    for (origen, destino) in &casos_bfs {
        let resultado = bfs(&red, origen, destino);
        mostrar_bfs(&resultado);
        println!();
    }

    //DFS: detección de ciclos
    println!("------------------------------------------");
    println!("DFS — Detección de ciclos en la red");
    println!("------------------------------------------");
    let resultado_dfs = detectar_ciclos(&red);
    mostrar_dfs(&resultado_dfs);

    println!("\n Programa finalizado.");
}