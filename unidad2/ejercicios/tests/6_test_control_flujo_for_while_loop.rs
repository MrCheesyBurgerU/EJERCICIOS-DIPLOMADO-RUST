#[cfg(test)]
mod loops_tests {

    #[test]
    fn test_loop_con_break() {
        // Objetivo: Entender el bucle `loop` y cómo usar `break` para salir y devolver un valor.

        let mut contador = 0;

        // 1. Usa un bucle `loop` para incrementar `contador`.
        //    El `loop` debe detenerse cuando `contador` llegue a 10.
        //    Cuando se detenga, debe devolver el valor de `contador` multiplicado por 2.
        let result: i32 = loop {
                            if contador == 10{
                                break contador * 2;
                            }
                            contador += 1;
                        };

        // 2. Verifica que el resultado devuelto por el `loop` es 20.
        assert_eq!(result, 20);
    }

    #[test]
    fn test_while_loop() {
        // Objetivo: Usar un bucle `while` para ejecutar código mientras una condición sea verdadera.
        let mut numero = 3;

        // 1. Usa un bucle `while` para decrementar `numero` hasta que llegue a 0.
        while numero > 0 {
            numero -= 1;
        }

        // 2. Verifica que, después del bucle, `numero` es 0.
        assert_eq!(numero, 0);
    }

    #[test]
    fn test_for_con_rango() {
        // Objetivo: Usar un bucle `for` para iterar sobre un rango de números.

        let mut suma = 0;

        // 1. Usa un bucle `for` para iterar sobre los números del 1 al 5 (incluido).
        //    Suma cada número a la variable `suma`.
        //    El rango `1..=5` incluye el 5.
        for i in 1..=5 {
            suma += i;
        }

        // 2. Verifica que la suma sea la correcta (1 + 2 + 3 + 4 + 5 = 15).
        assert_eq!(suma, 15);

        // 3. Usa un bucle `for` con `.rev()` para contar hacia atrás.
        for i in (1..=5).rev() {
            print!("Number {}", i);
        }
    }

    #[test]
    fn test_for_con_iterador_de_coleccion() {
        // Objetivo: Usar un bucle `for` para iterar sobre los elementos de una colección (array).
        let a: [u8; 5] = [1, 2, 3, 4, 5];
        let mut sum: u8 = 0;

        // 1. Usa un bucle `for` para iterar sobre cada elemento del array `a`.
        //    El método `.iter()` crea un iterador que nos permite recorrer la colección.
        for i in a.iter() {
            sum += i;
        }

        // 2. Verifica que la suma de los elementos sea correcta.
        assert_eq!(sum, 15);
    }
}
