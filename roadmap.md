# Roadmap de estudio de sistemas operativos

Objetivo: construir conocimiento técnico verificable mediante teoría, lectura de código fuente, experimentos repetibles y documentación. Este repositorio es independiente del código de Thanos OS.

## Método para cada unidad

1. **Comprender:** conceptos, terminología y bibliografía primaria.
2. **Inspeccionar:** localizar y estudiar una implementación real.
3. **Experimentar:** construir un laboratorio mínimo y reproducible.
4. **Documentar:** registrar hipótesis, herramientas, versiones, resultados, límites y preguntas abiertas.

**Definición de terminado:** no basta con una explicación conceptual. Cada unidad debe entregar evidencia del laboratorio, instrucciones de reproducción y una conclusión que separe hechos de hipótesis.

## Enfoque de lenguajes

OS Research emplea **C, Rust y Assembly** según lo que facilite la comprensión. Thanos OS mantiene Rust como lenguaje principal; no es necesario portar cada laboratorio a Rust.

## Unidades e Issues

| Unidad | Área | Seguimiento |
| --- | --- | --- |
| 00 | **Arquitectura de computadores: ISA, ABI, registros, privilegios y MMU** — x86-64 primero; RISC-V y ARM64 después | [#10](https://github.com/SantosJMM/os-research/issues/10) |
| 01 | CPU, privilegios, interrupciones y syscalls | [#1](https://github.com/SantosJMM/os-research/issues/1) |
| 02 | Procesos, hilos, planificación y sincronización | [#2](https://github.com/SantosJMM/os-research/issues/2) |
| 03 | IPC: sockets, pipes, memoria compartida y Schemes | [#3](https://github.com/SantosJMM/os-research/issues/3) |
| 04 | Memoria física, virtual y paginación | [#4](https://github.com/SantosJMM/os-research/issues/4) |
| 05 | VFS, archivos, dispositivos y persistencia | [#5](https://github.com/SantosJMM/os-research/issues/5) |
| 06 | Seguridad: aislamiento, capacidades y verificación formal | [#6](https://github.com/SantosJMM/os-research/issues/6) |
| 07 | Kernel mínimo en Rust y QEMU | [#7](https://github.com/SantosJMM/os-research/issues/7) |
| 08 | Linux, Redox, Asterinas y seL4 | [#8](https://github.com/SantosJMM/os-research/issues/8) |
| 09 | IA como participante de primera clase (Thanos OS) | [#9](https://github.com/SantosJMM/os-research/issues/9) |

## Orden de trabajo sugerido

**Ruta principal:** 00 → 01 → 02 → 03 → 04 → 05 → 06 → 07 → 08 → 09.

**Laboratorio transversal:** los temas 03 y 09 pueden explorarse en paralelo con Thanos OS, sin introducir cambios en su repositorio hasta completar pruebas y revisar decisiones.

No hay plazos arbitrarios ni obligación de seguir una cadencia fija: el progreso se mide mediante evidencia de aprendizaje.

## Plantilla mínima para experimentos

```markdown
# Pregunta
# Hipótesis
# Entorno (OS, kernel, versiones, hardware/QEMU)
# Procedimiento reproducible
# Resultados observados
# Limitaciones y riesgos
# Conclusión
# Preguntas abiertas y referencias
```

## Primera práctica

Comenzar por arquitectura de computadores: compilar C, inspeccionar Assembly y registros ([Issue #10](https://github.com/SantosJMM/os-research/issues/10)); después estudiar el recorrido de una syscall en Linux (Issue #1) y, como segundo proyecto, comparar un servidor petición-respuesta mediante socket Unix con un proveedor Scheme de Redox (Issue #3). La comparación debe considerar semántica, seguridad, reproducibilidad y mediciones, sin declarar un ganador de antemano.

## Regla de separación

Las decisiones oficiales y la implementación de Thanos OS permanecen en su repositorio. Aquí se guardan hipótesis, experimentos, resultados y referencias públicas.
