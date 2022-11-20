# Taller N° 3 - Rust
Taller de rust realizado con structs y traits para simular herencia a traves de composicion, y el uso de los traits para simular herencia de metodos entre clases.

#
Punto2D
```
Structs el cual cuenta con 2 fields de tipo f32, los cuales corresponden a la posicion en X e Y,
Posee un constructor parametrizado, de copia y uno con valores default,
Nos permite calcular la distancia entre 2 Puntos2D
```
#
Recta2D
```
Structs el cual posee 3 fields de tipo f32, los cuales representan los coeficientes a, b y c de la Recta2D.
Nos permite calcular la distancia de un Punto2D a la Recta2D y comparar 2 Recta2D para ver si son perpendiculares o paralelas.
```
#
PuntoGrafico2D
```
Punto2D especializado, nada nuevo excepto 2 nuevos fields, uno para el nombre del PuntoGrafico2D y uno de tipo i32 para el color del PuntoGrafico2D.
```
#
ContornoGrafico2D
```
Struct con 3 fields, nombre_contorno de tipo String, num_puntos de tipo i32 y vector_puntos de tipo Vec<Punto2D>.
nos permite leer un ContornoGrafico2D desde un archivo o grabarlo en uno.
#
```
Poligono2D
```
Struct base para las clases Triangulo2D y Cuadrado2D, posee 2 fields, un num_vertices de tipo i32 y un vector_vertices de tipo Vec<Punto2D>.
Esta clase la utilizo para simular herencia a traves de composicion.
```
#
Triangulo2D
```
Struct que posee 1 solo field el cual es un poligono de tipo Poligono2D, debido a que el struct Triangulo2D no posee nuevos fields.
Nos permite verificar de que los vertices que posee el Triangulo2D formen un Triangulo2D y nos devuelve un tipo de dato bool,
tambien podemos calcular el perimetro del Triangulo2D y en caso de verificar que efectivamente los vertices formen un triangulo,
podemos calcular el area del triangulo, en caso contrario la funcion devuelve el valor 0.0 (tipo f32).
```
#
Cuadrado2D
```
Struct que posee 1 solo field el cual es un poligono de tipo Poligono2D, debido a que el struct Cuadrado2D no posee nuevos fields.
Nos permite comprobar que los vertices del Cuadrado2D formen realmente un cuadrado, esto lo hacemos calculando la distancia entre sus puntos,
debido a que debe existir la misma distancia entre los vertices para formar un cuadrado.
Tambien podemos calcular el perimetro, y en caso de que los vertices formen un cuadrado podemos calcular el area de este, en caso contrario la funcion devuelve el valor 0.0 (tipo f32).
```

# Instalacion

para instalar y correr el programa lo unico que hay que hacer es clonarlo, tener instalado rust/cargo y utilizar cargo run.

```
git clone https://github.com/Ephxion/Taller3
cd Taller3/src/
cargo run
```
