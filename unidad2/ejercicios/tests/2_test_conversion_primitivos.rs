#[cfg(test)]
mod conversion_tests {

    #[test]
    fn test_widening_conversion_as() {
        // Objetivo: Practicar la conversión "widening" (ampliación) donde no se pierde información.

        // 1. Declara una variable `small_number` de tipo u8 con el valor 100.
        let small_number: u8 = 100;

        // 2. Convierte `small_number` a un tipo u32 usando la palabra clave `as`.
        // Esta conversión es segura porque u32 puede contener todos los valores de u8.
        let small_number_u32: u32 = small_number as u32;

        // 3. Verifica que la conversión fue exitosa.
        assert_eq!(small_number, 100);
        assert_eq!(small_number_u32, 100);

        // 4. Declara un i16 y conviértelo a i64.
        let number: i16 = -27;
        let number_i64: i64 = number as i64;

        assert_eq!(number, -27);
        assert_eq!(number_i64, -27);
    }

    #[test]
    fn test_narrowing_conversion_as() {
        // Objetivo: Entender los riesgos de la conversión "narrowing" (reducción).

        // 1. Declara una variable `big_number` de tipo i32 con el valor 300.
        let big_number: i32 = 300;

        // 2. Convierte `big_number` a un tipo i8 usando `as`.
        // ¡Atención! i8 solo puede contener valores de -128 a 127.
        // La conversión truncará los bits. 300 en binario es 0001 0010 1100.
        // Al truncar a 8 bits, nos quedamos con 0010 1100, que es 44 en decimal.
        let big_number_i8: i8 = big_number as i8;

        // 3. Verifica el resultado del truncamiento.
        assert_eq!(big_number_i8, 44);

        // 4. ¿Qué pasa con un número negativo?

        // 1000 en binario es 0011 1110 1000. Truncado a 8 bits es 1110 1000.
        // En complemento a dos, 1110 1000 representa -24.
        let big_number_2: i32 = 1000;
        let big_number_2_i8: i8 = big_number_2 as i8;
        assert_eq!(big_number_2_i8, -24)

    }

    #[test]
    fn test_float_to_integer_conversion() {
        // Objetivo: Observar cómo se maneja la conversión de flotante a entero.

        // 1. Declara una variable `pi` de tipo f64.
        let pi: f64 = 3.14159;


        // 2. Convierte `pi` a un i32 usando `as`.
        // La parte decimal se trunca (se elimina, no se redondea).
        let pi_as_int: i32 = pi as i32;

        // 3. Verifica que la parte decimal fue eliminada.
        assert_eq!(pi_as_int, 3);

        // 4. Prueba con un número negativo.
        let negative_float: f64 = -2.71828;
        let negative_int: i32 = negative_float as i32;
        assert_eq!(negative_int, -2);
    }

    #[test]
    fn test_integer_to_float_conversion() {
        // Objetivo: Practicar la conversión de entero a flotante.

        // 1. Declara un entero `my_integer`.
        let my_integer: i32 = 42;

        // 2. Conviértelo a f64. Esta conversión generalmente es segura y no pierde precisión
        // para enteros de 32 bits.
        let my_float: f64 = my_integer as f64;

        // 3. Verifica la conversión.
        assert_eq!(my_float, 42.0);

        // 4. Para enteros muy grandes (i64), la conversión a f64 puede perder precisión.
        let big_integer: i64 = 9007199254740991; // Límite de precisión para f64
        let float_from_big: f64 = big_integer as f64;

        let bigger_integer: i64 = 9007199254740993; // Un número mayor
        let imprecise_float: f64 = bigger_integer as f64;

        assert_eq!(float_from_big, 9007199254740991.0);

        // Debido a la falta de precisión, el resultado de la conversión no es exacto.
        // El entero se redondea al valor f64 más cercano representable.
        assert_eq!(imprecise_float, 9007199254740992.0, "La conversión debería redondear al flotante más cercano.");
    }

    #[test]
    fn test_from_trait_conversion() {
        // Objetivo: Usar el trait `From` para conversiones seguras y explícitas.
        // `From` es una forma más idiomática y segura de realizar conversiones "widening".

        // 1. Declara un u8.
        let small_number: u8 = 200;

        // 2. Usa `u16::from()` para convertir el u8 a u16.
        let wider_number: u16 = u16::from(small_number);
        assert_eq!(wider_number, 200);

        // 3. Haz lo mismo para convertir un i8 a i32.
        let signed_byte: i8 = -42;
        let wider_signed: i32 = i32::from(signed_byte);
        assert_eq!(wider_signed, -42);

        // 4. El trait `Into` es la contraparte de `From`. Si `T::from(U)` está implementado,
        // entonces `U` tiene una implementación de `into()` para `T`.
        let value: u8 = 100;
        let converted: u16 = value.into(); // Internamente usa u16::from(value)
        assert_eq!(converted, 100);

        let value2: i8 = -8;
        let converted2: i32 = value2.into(); // Internamente usa i32::from(value2)
        assert_eq!(converted2, -8);
    }
}
