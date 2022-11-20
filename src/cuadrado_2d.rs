use crate::Poligono2D;
use crate::Punto2D;
use crate::PerimetroArea;

#[derive(Debug)]

// Usamos composicion para simular herencia en este caso Cuadrado2D tiene un field de tipo Poligono2D ya que no tiene atributos adicionales a la clase padre.
pub struct Cuadrado2D{
    poligono: Poligono2D
}

impl Cuadrado2D{

    // Constructor parametizado
    pub fn new() -> Cuadrado2D {
        Cuadrado2D{
            poligono: Self::crear_poligono(),
        }
    }

    // Getters
    pub fn get_poligono(&self) -> Poligono2D {
        return self.poligono.to_owned()
    }
    
    // Las distancias entre cada vertice deben ser iguales o el poligono NO corresponde a un cuadrado.
    pub fn es_cuadrado(&self) -> bool {

        let a: f32 = self.poligono.get_vector_vertices()[0].calcular_distancia(self.poligono.get_vector_vertices()[1]);
        let b: f32 = self.poligono.get_vector_vertices()[1].calcular_distancia(self.poligono.get_vector_vertices()[2]);
        let c: f32 = self.poligono.get_vector_vertices()[2].calcular_distancia(self.poligono.get_vector_vertices()[3]);
        let d: f32 = self.poligono.get_vector_vertices()[0].calcular_distancia(self.poligono.get_vector_vertices()[3]);

        if a != b || a != c || a != d {return false;} else {return true;}
    }

}

// Usamos el trait PerimetroArea para simular herencia para los metodos de las clases que derivan de Poligono2D,
// en este caso Cuadrado2D. (Tristemente solo podemos simular herencia para metodos pero no para atributos)

impl PerimetroArea for Cuadrado2D {

    fn calcular_perimetro(&self) -> f32 {

        let a: f32 = self.poligono.get_vector_vertices()[0].calcular_distancia(self.poligono.get_vector_vertices()[1]);
        let b: f32 = self.poligono.get_vector_vertices()[1].calcular_distancia(self.poligono.get_vector_vertices()[2]);
        let c: f32 = self.poligono.get_vector_vertices()[2].calcular_distancia(self.poligono.get_vector_vertices()[3]);
        let d: f32 = self.poligono.get_vector_vertices()[0].calcular_distancia(self.poligono.get_vector_vertices()[3]);

        return a + b + c + d;
    }

    fn calcular_area(&self) -> f32 {

        if self.es_cuadrado() {

            let a: f32 = self.poligono.get_vector_vertices()[0].calcular_distancia(self.poligono.get_vector_vertices()[1]);
            let b: f32 = self.poligono.get_vector_vertices()[1].calcular_distancia(self.poligono.get_vector_vertices()[2]);
            return a*b;
            
        } else { return 0.0; }
    }

    fn crear_poligono() -> Poligono2D {

        let mut vectorPuntos: Vec<Punto2D> = Vec::new();
        let x: Punto2D = Punto2D::new(0.0, 1.0);
        let y: Punto2D = Punto2D::new(3.0, 5.0);
        let z: Punto2D = Punto2D::new(7.0, 2.0);
        let j: Punto2D = Punto2D::new(4.0, -2.0);
        
        vectorPuntos.push(x);
        vectorPuntos.push(y);
        vectorPuntos.push(z);
        vectorPuntos.push(j);

        return Poligono2D::new(4, &vectorPuntos)

    }
}
