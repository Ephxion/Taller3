use std::io;

#[derive(Clone, Debug, Copy)]
pub struct Punto2D {
    cord_x: f32,
    cord_y: f32,
}

impl Punto2D{

    // Constructor parametizado
    pub fn new(cord_x: f32, cord_y: f32) -> Punto2D {
        Punto2D{
            cord_x,
            cord_y,
        }
    }
    // Constructor con valores default
    pub fn default() -> Punto2D {
        Punto2D{
            cord_x: 0.0,
            cord_y: 0.0,
        }
    }
    // Constructor de copia
    pub fn copy(&self) -> Punto2D {
        Punto2D {
            ..*self
        }
    }

    // Getters
    pub fn get_cordx(&self) -> f32 {
        return self.cord_x;
    }

    pub fn get_cordy(&self) -> f32 {
        return self.cord_y;
    }

    // Setters
    pub fn set_cordx(&mut self) {

        let mut cord_x = String::new();
        io::stdin()
            .read_line(&mut cord_x)
            .expect("failed to read from stdin");

        let cord_x : f32 = cord_x.trim()
            .parse()
            .expect("Por favor introduzca un valor decimal."); 

        self.cord_x = cord_x;
    }

    pub fn set_cordy(&mut self) {

        let mut cord_y = String::new();
        io::stdin()
            .read_line(&mut cord_y)
            .expect("failed to read from stdin");

        let cord_y : f32 = cord_y.trim()
            .parse()
            .expect("Por favor introduzca un valor decimal."); 

        self.cord_y = cord_y;
    }

    // Leer un punto2D
    pub fn leer_punto2d(&mut self) {
        self.set_cordx();
        self.set_cordy();
        println!("Nuevo punto creado: X: {}, Y: {}", self.cord_x, self.cord_y);
    }

    // Escribir por pantalla el punto2D
    pub fn escribir_punto(&self) {
        println!("({}, {})", self.cord_x, self.cord_y);
    }

    // Calcular la distancia entre 2 puntos2D 
    pub fn calcular_distancia(&self, punto: Punto2D) -> f32 {
        return ((punto.cord_x - self.cord_x).powf(2.0) + (punto.cord_y - self.cord_y).powf(2.0)).sqrt();
    }

}