# Empezando con TDD

## 1.- Gameplan

Como tengo muchas ideas en la cabeza sobre lo que puede terminar siendo esto, voy a empezar aplicando TDD (Test Driven Development). Es una buena forma de mantenerme a raya y no implmentar cosas random que voy a terminar cambiando en 2 semanas. Un pequeño refresher de la metodología del TDD:

1. Escribir un test de funcionalidad que *falle*.
2. Implementar la solución más simple posible para que el test corra
3. Refactorizar el código de ser necesario, para mantener orden y buenas prácticas
4. Repetir desde el paso 1, cuidando que *siempre pasen todos los tests*.

Con estos 3/4 pasos puedo desarrollar código sin que se me vaya de las manos y me quede un choclo de 85 funciones de las cuales funcionan 5 y uso 4.

## 2.- Primer paso: El Ejercicio

Mi objetivo principal era manejar y navegar ejercicios de una materia, así que, como mínimo, el programa tendrá que tener una idea de lo que es *un ejercicio*. Empiezo por esa clase (o mejor dicho, por los *tests* de esa clase). Empecé por el primer test:

```Python
# File: tests/ExcerciseTest.py

class ExcerciseTest(unittest.TestCase):
    def test_01_excercise_belongs_to_assignature(self):
        exc = Excercise(assignature='Análisis Matemático 2')
        self.assertEqual(exc.assignature(), 'Análisis Matemático 2')
```
