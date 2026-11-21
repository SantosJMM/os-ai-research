# Reglas de trabajo para SASP

Estas reglas aplican a las modificaciones bajo `protocolos/sasp/`. Se complementan con [las normas del repositorio](../../AGENTS.md).

## Identidad

**SASP — Semantic Application & Service Protocol** se investiga como **integración de MCP y MCP Apps con el sistema operativo**. **No es un reemplazo de MCP ni solamente un contrato JSON**.

Mantener explícitos los seis ejes: **Application & Service Contract, SASP Registry, SASP Discovery, SASP Connection, SASP Lifecycle & Events y MCP Integration & Presentation**.

La [DECISION-0002](DECISION-0002-integracion-mcp-en-so.md) establece la dirección vigente. La [DECISION-0001](DECISION-0001-nombre-y-alcance.md) conserva la decisión de nombre y su contexto.

## Criterios técnicos

- Reutilizar MCP Tools, Resources, notificaciones, Tasks y MCP Apps antes de proponer nuevos métodos, transportes o extensiones.
- Distinguir **MCP Registry público**, **SASP Registry local** y **`server/discover` de un servidor MCP**.
- Discovery puede exponerse como servicio MCP del SO, utilizando herramientas y recursos convencionales; no inventar otro JSON-RPC por defecto.
- Mantener separados **paquete instalado**, **proveedor**, **instancia verificada**, **proceso**, **endpoint** y **autorización**.
- No convertir manifiestos, nombres de herramientas ni identificadores del cliente en permisos efectivos.
- Usar estado y suscripciones MCP como punto de partida; solo diseñar eventos SASP con garantías adicionales cuando se demuestre una necesidad no cubierta.
- No exigir HTML ni MCP Apps a aplicaciones nativas o daemons.
- No atribuir funcionalidades de protocolo de 2026 a SDKs sin verificar su versión y comportamiento en pruebas reales.
- Conservar los ejemplos históricos de TUP sin transformarlos silenciosamente en una especificación nueva.

## Estado de las investigaciones

Diferenciar **propuesta**, **archivo creado**, **código existente**, **prueba ejecutada**, **resultado reproducible** y **decisión adoptada**. Publicar las evidencias y los límites de cualquier experimento; no señalar como completada una prueba únicamente porque exista código.

## Estilo documental

Producir documentación técnica pública para desarrolladores. No incluir comentarios internos sobre asistentes, prompts, conversaciones o el proceso de redacción. Describir límites de la tecnología y madurez de los contratos cuando resulte relevante para implementadores.
