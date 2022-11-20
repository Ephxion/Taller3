#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_must_use)]

mod punto_2d;
mod recta_2d;
mod punto_grafico2d;
mod contorno_grafico2d;
mod triangulo_2d;
mod cuadrado_2d;
mod poligono_2d;
mod traits;

use crate::punto_2d::Punto2D;
use crate::recta_2d::Recta2D;
use crate::punto_grafico2d::PuntoGrafico2D;
use crate::contorno_grafico2d::ContornoGrafico2D;
use crate::triangulo_2d::Triangulo2D;
use crate::cuadrado_2d::Cuadrado2D;
use crate::poligono_2d::Poligono2D;
use crate::traits::PerimetroArea;

fn main() {

    let mut punto1 = Punto2D::new(12.0, 21.0);
    let mut punto2 = Punto2D::new(13.0, 34.0);
    let mut triangulo1 = Triangulo2D::new();
    let mut cuadrado1 = Cuadrado2D::new();
    let mut recta1 = Recta2D::new(2.4, 3.4, 1.2);
    let mut vector_contorno: Vec<Punto2D> = Vec::new();
    vector_contorno.push(punto1);
    vector_contorno.push(punto2);
    let mut contorno1 = ContornoGrafico2D::new(String::from("Contorno1"), 2, &vector_contorno);
    let mut punto_grafico1 = PuntoGrafico2D::new(0.0, 5.0, String::from("Punto Grafico 1"), 32);

    println!("========================================================");
    println!("Punto 1 y Punto 2:");
    punto1.escribir_punto();
    punto2.escribir_punto();
    println!("Recta entre los 2 puntos:");
    recta1.EscribirRecta2D();
    println!("========================================================");
    punto_grafico1.escribir_punto_grafico2d();
    println!("========================================================");
    contorno1.escribir_contorno_grafico();
<<<<<<< HEAD
    println!("Calcular centroide: {:?}", contorno1.calcular_centroide());
=======
>>>>>>> f6cf96c46e83a6e14e693f4ab96e4d98b89af317
    println!("========================================================");
    println!("Poligono correspondiente al triangulo: {:?}", triangulo1.get_poligono());
    println!("Es triangulo ?: {:?}", triangulo1.es_triangulo());
    println!("Perimetro del triangulo: {}, Area del triangulo: {}", triangulo1.calcular_perimetro(), triangulo1.calcular_area());
    println!("========================================================");
    println!("Poligono correspondiente al cuadrado: {:?}", cuadrado1.get_poligono());
    println!("Es cuadrado?: {:?}", cuadrado1.es_cuadrado());
    println!("Perimetro del cuadrado: {}, Area del cuadrado: {}", cuadrado1.calcular_perimetro(), cuadrado1.calcular_area());
    println!("========================================================");
    println!("Vertice posicion 0 del poligno: {:?}", triangulo1.get_poligono().get_vertice(0));
    let mut temp_poli = triangulo1.get_poligono();
    temp_poli.set_vertice(0, punto2);
    triangulo1.set_poligono(temp_poli);
    println!("Nuevo vertice posicion 0 del poligono: {:?}", triangulo1.get_poligono().get_vertice(0));
    println!("========================================================");

}

