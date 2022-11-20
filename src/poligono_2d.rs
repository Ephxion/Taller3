use crate::Punto2D;

#[derive(Clone, Debug)]
pub struct Poligono2D{
    vector_vertices: Vec<Punto2D>,
    num_vertices: i32,
}

impl Poligono2D {

    // Constructor parametrizado
    pub fn new(num_vertices: i32, vector_vertices: &Vec<Punto2D>) -> Poligono2D {
        Poligono2D{
            num_vertices,
            vector_vertices: vector_vertices.to_vec(),
        }
    }

    // Getters y Setters
    pub fn get_num_vertices(&self) -> i32 {
        return self.num_vertices;
    }

    pub fn get_vector_vertices(&self) -> Vec<Punto2D> {
        return self.vector_vertices.clone();
    }

    pub fn get_vertice(&self, indice: usize) -> Punto2D {
        return self.vector_vertices[indice];
    }

    pub fn set_vertice(&mut self, indice: usize, punto: Punto2D) {
        self.vector_vertices[indice] = punto;
    }
}