use crate::Poligono2D;

// Implemento un trait tipo PerimetroArea para las 2 clases que derivan de Poligono2D, y ademas un metodo llamado crear_poligono,
// debido a que use composicion para simular herencia entre las clases Poligono2D y {Triangulo2D, Cuadrado2D}.

pub trait PerimetroArea {
    fn calcular_perimetro(&self) -> f32;
    fn calcular_area(&self) -> f32;
    fn crear_poligono() -> Poligono2D;
}