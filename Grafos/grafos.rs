/* *****************************************************************************
 *  Nombre:     Nahomi Itzel Medellin Salas
 *  Matrícula:  180013
 *  Carrera:    Tecnologías de la Información
 *  Escuela:    Universidad Politécnica de San Luis Potosí
 *
 *  Materia:    Teoría Computacional
 *  Maestro:    Juan Carlos González Ibarra              
 *                
 *  Descripción: Operaciones con conjuntos en lenguaje Rust
 *
 *  Creado:       12/02/2020
 *  Última actualización:  14/02/2020
 *
 * ***************************************************************************** */
use graphlib::Graph;
use std::io;
use petgraph::Graph; //librerias para poder acceder a los metodos de los grafos

let mut graph: Graph<usize> = Graph::new();

// agregando los vertices al grafo
let id1 = graph.add_vertex('a');
let id2 = graph.add_vertex('b');
let id3 = graph.add_vertex('c');
let id4 = graph.add_vertex('d');
let id5 = graph.add_vertex('e');

// agregando las aristas que van a ir entre los vertices
//id1 -> a
//id2 -> b 
//id3 -> c
//id4 -> d
//id5 -> e

graph.add_edge(&id1, &id3);
graph.add_edge(&id2, &id3);
graph.add_edge(&id2, &id5);
graph.add_edge(&id3, &id4);
graph.add_edge(&id3, &id5);
graph.add_edge(&id3, &id1);
graph.add_edge(&id3, &id2);
graph.add_edge(&id5, &id2);
graph.add_edge(&id4, &id3);
graph.add_edge(&id5, &id3);

assert_eq!(*graph.fetch(&id1).unwrap(), 'a');
assert_eq!(*graph.fetch(&id2).unwrap(), 'b');
assert_eq!(*graph.fetch(&id3).unwrap(), 'c');
assert_eq!(*graph.fetch(&id4).unwrap(), 'd');
assert_eq!(*graph.fetch(&id5).unwrap(), 'e');

