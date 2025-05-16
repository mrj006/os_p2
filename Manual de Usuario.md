# Manual de Usuario

## Requisitos del Sistema

Sistema Operativo:
GNU/Linux (recomendado Debian/Ubuntu)

Dependencias:
Rust Y Cargo

## Instalación

1) Clonar el repositorio

2) Instalar Rust: 
```bash
curl https://sh.rustup.rs -sSf | sh
```
3) Recargar la terminal

4) Compilar el proyecto:
```bash
cargo build --release
```

## Ejecutar el Proyecto

Ejecutar el servidor:
```bash
cargo run
```
El servidor se iniciará por defecto en el puerto 7878 y escuchará peticiones entrantes

## Uso del servidor

### Funciones:

| Ruta                                            | Descripción                                           |
| ----------------------------------------------- | ----------------------------------------------------- |
| `/reverse?text=abc`                             | Invierte el texto                                     |
| `/toupper?text=abc`                             | Convierte a mayúsculas                                |
| `/fibonacci?num=N`                              | Devuelve el N-ésimo número Fibonacci                  |
| `/hash?text=text`                               | Devuelve el SHA-256 del texto                         |
| `/timestamp`                                    | Devuelve la hora del sistema en ISO-8601              |
| `/random?count=n&min=a&max=b`                   | N números aleatorios entre a y b                      |
| `/createfile?name=file.txt&content=hi&repeat=3` | Crea archivo                                          |
| `/deletefile?name=file.txt`                     | Elimina archivo                                       |
| `/sleep?seconds=s`                              | Espera s segundos                                     |
| `/simulate?seconds=s&task=nombre`               | Simula una tarea con retardo                          |
| `/loadtest?tasks=n&sleep=s`                     | Ejecuta n tareas de sleep(s) controladas por hilo     |
| `/help`                                         | Lista todos los comandos disponibles                  |

### Ejecución de pruebas con Postman

Esta colección contiene pruebas para cada uno de los 12 endpoints implementados por el servidor, incluyendo:

- Un caso positivo por ruta (espera código `200 OK`)
- Un caso negativo por ruta (espera código `400 Bad Request`), cuando aplica
- La ruta `/timestamp` incluye solo un caso positivo, ya que no tiene parámetros que puedan generar error

Se debe de ejecutar los comandos de la colección manualmente o por medio del comando "Runner", asegurandose de que el servidor este corriendo.
