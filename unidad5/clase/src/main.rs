
mod calculadora;
mod utilidades;

fn main() {
    println!("sumar 6 + 5: {} ", calculadora::operacion::aritmetica::suma(6, 5));
    
    let suma = utilidades::internet::servicio_suma::sumar_from_google(6, 5);
    println!("suma {}", suma);
}
