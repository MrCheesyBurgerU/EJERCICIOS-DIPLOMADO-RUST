#[cfg(test)]
mod propiedad_tests {

    // --- Ownership: La Posesión de la Herramienta ---
    // Imagina que las variables poseen las herramientas. Cuando una herramienta
    // se mueve, la posesión se transfiere.

    #[test]
    fn test_transferir_propiedad() {
        // Objetivo: Entender cómo se transfiere la propiedad de los datos.

        // 1. Un nuevo martillo es creado y lo "posee" 'worker_a'.
        let hammer = String::from("Martillo de carpintero");

        // 2. Transfiere la propiedad del 'hammer' a 'worker_b'.
        //    Hint: Asigna 'hammer' a una nueva variable. ¿Qué le pasa a 'hammer' después de esto?
        let worker_b_hammer = hammer; 

        // 3. Intenta usar 'hammer' (que ahora debería estar "movido" o "tomado" por worker_b).
        //    Este código DEBERÍA causar un error de compilación.
        //    Coméntalo una vez que hayas verificado que el compilador te lo impide.
        //    println!("Worker A está usando: {}", hammer); // <--- Comentar esta línea para que el test pase.

        // 4. Asegúrate de que 'worker_b' pueda usar la herramienta.
        assert_eq!(worker_b_hammer, "Martillo de carpintero");
        println!("Worker B está usando: {}", worker_b_hammer);
    }

    #[test]
    fn test_transferir_propiedad_en_function() {
        // Objetivo: Ver cómo la propiedad se transfiere a la funcion y se devuelve.

        fn mi_funcion(tool: String) {
            println!("La función 'take_and_return_tool' recibió: {}", tool);
            // 1. La función toma la propiedad de la 'tool'.
            // 2. Retorna la propiedad de la 'tool'.
            
        }

        let mut screwdriver = String::from("Destornillador Phillips");

        // 1. Pasa 'screwdriver' a la función.
        //    ¿Qué le sucede a 'screwdriver' después de esta llamada si no se reasigna?
        mi_funcion(screwdriver); 

        // 2. Asegúrate de que 'screwdriver' todavía pueda ser usado por el dueño original.
        // descomenta el las dos lienas siguientes de codigo para validar el punto 2.
        //assert_eq!(screwdriver, "Destornillador Phillips");
        //println!("El dueño original recuperó y está usando: {}", screwdriver);
    }

    #[test]
    fn test_prestamo_inmutable() {
        // Objetivo: Practicar el préstamo inmutable (referencias compartidas).

        let drill = String::from("Taladro inalámbrico");

        // 1. Un técnico (worker_a) quiere VER el taladro para leer el modelo.
        //    Crea una referencia inmutable a 'drill'.
        let tecnico = &drill; // <-- completa aqui

        // 2. Otro técnico (worker_b) también quiere VER el taladro para tomar medidas.
        //    Crea otra referencia inmutable a 'drill'. ¿Es esto permitido?
        let otro_tecnico = &drill; // <-- No modifiques aqui nada

        // 3. Asegúrate de que ambos técnicos puedan "ver" la herramienta.
        assert_eq!(*tecnico, "Taladro inalámbrico");
        assert_eq!(*otro_tecnico, "Taladro inalámbrico");
        println!("Técnico A vio: {}", tecnico);
        println!("Técnico B vio: {}", otro_tecnico);

        // 4. Intenta modificar el 'drill' a través de una de las referencias (esto DEBERÍA fallar).
        //    Comenta la siguiente línea para que el test pase.
        //    technician_ref.push_str(" (con batería)"); 

        // 5. El dueño original aún puede usar el 'drill'.
        assert_eq!(drill, "Taladro inalámbrico");
        println!("El dueño original aún tiene su: {}", drill);
    }

