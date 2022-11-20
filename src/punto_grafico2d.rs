use std::io;

#[derive(Debug)]
pub struct PuntoGrafico2D {
    cord_x: f32,
    cord_y: f32,
    tipo_nombre: String,
    color: i32,
}

impl PuntoGrafico2D{

    // Constructor parametizado
    pub fn new(cord_x: f32, cord_y: f32, tipo_nombre: String, color: i32) -> PuntoGrafico2D {
        PuntoGrafico2D{
            cord_x,
            cord_y,
            tipo_nombre,
            color,
        }
    }
    // Constructor con valores default
    pub fn default() -> PuntoGrafico2D {
        PuntoGrafico2D{
            cord_x: 0.0,
            cord_y: 0.0,
            tipo_nombre: String::from(""),
            color: 0,
        }
    }
    // Constructor de copia
    pub fn copy(&self) -> PuntoGrafico2D {
        PuntoGrafico2D {
            tipo_nombre: String::from(""),
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

    pub fn get_tipo_nombre<'a>(&'a self) -> &'a str {
        return &self.tipo_nombre;
    }

    pub fn get_color(&self) -> i32 {
        return self.color;
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

    pub fn set_tipo_nombre(&mut self) {

        let mut tipo_nombre = String::new();
        io::stdin()
            .read_line(&mut tipo_nombre)
            .expect("failed to read from stdin");

        self.tipo_nombre = tipo_nombre;

    }

    pub fn set_color(&mut self) {

        let mut color = String::new();
        io::stdin()
            .read_line(&mut color)
            .expect("failed to read from stdin");

        let color : i32 = color.trim()
            .parse()
            .expect("Por favor introduzca un valor decimal."); 

        self.color = color;
    }
    

    // Leer un puntoGrafico2D
    pub fn leer_puntoGrafico2d(&mut self) {

        self.set_cordx();
        self.set_cordy();
        self.set_tipo_nombre();
        self.set_color();
        println!("Nuevo punto creado: X: {}, Y: {}, Tipo: {}, Color: {}", self.cord_x, self.cord_y, self.tipo_nombre, self.color);
    }

    // Escribir por pantalla el puntoGrafico2D
    pub fn escribir_punto_grafico2d(&self) {
        println!("{} = ({}, {}), {}", self.tipo_nombre, self.cord_x, self.cord_y, self.color);
    }

    // Set punto grafico completo
    pub fn set_punto_grafico2d(&mut self, tipo_nombre: String, cord_x: f32, cord_y: f32, color: i32){

        self.tipo_nombre = tipo_nombre;
        self.cord_x = cord_x;
        self.cord_y = cord_y;
        self.color = color;
    }
}