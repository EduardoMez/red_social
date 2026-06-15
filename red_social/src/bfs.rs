use crate::grafo::Red;
use petgraph::graph::NodeIndex;
use std::collections::{HashSet, VecDeque};

/// Realiza un recorrido BFS sobre la red social a partir de una persona inicial
///
/// El recorrido visita primero todos los amigos directos,
/// luego los amigos de esos amigos, y así sucesivamente
///
///      Ana
///     /   \
///  Bruno  Eva
///   |       |
/// Carlos  Hugo
///
/// BFS desde Ana:
///
/// Ana → Bruno → Eva → Carlos → Hugo
/// 
pub fn bfs(red: &Red, inicio: &str) {
    let inicio_idx = match red.obtener_nodo(inicio) {
        Some(idx) => idx,
        None => {
            println!("La persona '{}' no existe en la red.", inicio);
            return;
        }
    };

    let mut visitados: HashSet<NodeIndex> = HashSet::new();
    let mut cola: VecDeque<NodeIndex> = VecDeque::new();

    visitados.insert(inicio_idx);
    cola.push_back(inicio_idx);

    println!("Recorrido BFS desde {}:", inicio);

    while let Some(actual) = cola.pop_front() {
        println!("{}", red.grafo[actual]);

        for vecino in red.grafo.neighbors(actual) {
            if !visitados.contains(&vecino) {
                visitados.insert(vecino);
                cola.push_back(vecino);
            }
        }
    }
}