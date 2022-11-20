use std::io;
use crate::punto_2d::Punto2D;

#[derive(Debug)]
pub struct Recta2D {
    a: f32,
    b: f32,
    c: f32,
}

impl Recta2D{

    // Constructor parametizado
    pub fn new(a: f32, b: f32, c: f32) -> Recta2D {
        Recta2D{
            a,
            b,
            c,
        }
    }
    // Constructor con valores default
    pub fn default() -> Recta2D {
        Recta2D{
            a: 0.0,
            b: 0.0,
            c: 0.0,
        }
    }
    // Constructor de copia
    pub fn copy(&self) -> Recta2D {
        Recta2D {
            ..*self
        }
    }

    // Constructor con 2 Puntos2D
    pub fn new_points(&self, punto1: Punto2D, punto2: Punto2D) -> Recta2D {
        Recta2D {
            a: punto2.get_cordx() - punto1.get_cordx(),
            b: punto2.get_cordy() - punto2.get_cordy(),
            c: punto2.get_cordx() - punto1.get_cordx() * punto1.get_cordx() * punto2.get_cordy() - punto2.get_cordy() * punto1.get_cordy(),
        }
    }

    // Getters
    pub fn get_a(&self) -> f32 {
        return self.a;
    }

    pub fn get_b(&self) -> f32 {
        return self.b;
    }

    pub fn get_c(&self) -> f32 {
        return self.c;
    }

    // Setters
    pub fn set_a(&mut self) {

        let mut a = String::new();
        io::stdin()
            .read_line(&mut a)
            .expect("failed to read from stdin");

        let a : f32 = a.trim()
            .parse()
            .expect("Por favor introduzca un valor decimal."); 
        self.a = a;
    }

    pub fn set_b(&mut self) {

        let mut b = String::new();
        io::stdin()
            .read_line(&mut b)
            .expect("failed to read from stdin");

        let b : f32 = b.trim()
            .parse()
            .expect("Por favor introduzca un valor decimal."); 

        self.b = b;
    }

    pub fn set_c(&mut self) {

        let mut c = String::new();
        io::stdin()
            .read_line(&mut c)
            .expect("failed to read from stdin");

        let c : f32 = c.trim()
            .parse()
            .expect("Por favor introduzca un valor decimal."); 

        self.c = c;
    }

    // Leer un Recta2D
    pub fn leer_recta2d(&mut self) {

        self.set_a();
        self.set_b();
        self.set_c();
        println!("Nuevo recta creada: A: {}, B: {}, C: {}", self.a, self.b, self.c);
    }

    pub fn EscribirRecta2D(&self) {
        if self.get_a() != 0.0 {
            if self.get_a() == 1.0 {
                print!("X ");
            }
            else {
                print!("{} X ",self.get_a());
            }
        }
        if self.get_b() != 0.0 {
            if self.get_b() > 0.0 {
                if self.get_b() == 1.0 {
                    print!(" + Y ");
                }
                else {
                    print!("+ {} Y ",self.get_b());
                }
            }
            else {
                if self.get_b() ==  -1.0 {
                    print!(" - Y ");
                }
                else {
                    print!("- {} Y ",self.get_b()*-1.0);
                }
            }
        }
        if self.get_c() != 0.0 {
            if self.get_c() > 0.0 {
                println!("+ {} = 0",self.get_c());
            }
            else {
                println!("- {} = 0",self.get_c()*-1.0);
            }
        }
    }

    // Calcular la distancia entre la Recta2D y un Punto2D
    pub fn calcular_distancia_punto2d_recta2d(&self, punto: Punto2D) -> f32 {
        return (self.a * punto.get_cordx() + self.b * punto.get_cordx() + self.c).abs() / (self.a * self.a + self.b * self.b).sqrt();
    }
    
    pub fn son_rectas2d_perpendiculares(&self, recta: Recta2D) -> bool {
        if self.a * recta.get_a() + self.b * recta.get_b() != 0.0 {return false;}
        return true;
    }

    pub fn son_rectas2d_paralelas(&self, recta: Recta2D) -> bool {
        if self.a * recta.get_b() - self.b * recta.get_a() != 0.0 {return false;}
        return true;
    }

}