# Laboratorio 01 — Del programa Rust al kernel Linux

**Issue:** [#1](https://github.com/SantosJMM/os-research/issues/1)

## Pregunta

¿Qué sucede cuando un programa de usuario solicita un servicio al kernel Linux?

## Conceptos a estudiar

- Privilegios de CPU y separación entre espacio de usuario y kernel.
- Syscall frente a llamada de función ordinaria e interrupción.
- Identificador de syscall, argumentos y valor devuelto.
- Arquitectura x86-64 como caso inicial; otras arquitecturas tienen convenciones distintas.

## Experimento A: observar las syscalls de un programa Rust

Requisitos: Linux x86-64, Rust y `strace`. Ejecutar sobre un entorno de laboratorio y **no** usar credenciales ni archivos de producción.

Crear `hello.rs`:

```rust
fn main() {
    println!("Hola desde espacio de usuario");
}
```

Compilar y ejecutar:

```bash
rustc hello.rs -o hello
strace -f -e trace=write,exit_group ./hello
```

Esperado: una syscall de escritura asociada a la salida del proceso y terminación del programa. La secuencia exacta puede variar según Rust, libc, arquitectura y versión del kernel.

## Experimento B: llamada directa con libc

En un proyecto Cargo de laboratorio con dependencia `libc`, llamar `libc::syscall(libc::SYS_getpid)`. Comparar el resultado con `std::process::id()`. El objetivo es estudiar la frontera usuario/kernel, no reemplazar APIs seguras por syscalls directas en producción.

## Inspección de código fuente

En el árbol fuente del kernel correspondiente a la arquitectura y versión, localizar el camino de entrada de syscalls x86-64 y la implementación de `getpid`. Anotar rutas y commit exactos; no asumir que las rutas son idénticas entre versiones.

## Cuestionario de comprensión (sin respuestas por adelantado)

1. ¿Qué diferencia hay entre syscall e IPC?
2. ¿Por qué el programa no puede acceder libremente a la memoria del kernel?
3. ¿Qué responsabilidad tiene la CPU y cuál tiene el kernel?
4. ¿Qué papel cumple `strace` y qué no puede demostrar?

## Evidencia mínima

- [ ] Versión del kernel (`uname -r`), Rust (`rustc --version`), arquitectura (`uname -m`).
- [ ] Código y comandos para reproducir.
- [ ] Extracto de `strace` sin datos sensibles.
- [ ] Diagrama usuario → syscall → kernel → retorno.
- [ ] Referencias al código del kernel de la versión estudiada.
- [ ] Observaciones, errores y límites de interpretación.

## Lecturas recomendadas

- https://man7.org/linux/man-pages/man2/syscall.2.html
- https://man7.org/linux/man-pages/man2/getpid.2.html
- https://docs.kernel.org/
