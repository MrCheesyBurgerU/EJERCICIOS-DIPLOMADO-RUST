#[cfg(test)]
mod tipos_compuestos_tests {

    // Las tuplas son una forma de agrupar varios valores con una variedad de tipos en un solo tipo compuesto.
    // Son de longitud fija: una vez declaradas, no pueden crecer ni encogerse en tamaño.

    #[test]
    fn test_crear_y_acceder_tuplas() {
        // Objetivo: Aprender a crear tuplas y acceder a sus elementos por índice.

        // 1. Crea una tupla llamada `persona` que contenga un nombre (String), una edad (u32) y una altura (f32).
        let persona: (String, u32, f32) = (String::from("Ana"), 28, 1.75);

        // 2. Accede al primer elemento (nombre) de la tupla usando la sintaxis de punto `.` y el índice.
        let _nombre = persona.0;

        let edad = persona.1;
        assert_eq!(edad, 28);

        // 4. Accede a la altura y verifica que sea 1.75.
        let altura = persona.2;
        assert!((altura - 1.75).abs() < 0.0001);
    }

    #[test]
    fn test_desestructuracion_de_tuplas() {
        // Objetivo: Aprender a desestructurar una tupla para extraer sus valores en variables separadas.

        // 1. Define una tupla `punto` que represente coordenadas (x, y, z).
        let punto: (i32, i32, i32) = (3, 7, 1);

        // 2. Desestructura la tupla `punto` en tres variables: `x`, `y`, y `z`.
        // Esta es una forma muy común y conveniente de trabajar con los valores de una tupla.
        let (x, y, z) = punto;

        // 3. Verifica que los valores se hayan asignado correctamente.
        assert_eq!(x, 3);
        assert_eq!(y, 7);
        assert_eq!(z, 1);

        // 4. A veces solo te interesan algunos valores. Usa `_` para ignorar los demás.
        // Desestructura la tupla `punto` para obtener solo el valor de `y`.
        let (_, solo_y, _) = punto;
        assert_eq!(solo_y, 7);
    }

    #[test]
    fn test_tuplas_anidadas() {
        // Objetivo: Trabajar con tuplas que contienen otras tuplas.

        // 1. Crea una tupla `rectangulo` que contenga dos puntos (tuplas `(x, y)`).
        // El primer punto es la esquina superior izquierda, el segundo es la inferior derecha.
        let punto_a = (10, 20);
        let punto_b = (40, 60);
        let rectangulo = (punto_a, punto_b);

        // 2. Accede a la coordenada `x` del primer punto (`punto_a`).
        // Se accede a la tupla externa primero, y luego a la interna.
        let x1 = rectangulo.0.0;
        assert_eq!(x1, 10);

        // 3. Desestructura la tupla `rectangulo` para obtener ambos puntos.
        let (esquina_sup_izq, esquina_inf_der) = rectangulo;
        assert_eq!(esquina_sup_izq.0, 10);
        assert_eq!(esquina_inf_der.1, 60);

        // 4. Desestructura completamente la tupla anidada para obtener las 4 coordenadas.
        let ((x1_de, y1_de), (x2_de, y2_de)) = rectangulo;
        assert_eq!(x1_de, 10);
        assert_eq!(y1_de, 20);
        assert_eq!(x2_de, 40);
        assert_eq!(y2_de, 60);
    }
}