    #[test]
    fn test_prestamo_mutable() {
        // Objetivo: Practicar el préstamo mutable (referencias exclusivas).

        let mut wrench = String::from("Llave inglesa ajustable");

        // 1. Un mecánico necesita Usar la llave. Presta la llave de forma mutable.
        let mechanic_mut_ref = &wrench; // <-- ompleta aqui

        // 2. El mecánico ajusta la llave.
        //mechanic_mut_ref.push_str(" (ajustada)"); // <-- remueve el comentario

        // 3. Intenta crear otra referencia mutable mientras la primera está activa (esto DEBERÍA fallar).
        //    Comenta la siguiente línea. ¿Por qué Rust prohíbe esto?
            let another_mechanic_mut_ref = &mut wrench; // <--- Comentar esta linea.

        // 4. Intenta crear una referencia INMUTABLE mientras la mutable está activa (esto DEBERÍA fallar).
        //    Comenta la siguiente línea.
            let inspector_ref = &wrench; // <--- Comentar esta línea.

        // 5. Una vez que el préstamo mutable termina (al salir del scope de `mechanic_mut_ref`),
        //    el dueño original puede acceder y verificar el cambio.
        //    Nota: El préstamo termina cuando `mechanic_mut_ref` deja de ser usado,
        //    en este caso, después de la línea 2.
        //assert_eq!(wrench, "Llave inglesa ajustable (ajustada)"); // <-- remueve el comentario de esla linea
        println!("La llave fue ajustada: {}", wrench);
    }
    #[test]
    fn test_lifetime_alcance() {
        // Objetivo: Comprender cómo el tiempo de vida de una referencia no puede exceder
        //           el tiempo de vida de los datos a los que apunta.

        let mut tool_box = String::from("Caja de herramientas grande"); // 'tool_box' vive mucho tiempo.

        { // Este es un scope más pequeño, como un proyecto temporal en el taller.
            let small_tool = String::from("Pinzas pequeñas"); // 'small_tool' solo vive dentro de este bloque.

            // 1. Intenta crear una referencia a 'small_tool'.
            let small_tool_ref = &small_tool; // <-- completa aqui

            // 2. Asegúrate de que puedas usar la referencia DENTRO de este scope.
            assert_eq!(*small_tool_ref, "Pinzas pequeñas");
            println!("Dentro del bloque, referencia a: {}", small_tool_ref);

            // Intenta hacer que 'small_tool_ref' viva más allá de este bloque.
            // Esto DEBERÍA causar un error de compilación.
            // Comenta la siguiente línea.
            // tool_box = small_tool_ref.to_string(); // <--- remueve el comentario
        } // 'small_tool' y 'small_tool_ref' se eliminan aquí.

        // 3. Intenta usar 'small_tool' o 'small_tool_ref' fuera de su scope (esto DEBERÍA fallar).
        //    Comenta las siguientes líneas.
        //    println!("Fuera del bloque, intentando usar: {}", small_tool); // <--- comentar.
        //    println!("Fuera del bloque, intentando usar referencia: {}", small_tool_ref); // <--- comentar.

        // 4. Asegúrate de que 'tool_box' todavía es válido.
        assert_eq!(tool_box, "Caja de herramientas grande");
        println!("Caja de herramientas original: {}", tool_box);
    }

    // Este test NO necesita ser resuelto por el estudiante, es para demostración.
    // Simplemente muestra un escenario común de error de lifetime y cómo Rust lo previene.
    #[test]
    fn test_lifetime_revenrir_referencias_colgantes() {
        // Objetivo: Demostrar cómo Rust previene referencias "colgantes".
        // Este test no tiene "solución" en el sentido de completar código,
        // sino en entender el porqué del error de compilación.

        // Imagine que tenemos una función que "presta" una herramienta recién creada.
        /*
        fn crear_nueva_herramienta_prestada() -> &String {
            let new_tool = String::from("Llave de impacto"); // 'new_tool' es creada aquí.
            &new_tool // Intentamos devolver una referencia a 'new_tool'.
            // ERROR: 'new_tool' se eliminará al final de esta función,
            //        lo que haría que la referencia devuelta fuera "colgante" (dangling).
            //        Rust NO permite esto.
        }
        */

        // Por lo tanto, no podemos llamar a esa función aquí, ya que no compilaría.
        // El punto es entender que Rust garantiza que las referencias siempre apunten a datos válidos.
        println!("Rust evita referencias colgantes, asegurando la validez de los préstamos.");
        assert!(true); // Este assert es solo para que el test pase.
    }

}