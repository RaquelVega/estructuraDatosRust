use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::cmp::Reverse;

/// Tipo de transporte entre estaciones
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum TipoTransporte {
    Metro,
    Bus,
    Tren,
}

/// Información de una estación
#[derive(Debug, Clone)]
struct Estacion {
    nombre: String,
    zona: u8,
}

/// Red de transporte: grafo ponderado
struct RedTransporte {
    // Lista de adyacencia: estación -> lista de (destino, costo_minutos)
    conexiones: HashMap<String, Vec<(String, u32)>>,
    // Información adicional de cada estación
    estaciones: HashMap<String, Estacion>,
}

impl RedTransporte {
    fn new() -> Self {
        RedTransporte {
            conexiones: HashMap::new(),
            estaciones: HashMap::new(),
        }
    }

    /// Agrega una estación a la red
    fn agregar_estacion(&mut self, nombre: &str, zona: u8) {
        self.estaciones.insert(
            nombre.to_string(),
            Estacion { nombre: nombre.to_string(), zona },
        );
        self.conexiones.entry(nombre.to_string()).or_insert_with(Vec::new);
    }

    /// Agrega una conexión bidireccional con costo en minutos
    fn agregar_conexion(&mut self, a: &str, b: &str, minutos: u32) {
        self.conexiones
            .entry(a.to_string())
            .or_insert_with(Vec::new)
            .push((b.to_string(), minutos));
        self.conexiones
            .entry(b.to_string())
            .or_insert_with(Vec::new)
            .push((a.to_string(), minutos));
    }

    /// BFS: ruta con menor número de transbordos (saltos)
    fn ruta_menos_saltos(&self, origen: &str, destino: &str) -> Option<Vec<String>> {
        let mut visitados: HashSet<String> = HashSet::new();
        let mut cola: VecDeque<(String, Vec<String>)> = VecDeque::new();

        cola.push_back((origen.to_string(), vec![origen.to_string()]));
        visitados.insert(origen.to_string());

        while let Some((actual, camino)) = cola.pop_front() {
            if actual == destino {
                return Some(camino);
            }
            if let Some(vecinos) = self.conexiones.get(&actual) {
                for (vecino, _) in vecinos {
                    if !visitados.contains(vecino) {
                        visitados.insert(vecino.clone());
                        let mut nuevo_camino = camino.clone();
                        nuevo_camino.push(vecino.clone());
                        cola.push_back((vecino.clone(), nuevo_camino));
                    }
                }
            }
        }
        None
    }

    /// Dijkstra: ruta con menor costo total en minutos
    fn ruta_mas_rapida(&self, origen: &str, destino: &str) -> Option<(Vec<String>, u32)> {
        let mut distancias: HashMap<String, u32> = HashMap::new();
        let mut anteriores: HashMap<String, String> = HashMap::new();
        let mut heap: BinaryHeap<Reverse<(u32, String)>> = BinaryHeap::new();

        for estacion in self.estaciones.keys() {
            distancias.insert(estacion.clone(), u32::MAX);
        }
        distancias.insert(origen.to_string(), 0);
        heap.push(Reverse((0, origen.to_string())));

        while let Some(Reverse((costo_actual, nodo))) = heap.pop() {
            if nodo == destino {
                // Reconstruir camino
                let mut camino = vec![destino.to_string()];
                let mut actual = destino.to_string();
                while let Some(prev) = anteriores.get(&actual) {
                    camino.push(prev.clone());
                    actual = prev.clone();
                }
                camino.reverse();
                return Some((camino, costo_actual));
            }
            if costo_actual > *distancias.get(&nodo).unwrap_or(&u32::MAX) {
                continue;
            }
            if let Some(vecinos) = self.conexiones.get(&nodo) {
                for (vecino, peso) in vecinos {
                    let nuevo = costo_actual + peso;
                    if nuevo < *distancias.get(vecino).unwrap_or(&u32::MAX) {
                        distancias.insert(vecino.clone(), nuevo);
                        anteriores.insert(vecino.clone(), nodo.clone());
                        heap.push(Reverse((nuevo, vecino.clone())));
                    }
                }
            }
        }
        None
    }

    /// Muestra info de una estación
    fn info_estacion(&self, nombre: &str) {
        if let Some(est) = self.estaciones.get(nombre) {
            println!("🚉 {} (Zona {})", est.nombre, est.zona);
        }
    }
}

fn main() {
    let mut red = RedTransporte::new();

    // Construir red de ejemplo
    let estaciones = [
        ("Centro", 1), ("Norte", 1), ("Sur", 2),
        ("Este", 2), ("Oeste", 1), ("Aeropuerto", 3),
        ("Universidad", 2), ("Hospital", 1),
    ];
    for (nombre, zona) in estaciones.iter() {
        red.agregar_estacion(nombre, *zona);
    }

    // Conexiones (estación_a, estación_b, minutos)
    let conexiones = [
        ("Centro", "Norte", 5),
        ("Centro", "Sur", 8),
        ("Centro", "Oeste", 4),
        ("Centro", "Hospital", 3),
        ("Norte", "Este", 7),
        ("Sur", "Universidad", 6),
        ("Este", "Aeropuerto", 15),
        ("Oeste", "Aeropuerto", 20),
        ("Universidad", "Aeropuerto", 10),
        ("Hospital", "Norte", 6),
    ];
    for (a, b, t) in conexiones.iter() {
        red.agregar_conexion(a, b, *t);
    }

    println!("== ANALIZADOR DE RUTAS DE TRANSPORTE ===\n");

    let origen = "Centro";
    let destino = "Aeropuerto";

    red.info_estacion(origen);
    red.info_estacion(destino);
    println!();

    // BFS
    match red.ruta_menos_saltos(origen, destino) {
        Some(ruta) => {
            println!("Ruta con MENOS TRANSBORDOS ({} paradas):", ruta.len() - 1);
            println!("   {}", ruta.join(" → "));
        }
        None => println!("No existe ruta."),
    }

    // Dijkstra
    match red.ruta_mas_rapida(origen, destino) {
        Some((ruta, minutos)) => {
            println!("\n Ruta MÁS RÁPIDA ({} minutos):", minutos);
            println!("   {}", ruta.join(" → "));
        }
        None => println!("No existe ruta."),
    }
}
