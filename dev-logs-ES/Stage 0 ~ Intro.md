# FaQHelper ~ La idea

## 1.- Idea inicial

Un día (el mismo que creé este repo btw) estaba pensando "man, la verdad que necesitaría crearme un repo/carpeta con ejercicios de álgebra y análisis para no tener que improvisar en clase" (contexto: doy clases de álgebra y análisis). El pensamiento que le siguió inmediatamente a ese es "cómo los podría organizar de mejor forma?".

Podría dividirlos por guía de práctica, por tema, o por origen (guía de práctica, parcial, invento mío, etc). El primer criterio tiene el problema de que está atado a la guía de tps que es particular a esta materia (álgebra lineal) de esta facultad (F. Ing. UBA) en este cuatrimestre (1c2024), y no soy fan de basar cosas en contingencias. La segunda tiene la desventaja de que hay ejercicios que abarcan *varios temas* y no soy fan de tener varias copias del mismo archivo (ni mantener varios simlinks entre carpetas). La tercera, aunque buena partición, no tiene *nada que ver* con el contenido de los ejercicios, y resultaría incómoda de navegar.

Como buen estudiante de ingeniería de software después de leer 3 quotes de Kent Beck, fui por la opción de hacer una solución *mantenible y escalable*, en terminos de navegar y ver ejercicios, parciales, etc.

## 2.- El Objetivo

Quiero crear una aplicación/script que me ayude a organizar y navegar distintos ejercicios, guías y exámenes. Inicialmente, quiero que funcione para la *materia estándar de matemática* (i.e. tiene ejercicios con enunciado expresable en prosa, resolución expresable en prosa, y que abarcan un par de temas), y si todo sale bien (y soy bueno programando), de ahí expandirlo a otro tipo de materias (p. ej. preguntas con respuesa en código), otro tipo de material (apuntes, cheatsheets), y agregar funcionalidad adicional (simulación de parciales, comprobación de respuestas, etc). Dejo de listar features que quiero agregar porque nada más mientras escribía este párrafo se me ocurrieron como 8.

Un primer objetivo a mediano plazo de este proyecto es que yo le pueda pedir al script cosas del estilo: "dame ejercicios de producto interno y búsqueda de bases" y que me responda correctamente. Además, como requerimiento no funcional quisiera que ***ninguna*** parte del código me de ganas de llorar cuando la vuelva a leer más adelante (requerimiento difícil).

## 3.- Las Herramientas

Por un lado, me gusta mucho programar con objetos. Me parece una forma muy natural de modelar, organizar y escribir código. Por otro lado, hace poco descubrí un juguete nuevo (Rust) y me gustaría usarlo (cargo my beloved).

En un principio había pensado en hacerlo en python para hacerlo bien orientado a objetos y polimorfismo y tal, pero al final me decidí por Rust (no lean el mensaje del 2do commit¿). Sospecho que el sistema de tipado y los traits van a ser más convenientes para lo que quiero implementar.

## 4.- La Historia

Además de empezar el proyecto, decidí empezar un *dev journal* para registrar el desarrollo entero (lo habrán notado después de 8 párrafos no?). En vez de dividirlo por semanas que es lo habitual, lo voy a dividir en *stages*, subdivididos en *chapters* (si, soy fan de Initial D). Como la consistencia nunca ha sido mi punto fuerte, me parece mucho mejor agruparlo por los stints de desarrollo de funcionalidades relacionadas que pueda implementar en el equivalente a un par de semanas de desarrollo continuo, que tener un log por semana que la mitad de las veces esté vacío. 
