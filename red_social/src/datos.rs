use crate::grafo::Red;

/// Pobla la red con personas y amistades que representan
/// una red social simplificada de 10 personas.
///
///  Grafo de amistades:
///
///   Ana ── Bruno ── Carlos ── Diana
///    |       |                  |
///   Eva    Fran               Gael
///    |                         |
///  Hugo ── Iris ─────────── Julia
///
pub fn construir_red() -> Red {
    let mut red = Red::nueva();

    // --- Personas ---
    let personas = [
        "Ana", "Bruno", "Carlos", "Diana",
        "Eva",  "Fran",  "Gael",   "Hugo",
        "Iris", "Julia",
    ];
    for p in &personas {
        red.agregar_persona(p);
    }

    // --- Amistades ---
    let amistades = [
        ("Ana",   "Bruno"),
        ("Ana",   "Eva"),
        ("Bruno", "Carlos"),
        ("Bruno", "Fran"),
        ("Carlos","Diana"),
        ("Diana", "Gael"),
        ("Eva",   "Hugo"),
        ("Hugo",  "Iris"),
        ("Iris",  "Julia"),
        ("Gael",  "Julia"),   // <-- crea un ciclo: Diana-Gael-Julia-Iris-Hugo-Eva-Ana-Bruno-Carlos-Diana
    ];
    for (a, b) in &amistades {
        red.agregar_amistad(a, b);
    }

    red
}
