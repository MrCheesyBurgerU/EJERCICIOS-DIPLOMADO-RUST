#[cfg(test)]
mod tests {

    #[test]
    fn test_booleans() {
        // Objetivo: Practicar la creación y uso de variables booleanas.

        // 1. Declaración de una variable llamada 'is_sunny' de tipo booleano con valor de 'true'.
        let is_sunny: bool = true;

        // 2. Declara una variable llamada 'is_weekend' y asígnale el valor 'false'.
        let is_weekend: bool = false;

        // 3. Escribe una aserción que verifique que 'is_sunny' es verdadero.
        assert!(is_sunny, "Se esperaba que 'is_sunny' fuera verdadero.");

        // 4. Escribe una aserción que verifique que 'is_weekend' es falso.
        assert!(!is_weekend, "Se esperaba que 'is_weekend' fuera falso.");
    }

    #[test]
    fn test_characters() {
        // Objetivo: Practicar la creación y manipulación de caracteres (`char`).

        // 1. Declara una variable 'initial' con tu primera inicial.
        let initial: char = 'N';

        // 2. Asegúrate de que tu inicial sea un carácter alfabético.
        assert!(initial.is_alphabetic(), "'initial' debería ser un carácter alfabético.");

        // 3. Declara una variable 'digit' con un carácter numérico (ej: '7').
        let digit: char = '7';

        // 4. Asegúrate de que 'digit' sea un carácter numérico.
        assert!(digit.is_ascii_digit(), "'digit' debería ser un carácter numérico.");

        // 5. Declara una variable 'symbol' con un carácter especial (ej: '$').
        let symbol: char = '$';

        // 6. Asegúrate de que 'symbol' no sea ni alfabético ni numérico.
        assert!(!symbol.is_alphanumeric(), "'symbol' no debería ser un carácter alfabético ni numérico.");

        // 7. Declara una variable 'emoji' con un emoji (ej: '🦀').
        let emoji = '🦀';

        // 8. Asegúrate de que 'emoji' no sea alfabético ni numérico (generalmente).
        assert!(!emoji.is_alphanumeric(), "'emoji' no debería ser un carácter alfabético ni numérico.");

        // 9. Declara una variable 'unicode_char' con un carácter Unicode (ej: 'あ').
        let unicode_char = 'あ';
        assert!(!unicode_char.is_ascii(), "'unicode_char' no debería ser ASCII.");
    }

    #[test]
    fn test_signed_integers() {
        // Objetivo: Familiarizarse con los tipos de enteros con signo.

        // 1. Declara una variable 'temperature' de tipo i32 con un valor de -10.
        let temperature: i32 = -10;
        assert_eq!(temperature, -10);

        // 2. Declara una variable 'small_number' de tipo i8 con el valor -5.
        let small_number: i8 = -5;
        assert_eq!(small_number, -5);

        // 3. Declara una variable 'big_number' de tipo i64 con un valor grande (ej: 1_000_000_000).
        // Nota: Los guiones bajos `_` se pueden usar para mejorar la legibilidad de números grandes.
        let big_number: i64 = 1_000_000_000;
        assert_eq!(big_number, 1_000_000_000);

        // 4. Suma 'temperature' y 'small_number' (convertido a i32) y almacénalo en 'result'.
        let result: i32 = temperature + small_number as i32;
        assert_eq!(result, -15);

        // 5. Investiga y verifica cuál es el valor mínimo que puede tener un i8.
        // Pista: Usa la constante `MIN` asociada al tipo i8::MIN.
        let min_i8: i8 = i8::MIN;
        assert_eq!(min_i8, -128);

    }

    #[test]
    fn test_unsigned_integers() {
        // Objetivo: Comprender los tipos de enteros sin signo.

        // 1. Declara una variable 'age' de tipo u32 con tu edad.
        let age: u32 = 22; // ¡Cambia este valor a tu edad!
        assert!(age >= 0, "La edad no puede ser negativa.");

        // 2. Declara una variable 'max_u8' de tipo u8 con su valor máximo posible.
        // Pista: Usa la constante `MAX` asociada al tipo. u8::MAX
        let max_u8: u8 = u8::MAX;
        assert_eq!(max_u8, 255);

        // 3. Declara una variable 'items_count' de tipo u64 con un valor de 150.
        let items_count: u64 = 150;
        assert_eq!(items_count, 150);

        // 4. Resta 50 de 'items_count' y almacénalo en 'remaining_items'.
        let remaining_items: u64 = items_count - 50;
        assert_eq!(remaining_items, 100);

        // 5. ¿Qué pasa si intentas restar un número mayor de 'items_count', como 200?
        // En modo de depuración, esto causará un 'panic'. En modo de lanzamiento (`--release`),
        // Rust usará "two's complement wrapping" (desbordamiento). No necesitas escribir código para esto, solo entenderlo.
    }

    #[test]
    fn test_floating_points() {
        // Objetivo: Trabajar con números de punto flotante.

        // 1. Declara una variable 'pi' de tipo f64 con el valor 3.14159.
        let pi: f64 = 3.14159;

        // 2. Declara una variable 'radius' de tipo f32 con el valor 5.5.
        let radius: f32 = 5.5;

        // 3. Calcula el área del círculo (pi * radio^2) y almacénala en 'area'.
        // Nota: Necesitarás convertir 'radius' a f64 para que coincida con 'pi'.
        let area: f64 = pi * (radius as f64).powi(2);

        // 4. Los números de punto flotante rara vez son exactos. Para compararlos,
        // es mejor verificar si están "cerca" uno del otro.
        // Pista: assert!((area - expected_area).abs() < 0.0001, "El área calculada no es la esperada.");
        let expected_area: f64 = 3.14159 * 5.5_f64.powi(2);
        assert!((area - expected_area).abs() < 0.0001, "El área calculada no es la esperada.");

        // 5. Realiza una división de flotantes. Divide 10.0 por 3.0.
        let result: f64 = 10.0 / 3.0;
        // Verifica que el resultado es aproximadamente 3.333...
        assert!((result - 3.3333).abs() < 0.0001, "El resultado no es aproximadamente 3.3333");
    }

}