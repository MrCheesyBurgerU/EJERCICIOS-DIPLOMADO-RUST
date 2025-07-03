#[cfg(test)]
mod string_slice_tests {

    // En Rust, hay dos tipos principales de strings: `String` y `&str`.
    // `&str` (string slice): Es una referencia a una secuencia de bytes. Es inmutable y prestado.
    // `String`: Es un tipo poseído (owned), modificable y que se almacena en el heap.

    #[test]
    fn test_crear_string_y_str() {
        // Objetivo: Entender las formas básicas de crear `String` y `&str`.

        // 1. `&str` se crea a partir de literales de cadena.
        let mi_slice: &str = "Hola, mundo!";
        assert_eq!(mi_slice, "Hola, mundo!");

        // 2. `String` se puede crear a partir de un `&str` usando `.to_string()`.
        let my_string: String = mi_slice.to_string();
        assert_eq!(my_string, "Hola, mundo!");

        // 3. También se puede usar `String::from()`.
        let my_string_2: String = String::from("Hola, mundo!");
        assert_eq!(my_string_2, "Hola, mundo!");

        // 4. Un `&str` puede ser una vista de una parte de un `String`.
        let subcadena: &str = &my_string_2[0..4]; // "Hola"
        assert_eq!(subcadena, "Hola");
    }

    #[test]
    fn test_modificar_un_string() {
        // Objetivo: Aprender a modificar un `String` mutable.

        // 1. Para modificar un `String`, debe ser mutable (`mut`).
        let mut saludo = String::from("Hola");

        // 2. Usa `push_str()` para añadir un slice de string (`&str`).
        saludo.push_str(", mundo");

        // 3. Usa `push()` para añadir un solo carácter (`char`).
        saludo.push('!');

        assert_eq!(saludo, "Hola, mundo!");
    }

    #[test]
    fn test_concatenacion_con_operador_mas() {
        // Objetivo: Entender cómo funciona la concatenación con el operador `+`.

        let s1 = String::from("Hola, ");
        let s2 = String::from("mundo!");

        // El operador `+` toma posesión (ownership) del primer operando (`s1`).
        // `s1` es movido y ya no se puede usar después de esta línea.
        // El segundo operando (`s2`) es una referencia (`&String`), por lo que no se mueve.
        let resultado = s1 + &s2; // `s1` es movido aquí y ya no se puede usar.
        assert_eq!(resultado, "Hola, mundo!");

        // Descomenta la siguiente línea y verás un error de compilación porque `s1` fue movido.
        // println!("{}", s1);
    }

    #[test]
    fn test_concatenacion_con_macro_format() {
        // Objetivo: Usar la macro `format!` para concatenar strings sin tomar posesión.

        let s1 = String::from("Tic");
        let s2 = String::from("Tac");
        let s3 = String::from("Toe");

        // 1. Usa la macro `format!` para combinar `s1`, `s2` y `s3`.
        // `format!` no toma posesión de sus argumentos, por lo que `s1`, `s2` y `s3` siguen siendo válidos.
        let resultado = format!("{}-{}-{}", s1, s2, s3);

        // 2. Verifica el resultado.
        assert_eq!(resultado, "Tic-Tac-Toe");

        // 3. Verifica que `s1`, `s2` y `s3` todavía se pueden usar.
        assert_eq!(s1, "Tic");
        assert_eq!(s2, "Tac");
        assert_eq!(s3, "Toe");
    }
}
