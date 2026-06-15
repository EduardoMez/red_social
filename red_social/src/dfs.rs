use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use std::collections::HashSet;

use crate::grafo::Red;

// resultado de la detección de ciclos
pub struct ResultadoDFS {
    pub hay_ciclo: bool,
    pub ciclo: Vec<String>, // nodos que forman el primer ciclo hallado (queda vacío si no hay)
}

// lanza DFS sobre toda la red y detecta si hay al menos un ciclo
pub fn detectar_ciclos(red: &Red) -> ResultadoDFS {
    let mut visitados: HashSet<NodeIndex> = HashSet::new();
    let mut ciclo_encontrado: Vec<String> = Vec::new();

    for nodo in red.grafo.node_indices() {
        if !visitados.contains(&nodo) {
            let mut camino: Vec<NodeIndex> = Vec::new();
            if dfs_rec(red, nodo, None, &mut visitados, &mut camino) {
                // extraer el ciclo desde camino
                ciclo_encontrado = camino
                    .iter()
                    .map(|idx| red.grafo[*idx].clone())
                    .collect();
                break;
            }
        }
    }

    ResultadoDFS {
        hay_ciclo: !ciclo_encontrado.is_empty(),
        ciclo: ciclo_encontrado,
    }
}

// función recursiva de DFS que detecta ciclos en grafos no dirigidos
// devuelve true si encontró un ciclo y rellena camino
fn dfs_rec(
    red: &Red,
    actual: NodeIndex,
    padre: Option<NodeIndex>,
    visitados: &mut HashSet<NodeIndex>,
    camino: &mut Vec<NodeIndex>,
) -> bool {
    visitados.insert(actual);
    camino.push(actual);

    for arista in red.grafo.edges(actual) {
        let vecino = arista.target();

        // ignorar la arista de vuelta al nodo padre (grafo no dirigido)
        if Some(vecino) == padre {
            continue;
        }

        if visitados.contains(&vecino) {
            // ciclo detectado: agregar el nodo de cierre y terminar
            camino.push(vecino);
            return true;
        }

        if dfs_rec(red, vecino, Some(actual), visitados, camino) {
            return true;
        }
    }

    camino.pop();
    false
}

// imprime el resultado DFS de forma legible
pub fn mostrar_dfs(resultado: &ResultadoDFS) {
    if resultado.hay_ciclo {
        println!("  DFS - Se detectó un CICLO en la red social.");
        println!("  Ciclo: {}", resultado.ciclo.join(" -> "));
    } else {
        println!("  DFS - No se encontraron ciclos (la red es un árbol/bosque).");
    }
}