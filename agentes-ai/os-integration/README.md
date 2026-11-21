# Unidad 11 — Agentes y sistemas operativos

[Issue #12](https://github.com/SantosJMM/os-ai-research/issues/12)

## Temas
- Identidad y autoridad de agentes.
- Herramientas y ejecutores de políticas.
- Aislamiento del proceso y control de recursos.
- IPC, eventos, suscripciones y reanudación.
- Auditoría, aprobaciones, revocación, cancelación e idempotencia.
- Task Runtime, descubrimiento de servicios e interfaces de aplicaciones.
- Superficies de prompt injection y separación de datos de instrucciones.

## Experimento propuesto
Un agente simulado envía peticiones a un ejecutor independiente; el ejecutor evalúa una política y responde con aprobación o denegación. Registrar efectos del sistema y verificar que una petición prohibida no modifica recursos.

## Pregunta transversal
¿Qué protección debe proporcionar la CPU, el kernel, un servicio privilegiado y el runtime del agente? Evitar atribuir al modelo garantías que requieren aislamiento real.

## Relación con Thanos OS
Tomar Station, Ship, Capsuleer y SASP (antes TUP) como casos comparativos, no como respuestas predefinidas.
