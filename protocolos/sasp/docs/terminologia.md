# Terminología de SASP

> **Estado:** convención editorial y arquitectónica para la documentación de SASP. Este glosario define nombres de **roles y responsabilidades**, no nombres obligatorios de procesos, interfaces RPC ni identificadores JSON. Las implementaciones concretas pueden emplear otros nombres.

## 1. Principio de independencia del sistema operativo

**SASP — Semantic Application & Service Protocol** estudia cómo integrar MCP y MCP Apps en la instalación, registro, descubrimiento, conexión, observación y presentación de aplicaciones y servicios de un sistema operativo.

La especificación y sus diagramas generales deben utilizar **términos independientes de Thanos OS**. Los nombres de componentes de Thanos OS son válidos cuando se documenta **esa implementación concreta**, pero no deben convertirse en requisitos para otros sistemas operativos.

La [DECISION-0002](../DECISION-0002-integracion-mcp-en-so.md) establece el alcance MCP-first. Este documento complementa el [glosario conceptual](concepts.md), que detalla entidades y datos del protocolo.

## 2. Roles generales preferidos

| Término SASP | Equivalente en inglés | Responsabilidad | No implica |
| --- | --- | --- | --- |
| **Agente** | Agent | Cliente de IA que descubre capacidades, interpreta recursos y solicita operaciones MCP bajo autorización. | Un modelo particular, proceso privilegiado o componente obligatorio del SO. |
| **Presentador** | Presenter | Cliente o capa de UI humana que representa estado y acciones; puede usar GUI nativa, web o MCP Apps. | Que el software tenga ventana, use HTML o implemente una UI específica. |
| **Supervisor** | Supervisor / Service Manager | Controla el ciclo de vida de procesos y servicios y entrega información confiable sobre instancias. | Que SASP reemplace al init del SO. |
| **Autoridad de seguridad** | Security Authority | Autentica sujetos y aplica decisiones de acceso a conexiones y operaciones protegidas. | Que un manifiesto o el registro otorgue permisos. |
| **Ejecutor privilegiado** | Privileged Executor | Ejecuta acciones protegidas únicamente tras aplicar políticas y límites del host, si la arquitectura lo necesita. | Que todas las aplicaciones requieran un servicio root. |
| **Subsistema de auditoría** | Audit Subsystem | Registra hechos de seguridad y operación que el sistema anfitrión requiera conservar. | Un formato obligatorio de log o un registro central único. |
| **Sistema operativo anfitrión** | Host OS | Entorno donde se instalan y ejecutan aplicaciones y se aplican políticas. | Exclusividad de Linux o Thanos OS. |
| **Proveedor** | Provider | Aplicación o servicio que publica recursos, herramientas y otras capacidades. | Una instalación activa o identidad autenticada por sí sola. |
| **Instancia** | Instance | Ejecución concreta de un proveedor, distinguible de su identidad lógica y del paquete. | Un PID reutilizable como identidad suficiente. |
| **Cliente MCP** | MCP Client | Participante que se comunica con servidores MCP; un Agente o un Presentador pueden utilizarlo. | Un agente de IA como requisito. |

**Regla:** *Agente* y *Presentador* son roles **distintos**, pero un proceso puede desempeñar ambos. *Supervisor*, *Autoridad de seguridad*, *Ejecutor privilegiado* y *Subsistema de auditoría* pueden estar implementados en uno o varios componentes del SO, siempre que sus responsabilidades y límites permanezcan distinguibles.

## 3. Correspondencia con Thanos OS

