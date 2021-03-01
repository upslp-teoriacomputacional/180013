/* *****************************************************************************
 *  Nombre:     Nahomi Itzel Medellin Salas
 *  Matrícula:  180013
 *  Carrera:    Tecnologías de la Información
 *  Escuela:    Universidad Politécnica de San Luis Potosí
 *
 *  Materia:    Teoría Computacional
 *  Maestro:    Juan Carlos González Ibarra              
 *                
 *  Descripción: Operaciones con cadenas en lenguaje Rust
 *
 *  Creado:       25/02/2020
 *  Última actualización:  28/02/2020
 *
 * ***************************************************************************** */
 //use titlecase::titlecase;//capital letters
//std::fmt//macro assert_eq

fn main() {
  //otra forma de declarar cadenas
    let text1 = r##"RUST like this"##;
    println!("{}",text1);//RUST like this

  //declarar cadena con doble comilla
    let text2 = "RUST in Double Quotes";
    println!("{}",text2);//RUST in Double Quotes

    let message = "Hello, Python!";
    //pasar de una cadena a un vector para poder indexarlo
    let my_vec: Vec<char> = message.chars().collect();
    println!("my_vec[7]: {}", my_vec[7]); //P

    /* Text            =   P Y T H O N   
       Positive Index =   0 1 2 3 3 5   
       Negative Index = -(5 3 3 2 1 0)
    */ 

    let message2 = "PYTHON";
    let my_vec2: Vec<char> = message2.chars().collect();
    println!("my_vec[3]: {}", my_vec2[3]);//H


    //invertir la cadena para obtener posiciones negativas
    let cadenaRev: String = message2.chars().rev().collect();
    let my_vec3: Vec<char> = cadenaRev.chars().collect();
    println!("cadena invertida: {}", my_vec3[3]);//T

    //imprime solo una parte de la cadena
    let s = "Rust Language";
    let ss = &s[5..13];
    println!("{:?}", ss);//Language

    
    //quita los espacios de una cadena
    let s2 = "       Rust language     ";
    let palabra: String = s2.split_whitespace().collect();
    println!("palabra sin espacio {}", palabra);//Rust language

    //cadena en minuscula
    let s3 = "RUST STRING";
    println!("lower case string: {}",s3.to_lowercase());//rust string
    
    let s3 = "rust string";
    println!("Upper case string: {}",s3.to_uppercase());//RUST STRING

    //longitud de la cadena
    let s3 = "Rust Language";
    println!("Longitd: {}", s3.chars().count());//13

    //reemplazar una cadena
    println!("Cadena reemplazada {}",message.replace("Python", "Rust"));//Hello, Rust!

    //separar cadenas
    let strings= "Python, tutorials, with, dotnettechpoint.com".split(","); 
    for s in strings {
      print!("{}", s);
    }
    // [Python, tutorials, with, dotnettechpoint.com]
    println!("");
    //title 
    let texte = "a sample title to capitalize: an example";
    println!("{}",titlecase(text));//A sample title to capitalize: an example

    //count 
    let text3 = "python tutorials with dotnettechpoint.com";
    let c = text3.matches('t').count();
    println!("contar T: {}", c);//8
    
    //find a word
    println!("{:?}",text3.rfind("with"));//17
    //println!(text3.rfind("whith"));

  //concatenar
    let word1="Python";
    let word2="Tutorial";
    let together = format!("{} {}", word1, word2);
    println!("{}", together);

    let word3 = "3";
    let together1 = format!("{} {} {}", word1,word3, word2);
    println!("{}", together1);

    let number = 4;
    let s: String = number.to_string();
    let together2 = format!("{} {} {}",word1, s, word2);
    println!("{}", together2);

}
