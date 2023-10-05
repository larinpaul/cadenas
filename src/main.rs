//// 2023/10/05 // 22:05 //

//// # 15 Cadena (Strings)

// Cuando los programadores en lenguaje Rust se refiren a "cadenas", por lo general
// se suelen referir a las cadenas de tipo String, los segmentos de cadena y &str, no solo a uno de esos tipos.

// En esta vídeo vamos a tratar principalmente sobre el tipo String,
// pero en la programación habitualmente también se usan mucho los segmentos.
// Ambos tipos, perteneces a la biblioteca estándar de Rust y ambos están codificados en UTF-8.

fn main() {

    // let mut cadena = String::new(); // Una cadena vacía


    // let datos = "contenido inicial";
    // let cadena = datos.to_string();


    // let cadena = "contenido inicial".to_string();


    // let cadena = String::from("contenido inicial");
    // let cadena_chino = String::from("你好世界");
    // let cadena_emoticonos = String::from("🌱☀️");
    // println!("{}", cadena);    
    // println!("{}", cadena_chino);    
    // println!("{}", cadena_emoticonos);    


    // let hola = String::from("hola ");
    // let mundo = String::from("mundo");
    // let hola_mundo = hola + &mundo;
    // println!("{}", hola_mundo);
    // let hola_mundo2 = hola_mundo + " " + &mundo;
    // println!("{}", hola_mundo2);


    // let hola = String::from("hola ");
    // let mundo = String::from("mundo");
    // let hola_mundo = format!("{}{}", hola, mundo);
    // println!("{}", hola_mundo);


    // let hola = String::from("hola");
    // let mundo = String::from("mundo");
    // let amigos = String::from("y amigos de todo el mundo");
    // let hola_mundo = format!("{} {} {}", hola, mundo, amigos);
    // println!("{}", hola_mundo);


    // let mut hola = String::from("hola ");
    // let mundo = String::from("mundo");
    // // hola.push_str(&mundo);
    // hola.push('1');
    // println!("{}", hola);


    let cadena = String::from("hola");
    // let caracter = cadena[1]; // Incorrecto

    for c in cadena.chars() {
        println!("{}", c);
    }

    for b in cadena.bytes() {
        println!("{}", b);
    }

    for c in cadena.chars() {
        if c == 'o' {
            println!("{}", c);
        }
    }

    for (index, c) in cadena.char_indices() {
        if index == 1 {
            println!("Index of 'o': {}", index);
        }
    }

    for (index, c) in cadena.char_indices() {
        if c == 'o' {
            println!("Index of 'o': {}", index);
        }
    }

}
