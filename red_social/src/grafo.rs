use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;

/// Tipo alias para nuestro grafo no dirigido con etiquetas de texto
pub type RedSocial = Graph<String, (), Undirected>;

/// Estructura principal que encapsula el grafo y el mapa de nombres → nodos
pub struct Red {
    pub grafo: RedSocial,
    pub nodos: HashMap<String, NodeIndex>,
}

impl Red {
    /// Crea una red social vacía
    pub fn nueva() -> Self {
        Red {
            grafo: Graph::new_undirected(),
            nodos: HashMap::new(),
        }
    }

    /// Agrega una persona a la red (nodo). Devuelve el índice del nodo.
    pub fn agregar_persona(&mut self, nombre: &str) -> NodeIndex {
        if let Some(&idx) = self.nodos.get(nombre) {
            return idx; // ya existe, no duplicar
        }
        let idx = self.grafo.add_node(nombre.to_string());
        self.nodos.insert(nombre.to_string(), idx);
        idx
    }

    /// Conecta dos personas como amigos (arista).
    /// Si alguna no existe, la crea automáticamente.
    pub fn agregar_amistad(&mut self, a: &str, b: &str) {
        let idx_a = self.agregar_persona(a);
        let idx_b = self.agregar_persona(b);
        // Evitar aristas duplicadas
        if !self.grafo.contains_edge(idx_a, idx_b) {
            self.grafo.add_edge(idx_a, idx_b, ());
        }
    }

    /// Obtiene el NodeIndex de una persona, si existe
    pub fn obtener_nodo(&self, nombre: &str) -> Option<NodeIndex> {
        self.nodos.get(nombre).copied()
    }
}
