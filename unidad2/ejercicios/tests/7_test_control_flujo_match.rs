#[cfg(test)]
mod match_statement_tests {

    // `match` es una de las construcciones de control de flujo más poderosas de Rust.
    // Permite comparar un valor con una serie de patrones y ejecutar código basado en qué patrón coincide.
    // Es como un `switch` en otros lenguajes, pero mucho más potente.

    #[test]
    fn test_match_basico() {
        // Objetivo: Entender la sintaxis básica de `match`.

        let numero = 3;
        let resultado: &str;

        // 1. Usa `match` para comparar `numero` con diferentes valores.
        resultado = match numero {
                        1 => "Uno",
                        2 => "Dos",
                        3 => "Tres",
                        4 | 5 => "Cuatro o Cinco", 
                        _ => "Otro número",       
                    };

    // 2. Verifica que el resultado sea el esperado.
    assert_eq!(resultado, "Tres");
    }

    #[test]
    fn test_match_con_option() {
        // Objetivo: Usar `match` para manejar el tipo `Option<T>` de forma segura.

        fn obtener_numero(val: bool) -> Option<i32> {
            if val {
                Some(42)
            } else {
                None
            }
        }

        // 1. Usa `match` para manejar el caso `Some`.
        let resultado_algun_numero: String = match obtener_numero(true) {
            Some(valor) => format!("El número es {}", valor),
            None => "No hay número".to_string(),
        };
        assert_eq!(resultado_algun_numero, "El número es 42");

        // 2. Usa `match` para manejar el caso `None`.
        let resultado_sin_numero: String = match obtener_numero(false) {
            Some(valor) => format!("El número es {}", valor),
            None => "No hay número".to_string(),
        };
        assert_eq!(resultado_sin_numero, "No hay número");
    }

    #[test]
    fn test_match_con_multiples_patrones_y_rangos() {
        // Objetivo: Aprender a combinar patrones y usar rangos.
        let numero = 5;

        let descripcion = match numero {
            1 | 2 => "Es uno o dos", // El operador `|` permite múltiples patrones.
            3..=6 => "Está entre 3 y 6 (incluidos)", // `..=` define un rango inclusivo.
            _ => "Es otro número",
        };

        assert_eq!(descripcion, "Está entre 3 y 6 (incluidos)");
    }

}
