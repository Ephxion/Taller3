# Taller N° 3 - Rust
Taller de rust realizado con structs y traits para simular herencia a traves de composicion, y el uso de los traits para simular herencia de metodos entre clases.

Punto2D
```
Structs el cual cuenta con 2 fields de tipo f32, los cuales corresponden a la posicion en X e Y,
Posee un constructor parametrizado, de copia y uno con valores default,
Nos permite calcular la distancia entre 2 Puntos2D
```
Recta2D
```
Structs el cual posee 3 fields de tipo f32, los cuales representan los coeficientes a, b y c de la Recta2D.
Nos permite calcular la distancia de un Punto2D a la Recta2D y comparar 2 Recta2D para ver si son perpendiculares o paralelas.
```
PuntoGrafico2D
```
Punto2D especializado, nada nuevo excepto 2 nuevos fields, uno para el nombre del PuntoGrafico2D y uno de tipo i32 para el color del PuntoGrafico2D.
```
ContornoGrafico2D
```
Struct con 3 fields, nombre_contorno de tipo String, num_puntos de tipo i32 y vector_puntos de tipo Vec<Punto2D>.

```
Installing the requirements

to install the requirements you just need to run:

```
python -m pip install -r requirements.txt
```

Now you need to create a .env file, to store your bot token and guild id:

```
TOKEN=your_bot_token
GUILD_ID=your_guild_id
```
And that's all, now you can run the bot and start testing with it.
Hope this template helped you!
