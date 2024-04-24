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

```Rust
// File: src/excercise/mod.rs
#[cfg(test)]
mod tests{
    #[test]
    fn test_01_excercise_belongs_to_assignature() {
        let exc = Excercise::new("Álgebra Lineal".to_string());
        assert_eq!(exc.is_for_assignature("Álgebra Lineal"), true);
        assert_eq!(exc.is_for_assignature("Física 1"), false);
    }
}
```

Es bastante natural: debe haber alguna clase de struct *ejercicio*, que tenga por lo menos una *asignatura*. En cuanto a sintaxis de Rust, `#cfg(test)` configura el módulo para que sea de pruebas, y `assert_eq!` es una macro que falla el test si los dos argumentos no son iguales.

Obviamente así como está, no pasa porque `Excercise::new` ni siquiera existe aún (eso sí, se habrán dado cuenta por el nombre del archivo que ya creé el módulo `Excercise`). Vamos a crear la struct:

```Rust
// File: src/excercise/mod.rs
struct Excercise{
    assignature: String
}

impl Excercise {
    pub fn new(assignature: String) -> Excercise{
        Excercise {
            assignature
        }
    }

    pub fn is_for_assignature(&self, assignature: &str) -> bool {
        self.assignature == assignature
    }
}
```

Como ven, esta implementación es de lo más sencilla y hace que pase el test 1. Una decisión importante es que implementé `is_for_assignature` para verificar de que asignatura es un ejercicio en vez de hacer un getter del atributo o hacerlo público. Esto es porque no sé si en el futuro `assignature` va a ser una string o no (de hecho, sospecho que no), pero si sé que me gustaría poder buscar asignaturas por strings, así que la función me permite poder hacer comparaciones más complicadas por debajo en un futuro sin tener que cambiar la api pública. Este es el famoso encapsulamiento.

## 3.- Expandiendo funcionalidad con más tests

Hora de implementar un segundo test. También quería que los ejercicios tuvieran tags. También ma gustaría que se pudieran cambiar las tags de un ejercicio existente (como en todo foro, uno puede añadir o quitar tags cuando quiera). Un buen segundo test me pareció:

```Rust
// File: src/excercise/mod.rs
#[test]
fn test_02_can_add_tags_to_excercise() {
    let mut exc = Excercise::new("Álgebra Lineal".to_string());
    exc.add_tag("Matrices".to_string());
    assert_eq!(exc.has_tag("Matrices"), true);
}
```

De aquí puedo ver que voy a querer que los ejercicios sean mutables (ugh, eso va a traer problemas después, pero no se le puede hacer mucho). Cambiemos la struct de ejercicio para fitear los nuevos cambios:

```Rust
// File: src/excercise/mod.rs
struct Excercise{
    assignature: String,
    tags: Vec<String>,
}

impl Excercise {
    pub fn new(assignature: String) -> Excercise{
        Excercise {
            assignature,
            tags: Vec::new()
        }
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    }
}
```

Otra vez, uso funciones en lugar de getters porque no sé todavía si las tags van a terminar siendo strings o no, y ni siquiera si van a terminar estando en un vector u otro tipo de colección.

Después de un par de tests más, mi suite tenía esta pinta:

