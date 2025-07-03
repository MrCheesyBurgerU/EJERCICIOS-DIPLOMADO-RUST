#[cfg(test)]
mod if_statement_tests {

    // La construcción `if` permite bifurcar el código dependiendo de una condición.
    // En Rust, `if` es una expresión, lo que significa que puede devolver un valor.

    #[test]
    fn test_if_else_basico() {
        // Objetivo: Entender la estructura básica de un `if-else`.

        let mut numero = 10;
        let  mut resultado: &str;

        // 1. Escribe una condición `if` para verificar si `numero` es mayor que 5.
        if numero > 5 {
            resultado = "Número es mayor que 5";
        }
        else {
            resultado = "Número es menor o igual que 5";
        }

        // 2. Verifica que el resultado es el esperado.
        assert_eq!(resultado, "Número es mayor que 5");

        // 3. Haz lo mismo con un número que no cumpla la condición.
        numero = 4;

        if numero > 5 {
            resultado = "Número es mayor que 5";
        }
        else {
            resultado = "Número es menor o igual que 5";
        }

         assert_eq!(resultado, "Número es menor o igual que 5");
    }

    #[test]
    fn test_if_else_if_else() {
        // Objetivo: Manejar múltiples condiciones con `else if`.

        let n = 6;

        // 1. Escribe una cadena de `if-else if-else` para clasificar `n`.
        // - Si `n` es divisible por 4, devuelve "Divisible por 4".
        // - Si `n` es divisible por 3, devuelve "Divisible por 3".
        // - Si `n` es divisible por 2, devuelve "Divisible por 2".
        // - De lo contrario, devuelve "No es divisible por 2, 3 ni 4".
        let result = if n % 4 == 0 {
            "Divisible por 4"
        } else if n % 3 == 0 {
            "Divisible por 3"
        } else if n % 2 == 0 {
            "Divisible por 2"
        } else {
            "No es divisible por 2, 3 ni 4"
        };

        // 2. Verifica el resultado para `n = 6`.
        assert_eq!(result, "Divisible por 3");
    }

    #[test]
    fn test_if_en_una_declaracion_let() {
        // Objetivo: Usar `if` como una expresión para asignar un valor a una variable.

        let mut condicion = true;

        // 1. Usa una expresión `if` para asignar un valor a la variable `numero`.
        // Si `condicion` es verdadera, `numero` debe ser 5. Si no, debe ser 0.
        // Nota: Ambos bloques (`if` y `else`) deben devolver el mismo tipo.
        let number: i32 = if condicion {
            5
        }
        else {
            0
        };

        // 2. Verifica el valor de `numero`.
        assert_eq!(number, 5);

        // 3. Haz lo mismo con la condición en `false`.
        condicion = false;

        let number_2: i32 = if condicion {
            5
        }
        else {
            0
        };

        assert_eq!(number_2, 0);
    }

    #[test]
    fn test_condiciones_combinadas() {
        // Objetivo: Usar operadores lógicos `&&` (Y) y `||` (O) en las condiciones.

        let edad = 25;
        let tiene_licencia = true;

        // 1. Escribe una condición que verifique si la persona es mayor de edad (>= 18) Y tiene licencia.
        let result_1: &str = if edad >= 18 && tiene_licencia {
            "Es mayor de edad y tiene licencia"
        }
        else {
            "No cumple con ambas condiciones"
        };

        assert_eq!(result_1, "Es mayor de edad y tiene licencia");

        let es_estudiante = true;
        let es_fin_de_semana = false;

        // 2. Escribe una condición que verifique si la persona puede descansar: si es fin de semana O no es estudiante.
        let result_2: &str = if es_estudiante || es_fin_de_semana {
            "La persona puede descansar"
        }
        else {
            "La persona no cumple con las condiciones para descansar"
        };

        assert_eq!(result_2, "La persona puede descansar");
    }
}
