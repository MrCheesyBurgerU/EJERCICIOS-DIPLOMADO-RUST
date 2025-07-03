use std::error;

fn main() {
        let edad_a: i32 = 28;
        let edad_b: u16 = 30;
        
        if edad_a < (edad_b as i32) {
                println!("edadA es menor que edadB");
        }
        
        ////////////////////////////////////////////
        
        let value: i32 = 1500;
        let result: i8 = value as i8;
        print!("result: {}", result);
        
        ///////////////////////////////////////////
        
        let value: i32 = 1500;
        let result: i16 = value.try_into().unwrap();
        println!("result: {}", result);
        
        ////////////////////////////////////////////
        
        let mut cont: i32 = 0;
        cont = cont + 2;

        {
                let mut cont:i32 = cont * 5;
                cont = cont + 1;
                println!("valor del alcance interno {}", cont);
        }
        
        println!("valor orginal de cont: {}", cont);
        
        
        ///////////////////////////////////////////////////
        
        let tupla: (i32, f64, u8) = (1501, 7.2, 5);
        
        let tupla_2 = (1500, 7.1, 1);
        
        let (x, y, z) = tupla_2;
        
        println!("x: {}, y: {}, z: {}", x, y, z);
        
        println!("tupla: {:?}", tupla);
        
        println!("x: {}", tupla.0);
        println!("y: {}", tupla.1);
        println!("z: {}", tupla.2);

        /////////////////////////////////////
         
        let x = 5;
        let y = 6;
        
        let resultado = sumar(x, y);
        
        println!("resultado: {}", resultado);
        
        
        /////////////////////////////////////////
        
        //let es_verdadero = false;
        //let resultado = if es_verdadero { 10 } else { "dos" };
        
        /////////////////////////////////////////
        
        let x = 4;
        
        match x {
                0..=5 => println!("0 a 5"),
                6..10 => println!("6 a 9"),
                _ => println!("otro valor"),
        }
        
        /////////////////////////////////////
        
        let letra = 'n';
        
        match letra {
                'a' =>  println!("vocal"),
                'e' =>  println!("vocal"),
                'i' =>  println!("vocal"),
                'o' =>  println!("vocal"),
                'u' =>  println!("vocal"),
                _ => println!("consonante"),
        }
        
        ////////////////////////////////////
        
        let coordenadas = (3.15, 9.1);
        
        // ¿donde estoy?
        match coordenadas {
                (0.0, 0.0) => println!("origen"),
                (x, 0.0) => if x > 0.0 {
                                        println!("Hemisferio Norte x: {}", x)
                                 } else {
                                        println!("Hemisferio Sur x: {}", x)
                                 },
                (0.0, y) => if y > 0.0 {
                                        println!("Este: y {}", y)
                                } else {
                                        println!("Oeste: y {}", y)
                                },
                (x, y) => if x > 0.0 && y > 0.0 {
                                        println!("Norte y Este: x {}, y {}", x, y)
                                   } else if x > 0.0 && y < 0.0 {
                                        println!("Norte y Oeste: x {}, y {}", x, y)
                                   } else if x < 0.0 && y > 0.0 {
                                        println!("Sur y Este: x {}, y {}", x, y)
                                        }
        
        }
                
        
       ////////////////////////////////////////////////////////
        
        for item in 0..10 {
                if item == 5 {
                        println!("salta {}", item);
                        continue;
                }
                
                println!("item: {}", item);
        }
        
        ///////////////////////////////////////////////
        
        let mut number = 0;
        
         let valor_final = loop {
                //println!("{}", number);
                if number == 10 {
                        break "hola loop"
                }
                number += 1;                
        };
        println!("valor final: {}", valor_final);
}

fn sumar(parametro1: i32, parametro2: i32) -> i32 { 
        parametro1 + parametro2
}