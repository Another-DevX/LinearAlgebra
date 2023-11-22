# Guía y Fundamentos Matemáticos del Programa de Eliminación de Gauss-Jordan

## Fundamentos Matemáticos

La eliminación de Gauss-Jordan es un método algebraico para resolver sistemas de ecuaciones lineales. Se fundamenta en la transformación de la matriz asociada al sistema a una forma escalonada reducida por medio de operaciones elementales en las filas.

### Operaciones Elementales en las Filas

1. **Intercambiar dos filas (`R_i ↔ R_j`):** Cambiar la posición de la fila `i` con la fila `j`.
2. **Multiplicar una fila por un escalar no nulo (`kR_i`):** Todos los elementos de la fila `i` se multiplican por un escalar `k`.
3. **Sumar o restar filas (`R_i ± kR_j`):** A los elementos de la fila `i` se les suma o resta los elementos correspondientes de la fila `j`, previamente multiplicados por un escalar `k`.

### Objetivo de la Eliminación de Gauss-Jordan

Convertir la matriz \( A \) del sistema \( Ax = b \) en una forma donde cada fila tenga un pivote (primer elemento no nulo) igual a 1, y que todos los elementos por encima y por debajo de los pivotes sean 0. Esto se logra mediante las operaciones elementales, llevando la matriz a su forma escalonada reducida.

## Uso del Programa

El programa facilita la resolución de sistemas de ecuaciones lineales aplicando la eliminación de Gauss-Jordan.

### Pasos para Utilizar el Programa

1. **Iniciar el Programa:** Ejecute el programa y espere a que solicite el tamaño de la matriz.

2. **Ingresar Tamaño de la Matriz:**
   - Ingrese el tamaño \( n \) para una matriz cuadrada de \( n \times n \).
   - Incluya una columna adicional para la matriz extendida del sistema \( Ax = b \).

3. **Introducir los Elementos de la Matriz:**
   - Ingrese los valores de cada fila, separados por espacios.
   - Incluya los valores de la matriz extendida al final de cada fila.
   - Ejemplo: para una matriz \( 3 \times 3 \) con matriz extendida, ingrese 4 valores por fila.

4. **Visualización y Procesamiento:**
   - Tras ingresar todas las filas, se mostrará la matriz.
   - El programa realizará la eliminación de Gauss-Jordan y mostrará la matriz resultante.

5. **Interpretación de Resultados:**
   - La matriz resultante revela las relaciones lineales entre las variables.
   - Si hay inconsistencias (fila de ceros igual a un valor real), se mostrará un mensaje de error.

### Ejemplo de Uso

Para resolver el sistema:

\[ 
\begin{align*}
x + 2y + 3z &= 4 \\
2x + 4y + 6z &= 8 \\
3x + 5y + 7z &= 9 
\end{align*}
\]

La entrada sería:

$: Tamaño de la matriz (n x n):
3
$: Ingrese la fila 1 (separada por espacios), recuerde añadir el valor de la matriz extendida:
1 2 3 4
$: Ingrese la fila 2 (separada por espacios), recuerde añadir el valor de la matriz extendida:
2 4 6 8
$: Ingrese la fila 3 (separada por espacios), recuerde añadir el valor de la matriz extendida:
3 5 7 9

El programa procesará la matriz y mostrará el resultado en su forma escalonada reducida.

### Notas Importantes

- Asegúrese de ingresar los datos correctamente para obtener resultados precisos.
- Revise la consistencia de su sistema de ecuaciones y los datos ingresados en caso de errores.
