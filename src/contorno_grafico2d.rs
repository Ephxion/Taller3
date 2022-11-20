use std::str::FromStr;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use crate::Punto2D;
<<<<<<< HEAD
use crate::PuntoGrafico2D;
=======
>>>>>>> f6cf96c46e83a6e14e693f4ab96e4d98b89af317

#[derive(Debug)]
pub struct ContornoGrafico2D {
    nombre_contorno: String,
    num_puntos: i32,
    vector_puntos: Vec<Punto2D>,
}

impl ContornoGrafico2D{

    // Constructor parametizado
    pub fn new(nombre_contorno: String, num_puntos: i32, vector_puntos: &Vec<Punto2D>) -> ContornoGrafico2D {
        ContornoGrafico2D{
            nombre_contorno,
            num_puntos,
            vector_puntos: vector_puntos.to_vec(),
        }
    }
    // Constructor con valores default
    pub fn default() -> ContornoGrafico2D {
        ContornoGrafico2D{
            nombre_contorno: String::from(""),
            num_puntos: 0,
            vector_puntos: Vec::new()
        }
    }
    // Constructor de copia
    pub fn copy(&self) -> ContornoGrafico2D {
        ContornoGrafico2D {
            nombre_contorno: self.nombre_contorno.clone(),
            vector_puntos: self.vector_puntos.clone(),
            ..*self
        }
    }

    // Constructor con un parametro

    pub fn new_points(vector_puntos: &Vec<Punto2D>) -> ContornoGrafico2D {
        ContornoGrafico2D {
            vector_puntos: vector_puntos.to_vec(),
            nombre_contorno: String::from(""),
            num_puntos: vector_puntos.len() as i32,
        }
    }

    // Getters
    pub fn get_nombre_contorno<'a>(&'a self) -> &'a str {
        return &self.nombre_contorno;
    }

    pub fn get_num_puntos(&self) -> i32 {
        return self.num_puntos;
    }

    pub fn get_vector_puntos(&self) -> Vec<Punto2D> {
        return self.vector_puntos.clone();
    }

    // Setters
    pub fn set_nombre_contorno(&mut self, nombre: String) {
        self.nombre_contorno = nombre
    }

    pub fn set_num_puntos(&mut self, num_puntos: i32) {
        self.num_puntos = num_puntos
    }

    pub fn set_vector_puntos(&mut self, vector_puntos: &Vec<Punto2D>) {
        self.vector_puntos = vector_puntos.clone();
    }

    pub fn escribir_contorno_grafico(&self) {
    
        println!("Contorno Gráfico {}",self.nombre_contorno);
        println!("Número de puntos = {}",self.num_puntos);
        println!("Puntos:");
        let vp:Vec<Punto2D> = self.get_vector_puntos();
        for i in vp {
            i.escribir_punto();
        }
        
    }

    pub fn leer_contorno_archivo(&mut self, nombre_archivo: &str) -> std::io::Result<()> {
        let archivo = match File::open(nombre_archivo)
	    {
		    Ok(archivo) => archivo,
		    Err(..) => panic!("Error al abrir el archivo"),
	    };

	    let lector = BufReader::new(archivo);
        let mut i:i32 = 1;
        let mut linea_str: String;
        for linea  in lector.lines() {
            if i == 1 {
                linea_str = linea.unwrap();
                self.set_nombre_contorno(linea_str);
                i = 2;
            }
            else if i == 2 {
                linea_str = linea.unwrap();
                let nP: i32 = linea_str.trim().parse().expect("Error en número"); 
                self.set_num_puntos(nP);
                i = 3;
            }
            else {
                linea_str = linea.unwrap();
                let coordenadas: Vec<&str> = linea_str.split(" ").collect();
                let x: f32 = coordenadas[0].trim().parse().expect("Error en número");
                let y: f32 = coordenadas[1].trim().parse().expect("Error en número");
                let p:Punto2D = Punto2D::new(x, y);
                let mut vp:Vec<Punto2D> = self.get_vector_puntos();
                vp.push(p);
                self.set_vector_puntos(&vp);
            }
        }
        if !self.validar() {
            println!("Error puntos no constituyen un contorno");
            println!("Contorno quedará con valores por defecto");
            self.set_nombre_contorno(String::from(""));
            self.num_puntos = 0;
            let vp:Vec<Punto2D> = Vec::new();
            self.set_vector_puntos(&vp);

        }
	    Ok(()) 
    }

    fn validar(&self) -> bool {
        let vp:Vec<Punto2D> = self.vector_puntos.clone();
        for i in 0..vp.len()-1 {
            if vp[i].calcular_distancia(vp[i+1]) > 2.0_f32.sqrt() {
                return false;
            }
        }
        if vp[vp.len()-1].calcular_distancia(vp[0]) > 2.0_f32.sqrt() {
            return false;
        }
        for i in 0..vp.len() {
            for j in i+1..vp.len() {
                if vp[i].get_cordx() == vp[j].get_cordx() && vp[i].get_cordy() == vp[j].get_cordy() {
                    return false;
                }
            }
        }
        true
    }
<<<<<<< HEAD

    // Calculamos la centroide del poligono y devolvemos un PuntoGrafico2D con los valores de posicion del centroide obtenido.
    pub fn calcular_centroide(&self) -> PuntoGrafico2D{

        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        for i in &self.vector_puntos{
            x += i.get_cordx();
            y += i.get_cordy();
            z+=1.0;
        }
        return PuntoGrafico2D::new(x/z, y/z, String::from("Centroide"), 10)
    }
=======
>>>>>>> f6cf96c46e83a6e14e693f4ab96e4d98b89af317
}
