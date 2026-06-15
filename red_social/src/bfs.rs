use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, VecDeque};

use crate::grafo::Red;

/// Resultado de una búsqueda BFS
pub struct ResultadoBFS {
    pub origen: String,
    pub destino: String,
    pub ruta: Vec<String>,      // nombres en orden
    pub pasos: usize,           // cantidad de conexiones (aristas)
}

/// Ejecuta BFS desde `origen` hasta `destino` en la red.
/// Devuelve `Some(ResultadoBFS)` si existe ruta, `None` si no.
pub fn bfs(red: &Red, origen: &str, destino: &str) -> Option<ResultadoBFS> {
    let nodo_origen  = red.obtener_nodo(origen)?;
    let nodo_destino = red.obtener_nodo(destino)?;

    // padre[n] = nodo que descubrió n (para reconstruir ruta)
    let mut padre: HashMap<NodeIndex, Option<NodeIndex>> = HashMap::new();
    let mut cola: VecDeque<NodeIndex> = VecDeque::new();

    padre.insert(nodo_origen, None);
    cola.push_back(nodo_origen);

    'bfs: while let Some(actual) = cola.pop_front() {
        if actual == nodo_destino {
            break 'bfs;
        }
        for arista in red.grafo.edges(actual) {
            let vecino = arista.target();
            if !padre.contains_key(&vecino) {
                padre.insert(vecino, Some(actual));
                cola.push_back(vecino);
            }
        }
    }

    // ¿llegamos al destino?
    if !padre.contains_key(&nodo_destino) {
        return None;
    }

    // Reconstruir ruta al revés
    let mut ruta_idx = Vec::new();
    let mut cur = nodo_destino;
    loop {
        ruta_idx.push(cur);
        match padre[&cur] {
            None => break,
            Some(p) => cur = p,
        }
    }
    ruta_idx.reverse();

    let ruta: Vec<String> = ruta_idx
        .iter()
        .map(|idx| red.grafo[*idx].clone())
        .collect();

    let pasos = ruta.len() - 1;

    Some(ResultadoBFS {
        origen: origen.to_string(),
        destino: destino.to_string(),
        ruta,
        pasos,
    })
}

/// Imprime el resultado BFS de forma legible
pub fn mostrar_bfs(resultado: &Option<ResultadoBFS>) {
    match resultado {
        None => println!("  ✗ No existe ruta entre los nodos indicados."),
        Some(r) => {
            println!("  BFS | {} → {}", r.origen, r.destino);
            println!("  Ruta  : {}", r.ruta.join(" → "));
            println!("  Pasos : {} conexión/es", r.pasos);
        }
    }
}