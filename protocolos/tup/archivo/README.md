# Archivo técnico de TUP

**TUP — Thanos UI Protocol** fue el nombre original de esta línea de investigación, posteriormente denominada **SASP — Semantic Application & Service Protocol**.

El archivo reúne los principales conceptos de diseño de TUP: entidades, contratos, estructura de mensajes, acciones, modelos de ejecución y seguridad.

## Documentos

- [README inicial de TUP](../README.md) — visión, alcance y primeros experimentos.
- [01 — TUP v0.1: Task, View y App](01-propuesta-v01.md) — arquitectura y responsabilidades conceptuales.
- [02 — Ejemplos JSON y JSON-RPC](02-ejemplos-json.md) — manifiestos, eventos, solicitudes y respuestas.
- [03 — Acciones, permisos y Task Runtime](03-acciones-y-runtime.md) — ejecución, autorizaciones, cancelación y recuperación.
- [04 — De TUP a SASP: origen, hallazgos y evolución](04-origen-hallazgos-evolucion.md) — motivación inicial, contraste con MCP/MCP Apps, cambios de alcance y preguntas pendientes.

## Estado de diseño

Los anexos contienen **propuestas técnicas y ejemplos no normativos**. Las estructuras ilustran el modelo de interacción, pero todavía requieren especificar esquemas, transporte, seguridad, errores y garantías de interoperabilidad.

## Evolución

La investigación continúa bajo el nombre [SASP](../../sasp/README.md), con alcance independiente de Thanos OS y abierto a aplicaciones y servicios con o sin interfaz gráfica.