```Rust
#[cfg(test)]
mod tests {
    fn test_exc_1() -> Excercise {
        Excercise::new(
            "Álgebra Lineal".to_string(),
            "tests/testfiles/exc1.md".to_string(),
        )
    }

    #[test]
    fn test_01_excercise_belongs_to_assignature() {
        let exc = test_exc_1();
        assert_eq!(exc.is_for_assignature("Álgebra Lineal"), true);
        assert_eq!(exc.is_for_assignature("Física 1"), false);
    }

    #[test]
    fn test_02_can_add_tags_to_excercise() {
        let mut exc = test_exc_1();
        exc.add_tag("Matrices".to_string());
        assert_eq!(exc.has_tag("Matrices"), true);
    }

    #[test]
    fn test_03_excercise_can_have_multiple_tags() {
        let mut exc = test_exc_1();
        exc.add_tag("Matrices".to_string());
        exc.add_tag("Determinantes".to_string());
        assert_eq!(exc.has_tag("Matrices"), true);
        assert_eq!(exc.has_tag("Determinantes"), true);
    }

    #[test]
    fn test_04_can_remove_tags_from_excercise() {
        let mut exc = test_exc_1();
        exc.add_tag("Matrices".to_string());
        assert_eq!(exc.has_tag("Matrices"), true);
        exc.remove_tag("Matrices".to_string());
        assert_eq!(exc.has_tag("Matrices"), false);
    }
}
```

Lo único a destacar es que, como necesitaba un ejercicio para cada test, decidí usar el mismo para todos y extraerlo a una función, para que sea más fácil de cambiar si hay un cambio (foreshadowing!)

## 4.- Un cambio más grande

Lo siguiente que me gustaría es que cada ejercicio tenga asociado un archivo (al fin y al cabo, esto es un administrador de archivos). Además, me gustaría que ese archivo *por lo menos* exista. Algo del estilo:

```Rust
#[test]
fn test_05_excercise_has_path() {
    let exc = Excercise::new(
        "Álgebra Lineal".to_string(),
        "tests/testfiles/exc1.md".to_string()
        )
        .unwrap();
    assert_eq!(exc.get_path(), "tests/testfiles/exc1.md");
}

#[test]
fn test_06_path_must_be_valid() {
    let exc = Excercise::new("Álgebra Lineal".to_string(), "not/a/path.txt".to_string());
    assert_eq!(exc.is_err(), true);
}
```

En estos dos tests cambiaron dos cosas importantes de la creación del ejercicio. En primer lugar, ahora necesita también un *path*. En segundo lugar, `new()` ahora necesito devolver un `Result` en lugar de un `Excercise` en caso de que me pasen un path inválido (notar que usé `unwrap()` y `is_err()`). Esto requiere cambios en la api pública de `Excercise` (lo que queríamos evitar, pero en esta etapa tan temprana es esperable):

```Rust
pub struct Excercise {
    assignature: String,
    tags: Vec<String>,
    path: String,
}

impl Excercise {
    pub fn new(assignature: String, path: String) -> Result<Excercise, std::io::Error> {
        if !std::path::Path::new(&path).exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            ));
        }
        Ok(Excercise {
            assignature,
            tags: Vec::new(),
            path,
        })
    }
}
```

Ahora mis nuevos tests pasan, pero los viejos se rompieron porque cambié `new`. Menos mal que sólo llamaba a `new` en un sólo lugar (`test_exc_1()`) y con sólo cambiarlo ahí basta para hacer pasar los tests! (se los dije, foreshadowing):

```Rust
fn test_exc_1() -> Excercise {
    Excercise::new(
        "Álgebra Lineal".to_string(),
        "tests/testfiles/exc1.md".to_string(),
    )
    .unwrap() //hardcoded test file, should always exist
}
```

Con ese cambio vuelven a pasar los tests 1-4 sin tocar ni una línea de ninguno de ellos. Ven, código bien factorizado es fácil de cambiar y adaptar.

## 5.- Conclusión

Este sería un buen bosquejo de la funcionalidad básica que debería tener este struct. Y sólo media tarde de trabajo, y sin funciones que están de adorno! (bueno, el analyzer me sigue tirando warnings de `dead_code` pero eso es porque aún no tengo un main). Para expandir en funcionalidad, va a hacer falta crear nuevos structs.

Los siguientes chapters probablemente sean mucho más acotados. Este es más "jugada a jugada" porque es el primero, y sirve de introducción.

---

- commit: 9f61b8a643db226b4b09f43757e2feeaa1acad05
- fecha: 2024/04/24
