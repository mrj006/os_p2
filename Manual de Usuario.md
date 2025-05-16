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

### Estructura del Código

El proyecto está organizado de forma modular para facilitar la lectura, mantenimiento y pruebas. A continuación se describe cada componente clave de la arquitectura:

#### /src

Contiene todo el código fuente del proyecto.

---

#### /server

Encargado del manejo de las solicitudes HTTP y la lógica central del servidor.

- `main.rs`: Punto de entrada del servidor. Inicia el socket TCP, configura el thread pool y ejecuta el ciclo principal.
- `request.rs`: Define la estructura `HttpRequest`, descompone la URI y extrae parámetros.
- `response.rs`: Construye respuestas HTTP manualmente (status line, headers y cuerpo).
- `parser.rs`: Realiza el análisis de la solicitud HTTP entrante.
- `routes.rs`: Contiene el enrutador que llama a las funciones correctas según la ruta.
- `mod.rs`: Módulo raíz para importar y exponer los demás componentes de `server`.

---

#### /functions

Contiene la implementación de cada funcionalidad expuesta como endpoint.

- `fibonacci.rs`: Calcula el n-ésimo número Fibonacci.
- `toupper.rs`: Convierte un texto a mayúsculas.
- `reverse.rs`: Invierte una cadena.
- `hash.rs`: Calcula el SHA-256 de una entrada.
- `timestamp.rs`: Devuelve la fecha y hora actual.
- `createfile.rs`, `deletefile.rs`: Crea y elimina archivos en el sistema.
- `simulate.rs`, `sleep.rs`: Simulan retrasos en la ejecución.
- `random.rs`: Genera números aleatorios dentro de un rango.
- `help.rs`: Devuelve una lista con todos los comandos disponibles.
- `mod.rs`: Expone las funciones al resto del proyecto.

---

#### /pool

Maneja la ejecución concurrente del servidor.

- `threadpool.rs`: Implementación del `ThreadPool` personalizado.
- `status.rs`: Proporciona el estado actual del servidor (PID, uptime, etc.) para la ruta `/status`.
- `mod.rs`: Módulo de exposición de los elementos del pool.

---

#### /errors

Contiene los tipos y manejadores de errores definidos para distintas etapas del procesamiento.

- `parse.rs`: Errores relacionados con el análisis de solicitudes.
- `server.rs`: Errores generales del servidor.
- `pool.rs`: Errores relacionados con el sistema de hilos y concurrencia.
- `implement.rs`: Posibles errores personalizados.
- `mod.rs`: Módulo centralizador para importar y exponer todos los errores.

---

#### Cargo.toml

Archivo de configuración del proyecto en Rust. Define las dependencias, el nombre del paquete y la versión del compilador utilizada.

---

### Ejecución de pruebas con Postman

Esta colección contiene pruebas para cada uno de los 12 endpoints implementados por el servidor, incluyendo:

- Un caso positivo por ruta (espera código `200 OK`)
- Un caso negativo por ruta (espera código `400 Bad Request`), cuando aplica
- La ruta `/timestamp` incluye solo un caso positivo, ya que no tiene parámetros que puedan generar error

Se debe de ejecutar los comandos de la colección manualmente o por medio del comando "Runner", asegurandose de que el servidor este corriendo.
