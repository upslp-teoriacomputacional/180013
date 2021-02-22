/* *****************************************************************************
 *  Nombre:     Nahomi Itzel Medellin Salas
 *  Matrícula:  180013
 *  Carrera:    Tecnologías de la Información
 *  Escuela:    Universidad Politécnica de San Luis Potosí
 *
 *  Materia:    Teoría Computacional
 *  Maestro:    Juan Carlos González Ibarra              
 *                
 *  Descripción: This program validate chars to alphabet {a-z, A-Z} and digits {0-9}
 *
 *  Creado:       19/02/2020
 *  Última actualización:  21/02/2020
 *
 * ***************************************************************************** */

fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}
 
fn main() {
    let strings = [
        "",
        "...())&&",
        "abcDEFG",
        "XYZ",
        "1234567890ABCDEFGHIJKLMN0PQRSTUVWXYZ",
        "$%&()(((//",
        "hétrognUitö",
        "jih7 88",
        "$%$$&/",
    ];
 
    for strings in &strings {
        print!("\"{}\" (length {})", strings, strings.chars().count());
        match unique(strings) {
            None => println!(" TRUE"),
            Some((i, j, c)) => println!(
                " FALSE"
                
            ),
        }
    }
}