use crate::cuenta::TipoCuenta;
struct Cliente {
    nombre: String,
    saldo: f64,
    cuenta: TipoCuenta,
}

impl Cliente {
    fn nueva(nombre: &str, cuenta: TipoCuenta) -> Self {
        Cliente {
            nombre: nombre.to_string(),
            saldo: 0.0,
            cuenta,
        }
    }

    fn depositar(&mut self, monto: f64) {
        self.saldo += monto;
        println!("{} depositó {}", self.nombre,  monto);
    }

    fn retirar(&mut self, monto: f64) {
        let permitido = match self.cuenta {
            TipoCuenta::Ahorros => self.saldo >= monto,
            TipoCuenta::Corriente => self.saldo - monto >= -500000.0,
        };

        if permitido {
            self.saldo -= monto;
            println!("{} retiró {}",  self.nombre, monto);
        } else {
            println!("no tiene fondos suficientes");
        }
    }

    fn mostrar(&self) {
        println!("Nombre: {}", self.nombre );
        println!("Saldo: {}", self.saldo);
        eprintln!("Cuenta: {:?}", self.cuenta);
    }
}