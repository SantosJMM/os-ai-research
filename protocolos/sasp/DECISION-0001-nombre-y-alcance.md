# DECISION-0001 — Nombre y alcance de SASP

- **Fecha:** 2026-11-21
- **Estado de esta decisión:** Adoptada en OS AI Research
- **Estado del protocolo:** Propuesta de investigación (borrador; sin versión normativa)
- **Anteriormente:** Thanos UI Protocol (TUP)
- **Evolución de alcance:** [DECISION-0002 — integración de MCP con el sistema operativo](DECISION-0002-integracion-mcp-en-so.md)

## Contexto

El proyecto empezó bajo el nombre **Thanos UI Protocol (TUP)**. Al definir el alcance observamos que no está limitado a UI gráfica ni a aplicaciones Thanos: también puede servir a procesos y daemons, y pretende funcionar con agentes o clientes distintos a Capsuleer. Conocemos capacidades de MCP y MCP Apps que se solapan y deben aprovecharse o compararse antes de justificar un protocolo distinto.

## Decisión

Adoptamos el nombre **SASP — Semantic Application & Service Protocol** («Protocolo Semántico de Aplicaciones y Servicios»). Se propone pronunciar **«Sasp»**, en una sílaba (aproximaciones: /sæsp/ en inglés, /sasp/ en español).

El alcance de investigación es la descripción semántica de **identidad, estado, capacidades, métodos, eventos y contexto de interacción** de aplicaciones y servicios, con o sin interfaz gráfica.

## Límites explícitos

- SASP **no sustituye** señales, syscalls, supervisión de procesos, permisos, políticas de ejecución ni APIs nativas.
- SASP **no concede autoridad** a quien descubre una capacidad.
- SASP **no exige** usar Thanos OS ni un toolkit visual concreto.
- SASP **no obliga** a crear un transporte separado: MCP, MCP Apps, sockets Unix y broker son alternativas o puentes a estudiar.
- Nombrar la propuesta **no equivale** a publicar un estándar técnico estable, declarar compatibilidad verificada ni afirmar originalidad exclusiva.

## Consecuencias

- Nuevos documentos y experimentos usarán `SASP` y la ruta `protocolos/sasp/`.
- La antigua ruta `protocolos/tup/` mantendrá una referencia histórica y un enlace a SASP.
- El Issue #13 mantiene su número, historial y función de seguimiento.
- Los experimentos se identificarán como SASP-01, SASP-02, etcétera.

## Pregunta pendiente

¿El contrato semántico SASP necesita un protocolo propio, o puede expresarse como perfil/extensión de MCP y sus mecanismos existentes? Se resolverá mediante prototipos y comparaciones, no por preferencia de nombre.