| Implementación de referencia | Término general SASP | Uso recomendado |
| --- | --- | --- |
| **Capsuleer** | **Agente** | Escribir **Agente** en arquitectura SASP; conservar **Capsuleer** únicamente al explicar el nombre del componente concreto de Thanos OS o su historia. |
| **Thanos Shell** | **Presentador** | Escribir **Presentador** en el modelo universal; conservar **Thanos Shell** en el apartado de integración de Thanos OS. |
| **Station** | **Supervisor** | Usar **Supervisor** al describir control del ciclo de vida; **Station** identifica el PID 1 de Thanos OS y tiene otras responsabilidades propias. |
| **Ship** | **Autoridad de seguridad** y/o **Ejecutor privilegiado**, según función | Usar el rol específico; **Ship** combina verificación de políticas y ejecución protegida en Thanos OS. No simplificar todos sus comportamientos a «autoridad». |
| **Atlas** | **Subsistema de auditoría** | Usar el rol genérico; conservar **Atlas** al explicar su implementación concreta y encadenamiento de auditoría. |
| **Thanos OS** | **Sistema operativo anfitrión** | Utilizar el término genérico para requisitos y diagramas multiplataforma; conservar **Thanos OS** como caso de referencia. |
| **Thanos Music** | **Aplicación de ejemplo / reproductor musical** | Mantener el nombre histórico en los documentos TUP y enlaces; usar **Music Player** para experimentos SASP genéricos cuando corresponda. |
| **TUP** | Antecedente histórico de SASP | No renombrar retrospectivamente documentos ni decisiones anteriores. |

Estas correspondencias son **de responsabilidades**, no promesas de sustitución uno a uno entre procesos. SASP tampoco prescribe los nombres de ejecutables del SO anfitrión.

## 4. Terminología específica de SASP y MCP

| Concepto | Distinción |
| --- | --- |
| **SASP Registry** | Catálogo **local** de instalaciones e instancias verificadas del host; no es MCP Registry. |
| **MCP Registry** | Ecosistema de publicación y metadatos de servidores MCP; no es un inventario en vivo de procesos. |
| **SASP Discovery** | Búsqueda autorizada de proveedores/instancias del host, que puede ofrecerse como **servidor MCP** del SO. |
| **MCP `server/discover`** | Consulta de capacidades y versiones de **un servidor MCP ya localizado**; no explora automáticamente todas las aplicaciones del host. |
| **SASP Connection** | Resolución y conectividad autorizada usando transportes MCP existentes, o mediación cuando sea necesaria. |
| **MCP Apps** | Extensión opcional de presentación interactiva en hosts compatibles; no equivale a **Presentador** ni a ventanas nativas. |
| **Task Runtime** | Motor de ejecución de tareas del SO, si existe; es diferente de las capacidades de la extensión MCP Tasks. |
| **Contrato de proveedor** | Descripción semántica de recursos, métodos, estado y contexto; es **una parte de SASP**, no todo el protocolo. |

## 5. Reglas de redacción y ejemplos

En las **secciones generales**:

- «El **Agente** consulta **SASP Discovery** para encontrar proveedores accesibles».
- «El **Supervisor** informa al **SASP Registry** de una instancia válida».
- «El **Presentador** puede usar MCP Apps o una GUI nativa».
- «La **Autoridad de seguridad** verifica permisos; el **Ejecutor privilegiado** aplica los efectos protegidos cuando corresponda».
- «El **Subsistema de auditoría** registra la operación de acuerdo con las políticas del host».

En la sección **«Integración en Thanos OS»** se pueden presentar equivalencias concretas: «**Station** desempeña el rol de Supervisor», «**Ship** concentra funciones de Autoridad y Ejecutor», «**Atlas** desempeña el rol de auditoría», «**Thanos Shell** es un Presentador» y «**Capsuleer** es la implementación específica del Agente».

**No cambiar** nombres de proyectos, rutas históricas, código Rust, claves JSON, identificadores o contratos publicados solamente para uniformar palabras de la documentación. Cualquier cambio de API o identificador requiere una decisión específica y pruebas de compatibilidad.

## 6. Documentos relacionados

- [Integración MCP con el sistema operativo](os-integration.md)
- [Arquitectura SASP](architecture.md)
- [Conceptos y vocabulario técnico](concepts.md)
- [SASP Registry y Discovery](registry-discovery.md)
- [DECISION-0001 — nombre y alcance](../DECISION-0001-nombre-y-alcance.md)
- [DECISION-0002 — integración MCP en el SO](../DECISION-0002-integracion-mcp-en-so.md)
