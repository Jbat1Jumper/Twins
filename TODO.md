# Roadmap de desarrollo

## Round 2

- [x] Arreglar brillito
- [x] Hacer funcionar alpha blending para brillito
- [ ] Función continua 
- [ ] Recibir parametros desde afuera del loop
- [ ] Separar y reorganizar archivos
- [ ] Interfaz con cursive

### Función continua

En primer lugar, vamos a generar un Mesh. En este caso usaremos de ejemplo un
rectangulo teselado.

Luego, parametrizaremos algúna propiedad de sus triangulos/vertices en función
de `W`. Eso puede ser definido como una función `fn(m: Mesh, w: W) -> Mesh`.
Por ejemplo una función podría ser:

```rust
fn scale(m: Mesh, w: f32) -> Mesh {
    m.map_vertex(|v: Vertex| -> Vertex {
        Vertex { 
            position: v.position * w,
            ..v
        }
    })
}
```

En nuestro caso queremos cómo entrada `t: i32`, la cantidad de milisegundos desde
el inicio de la visualización. Nuestra transformación va a ser, en el caso de
que la compomente `x` de la posición del vertice sea menor que 0, multiplicar
esa componente por el resultado de la función `fn plot(t: i32) -> f32` en
`plot(t + z)`, donde `z` es otra componente de la pocición del vertice.

Visto desde arriba queremos lograr algo así:

```
             z ^
               |
      ,--------|--------.
     ,  .  .  .|. . . . |
     ,  .  .  .|. . . . |
      , .  .  .|. . . . |
       , .  . .|. . . . |
        , . . .|. . . . |
        , . . .|. . . . |
       , .  . .|. . . . |
      ---------|--------'
               |
               /------------------->
                                  x 
```


### Recibir parametros desde afuera del loop

Hasta ahora `W` viene siendo un tipo generico, pero necesitamos especificar que
operaciones vamos a querer hacer con una instancia de `W`. En principio para
poder enviarlo entre threads vamos a necesitar que sea `Send`. Luego, queremos
definir una interfaz para poder listar que parametros tiene, junto con su
nombre y su descripción, así como también modificar sus valores.

Luego implementar una `repl` que esté en otro thread y permita manipular esos
parametros.


### Interfaz con cursive

Yendo de nuevo a lo anterior, la interfáz podría estar hecha con cursive,
permitiría explorar la lista de parametros, editar sus valores, y se debería
actualizar cuando los parametros cambien.


## Round 1.5

- [x] Tomar input de midi
- [x] Experimentar con un tipo Clock
- [x] Generar un mesh en base al input midi


## Round 1

- [x] Definir struct para triangulos, incluyendo vertex color.
- [x] Implementar render de triangulos en VulkanBackend
- [x] Hacer algo con eso... rehacer brillito
- [x] Implementar z-buffer en el backend y borrar el sort de los rayos de brillito

