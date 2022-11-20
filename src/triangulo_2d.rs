use crate::Poligono2D;
use crate::Punto2D;
use crate::PerimetroArea;

#[derive(Debug)]

// Usamos composicion para simular herencia en este caso Triangulo2D tiene un field de tipo Poligono2D ya que no tiene atributos adicionales a la clase padre.
pub struct Triangulo2D{
    poligono: Poligono2D
}

impl Triangulo2D{

    // Constructor parametizado
    pub fn new() -> Triangulo2D {
        Triangulo2D{
            poligono: Self::crear_poligono(),
        }
    }

    // Getters
    pub fn get_poligono(&self) -> Poligono2D {
        return self.poligono.to_owned()
    }

    pub fn set_poligono(&mut self, poligono: Poligono2D) {
        self.poligono = poligono;
    }
    
    // Comprobar si es triangulo
    pub fn es_triangulo(&self) -> bool {

        let a: f32 = self.poligono.get_vector_vertices()[0].calcular_distancia(self.poligono.get_vector_vertices()[1]);
        let b: f32 = self.poligono.get_vector_vertices()[1].calcular_distancia(self.poligono.get_vector_vertices()[2]);
        let c: f32 = self.poligono.get_vector_vertices()[0].calcular_distancia(self.poligono.get_vector_vertices()[2]);

        if (b - c).abs() < a && a < (b + c).abs() {return true;}
        return false;
    }

}

// Usamos el trait PerimetroArea para simular herencia para los metodos de las clases que derivan de Poligono2D,
// en este caso Triangulo2D. (Tristemente solo podemos simular herencia para metodos pero no para atributos)
impl PerimetroArea for Triangulo2D {

    fn calcular_perimetro(&self) -> f32 {

        let a: f32 = self.poligono.get_vector_vertices()[0].calcular_distancia(self.poligono.get_vector_vertices()[1]);
        let b: f32 = self.poligono.get_vector_vertices()[1].calcular_distancia(self.poligono.get_vector_vertices()[2]);
        let c: f32 = self.poligono.get_vector_vertices()[0].calcular_distancia(self.poligono.get_vector_vertices()[2]);

        return a + b + c;
    }

    fn calcular_area(&self) -> f32 {

        if self.es_triangulo(){

            let a: f32 = self.poligono.get_vector_vertices()[0].calcular_distancia(self.poligono.get_vector_vertices()[1]);
            let b: f32 = self.poligono.get_vector_vertices()[1].calcular_distancia(self.poligono.get_vector_vertices()[2]);
            let c: f32 = self.poligono.get_vector_vertices()[0].calcular_distancia(self.poligono.get_vector_vertices()[2]);
            let s: f32 = (a + b + c)/2.0;

            return (s * (s - a) * (s - b) * (s - c)).sqrt()

        } else { return 0.0; }
    }

    fn crear_poligono() -> Poligono2D {

        let mut vectorPuntos: Vec<Punto2D> = Vec::new();
        let x: Punto2D = Punto2D::new(12.0, 21.0);
        let y: Punto2D = Punto2D::new(4.0, 14.2);
        let z: Punto2D = Punto2D::new(5.0, 9.2);
        
        vectorPuntos.push(x);
        vectorPuntos.push(y);
        vectorPuntos.push(z);

        return Poligono2D::new(3, &vectorPuntos)

    }
}
