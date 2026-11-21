# DECISION-0002 — SASP como integración de MCP en el sistema operativo

- **Fecha:** 2026-11-21
- **Estado de esta decisión:** Adoptada como dirección de investigación de OS AI Research
- **Madurez técnica:** Arquitectura propuesta; pendiente de implementación y validación
- **Relación:** Complementa y actualiza el alcance de [DECISION-0001](DECISION-0001-nombre-y-alcance.md), sin cambiar el nombre SASP

## Contexto

SASP surgió de TUP (*Thanos UI Protocol*) para que las aplicaciones declarasen estado, métodos, eventos y contexto de interfaz de forma comprensible para agentes y personas. La comparación con **Model Context Protocol (MCP)** y **MCP Apps** confirmó que muchas de las primitivas necesarias ya existen: herramientas, recursos, esquemas, suscripciones y presentación interactiva opcional.

La cuestión principal no es diseñar otro transporte JSON-RPC ni duplicar todas las funciones de MCP, sino **integrar esos mecanismos con el modelo operativo de aplicaciones y servicios de un sistema operativo**.

Hay responsabilidades que una conexión MCP individual no resuelve por sí sola: catálogo local de software instalado, registro de instancias verificadas, búsqueda por capacidades a través de distintos proveedores, política de conexión, estado de procesos y correlación con el supervisor del SO.

## Decisión

**SASP — Semantic Application & Service Protocol** se desarrolla como una propuesta abierta de **integración de aplicaciones y servicios con sistemas operativos basada en MCP**.

Su objetivo es especificar y experimentar cómo un SO **registra, descubre, conecta, observa y presenta** proveedores que exponen herramientas, recursos, eventos y capacidades semánticas.

**MCP se utiliza como base de comunicación e interacción cuando cubra los requisitos**. **MCP Apps** se reutiliza para presentación interactiva cuando resulte apropiado. El contrato semántico de proveedor sigue formando parte de SASP, pero **no define por sí solo el alcance completo**.

La investigación se estructura en seis ejes:

1. **Application & Service Contract:** declaración de identidad, capacidades, estado y contexto, maximizando la reutilización de MCP Tools, Resources y JSON Schema.
2. **SASP Registry:** integración del inventario de instalaciones con el directorio de instancias activas y sus identidades verificadas.
3. **SASP Discovery:** búsqueda autorizada por identidad/capacidad y resolución de instancias, con un servidor MCP del SO como primera opción a experimentar.
4. **SASP Connection:** lanzamiento y canales autorizados, usando MCP por `stdio`, sockets Unix, HTTP o mediación según las condiciones del host.
5. **SASP Lifecycle & Events:** distinción de instalación, proceso, endpoint y tarea; uso de notificaciones MCP y definición de garantías adicionales solo si se demuestra su necesidad.
6. **MCP Integration & Presentation:** clientes humanos y agentes; MCP Apps opcional junto a UI nativa y aplicaciones sin ventana.

La **primera prioridad experimental** será demostrar **Registry local + Discovery mediante MCP**, apoyándose en el experimento existente de Tools/Resources para Music Player y Backup Daemon.

## Principios y límites

- **No reemplazar MCP**, MCP Apps, Linux ni los protocolos nativos de cada servicio.
- **No crear nuevos métodos de transporte por preferencia de arquitectura**. Evaluar primero MCP y sus extensiones oficiales.
- **Descubrir no es autorizar**. Registrar un proveedor, conocer su PID, encontrar su endpoint o leer su manifiesto no concede permisos.
- **Separar datos de instalación y ejecución.** MCP Registry público, `server/discover` y el registro local SASP cumplen funciones diferentes.
- **Conservar la independencia de Thanos OS**: otras plataformas pueden implementar la integración con sus gestores de paquetes, supervisores y autoridades.
- **MCP Apps no es obligatorio ni suficiente por definición para todas las interfaces nativas**. Su alcance debe probarse con casos GTK/Qt, Shell y servicios sin interfaz.
- **Las extensiones SASP requieren evidencia**, criterios de conformidad y comparación de complejidad con mecanismos ya existentes.

## Consecuencias

- La documentación de nivel superior presenta **integración con el SO** como misión de SASP y el **contrato semántico** como uno de sus componentes.
- Los ejemplos `manifest.json` y `state.snapshot.json` continúan como **datos experimentales**, no como una alternativa completa al estándar MCP.
- Los componentes Registry, Discovery, Connection y Lifecycle son roles arquitectónicos; no se exige un proceso separado para cada rol.
- El prototipo Rust actual queda identificado como **línea base MCP Tools/Resources** y no como implementación completa de SASP.
- Las decisiones sobre esquemas propios, eventos persistentes, broker y extensiones de UI quedan subordinadas a resultados reproducibles.

## Evidencia y experimentos

- [Arquitectura de integración del SO](docs/os-integration.md)
- [Plan OS-01 a OS-06](experiments/os-integration/README.md)
- [Discovery como servicio MCP](experiments/os-integration/discovery-mcp.md)
- [Contrato mínimo experimental](specification/draft/overview.md)
- [MCP — especificación oficial](https://modelcontextprotocol.io/specification/2026-07-28)
- [MCP Registry](https://github.com/modelcontextprotocol/registry)
- [MCP Apps](https://github.com/modelcontextprotocol/ext-apps)

## Revisión de la decisión

La dirección se revisará si los experimentos demuestran que un enfoque MCP-first no cubre requisitos observables de identidad, descubrimiento local, seguridad, eventos o presentación. Cualquier extensión normativa nueva deberá describir el requisito no cubierto y su prueba de conformidad.
