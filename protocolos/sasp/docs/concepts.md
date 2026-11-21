# Conceptos y vocabulario de SASP

> **Estado:** vocabulario de trabajo para la investigación de SASP. Los términos describen responsabilidades y relaciones; no establecen por sí mismos una API ni un formato de transporte normativo.

## 1. Software que participa en SASP

### Aplicación (`Application`)

Programa destinado a una función reconocible para usuarios u otros sistemas. Puede tener interfaz gráfica, consola o ninguna interfaz visual. Una aplicación instalada no equivale necesariamente a una instancia en ejecución.

### Servicio (`Service`)

Función software ejecutada y gestionada en segundo plano, normalmente sin requerir una ventana. Un daemon, un worker o un servicio de copias de seguridad pueden exponer un contrato SASP sin adoptar una interfaz gráfica.

### Proveedor (`Provider`)

Entidad lógica que **publica capacidades semánticas** a través de SASP. Puede ser una aplicación, un servicio o un adaptador delante de software existente. El proveedor conserva la lógica de su dominio; SASP no obliga a reescribirla.

### Instancia (`Instance`)

Ejemplar concreto y potencialmente efímero de un proveedor. Un mismo proveedor puede tener varias instancias activas. La identidad estable del proveedor y la identidad de su instancia son conceptos distintos.

### Endpoint

Punto de acceso al proveedor o a su adaptador. Puede ser un socket Unix, un canal mediado por broker u otra conexión compatible. Un endpoint **no es** la identidad confiable del servicio y puede cambiar sin que cambie el proveedor.

## 2. Contrato y datos

### Contrato semántico (`Semantic Contract`)

Descripción estructurada de **qué es el proveedor, qué expone y cómo puede interactuarse con él**. Incluye identidad, capacidades, esquemas de métodos, estado observable, eventos y, opcionalmente, contexto de interacción. Un contrato no sustituye permisos efectivos.

### Manifiesto (`Manifest`)

Documento de descripción de capacidades, pensado para cambiar menos frecuentemente que el estado en tiempo de ejecución. Puede incluir el identificador del proveedor, su tipo, los métodos y eventos publicados y los esquemas de datos correspondientes.

### Capacidad (`Capability`)

Función o clase de información declarada por el proveedor. Puede expresarse mediante un método, un estado observable o un tipo de evento. Una capacidad **anunciada** no es un permiso **concedido**.

### Método (`Method`)

Operación invocable con parámetros y resultados definidos. Un método puede observar información o solicitar una modificación. La categoría del efecto orienta al consumidor, pero la autorización debe verificarse en el momento de la operación.

### Estado (`State`)

Información actual sobre una instancia o recurso que el proveedor hace observable. Ejemplos: volumen, pista activa, estado de un backup o salud del servicio. El estado no debe confundirse con la mera disponibilidad del proceso.

### Instantánea (`Snapshot`)

Representación del estado en un punto de observación. Puede acompañarse de **revisión** o identificador comparable para detectar datos obsoletos. El modelo de consistencia de revisiones está por especificar.

### Evento (`Event`)

Notificación estructurada de que ha ocurrido un cambio o hecho relevante. La recepción de un evento no implica por sí sola que el consumidor posea el estado completo y actualizado.

### Suscripción (`Subscription`)

Relación por la que un consumidor observa un conjunto de eventos o cambios. Quedan por definir entrega, orden, retención, revocación, reconexión y resincronización.

### Contexto de interacción (`Interaction Context`)

Información opcional que ayuda a un cliente a interpretar, agrupar o presentar capacidades. **No es una ventana ni exige HTML**, y no debe convertirse en un canal de ejecución privilegiado.

## 3. Roles de comunicación

| Rol | Significado |
| --- | --- |
| **Client** | Consumidor de contratos, estado, eventos y métodos. Puede ser un agente, CLI o GUI. |
| **Registry** | Servicio opcional de localización e información de instancias. Encontrar un proveedor no autentica ni autoriza una operación. |
| **Broker** | Intermediario opcional que puede encaminar conexiones, mensajes o suscripciones. |
| **Bridge / Adapter** | Traducción entre SASP y otra interfaz o protocolo, por ejemplo MCP. |
| **Presenter / Shell** | Cliente que genera una experiencia humana; puede consumir contexto de interacción o disponer de UI propia. |
| **Security Authority** | Mecanismo externo que aplica identidad, políticas y controles efectivos de acceso. |

Un mismo programa puede cumplir varios roles. **SASP no presupone un servicio separado para cada rol.**

## 4. Identidades y autorizaciones

**Identidad declarada:** información como `provider.id` o el nombre de una instancia. Sirve para referencias semánticas, no como prueba criptográfica ni como autenticación del proceso.

**Identidad autenticada:** identidad establecida por el entorno o transporte mediante mecanismos verificables, por ejemplo credenciales del par para un socket Unix cuando proceda.

**Permiso / autorización:** decisión de una autoridad efectiva según identidad, operación y contexto. Un manifiesto, un modelo de lenguaje, el registro o un broker no pueden conceder privilegios por el solo hecho de anunciar capacidades.

**Efecto:** consecuencia potencial de una operación. Como guía de diseño pueden distinguirse consultas de operaciones modificadoras, pero la semántica de efectos, la idempotencia y las políticas deberán definirse y verificarse por separado.

## 5. Ciclos de vida independientes

- **Aplicación o servicio:** instalado, arrancando, ejecutándose, detenido, fallido, etc. Lo administra el sistema anfitrión.
- **Endpoint SASP:** inicializándose, disponible, desconectado o degradado. Su estado no equivale al del proceso.
- **Tarea:** trabajo solicitado, aceptado, ejecutándose, completado, denegado, fallido, cancelado o interrumpido. Su ejecución puede pertenecer al proveedor o a un Task Runtime separado.

Los valores anteriores son ejemplos; no constituyen enumeraciones normativas de estados.

## 6. Operaciones lógicas del modelo

Para describir las interacciones sin elegir prematuramente RPC, se utilizan estas operaciones conceptuales:

| Operación lógica | Función |
| --- | --- |
| **Discover** | Localizar proveedores o instancias accesibles. |
| **Describe** | Obtener el manifiesto y las capacidades anunciadas. |
| **Observe** | Consultar un estado o instantánea. |
| **Subscribe** | Recibir cambios observables según garantías definidas. |
| **Invoke** | Solicitar un método; su ejecución exige autorización aplicable. |
| **Resynchronize** | Recuperar información coherente tras una desconexión. |

Estos términos **no son nombres de métodos del protocolo**, sino parte del vocabulario arquitectónico.

## 7. Límites con tecnologías existentes

- **Linux y APIs nativas:** mantienen procesos, señales, syscalls, servicios, cgroups, aislamiento y permisos; SASP no los reemplaza.
- **MCP:** tiene herramientas, recursos, notificaciones y otros mecanismos que podrían cubrir parte del contrato. La investigación debe comparar reutilización con cualquier funcionalidad adicional.
- **MCP Apps:** ofrece una integración de UI interactiva dentro de hosts MCP compatibles; no es una obligación de presentación para SASP.
- **Task Runtime:** administra trabajo de larga duración cuando el sistema lo requiera; el contrato semántico puede exponerlo sin absorber la responsabilidad de ejecución.
- **Thanos OS:** posible implementación del modelo; sus nombres y mecanismos no son dependencias obligatorias de SASP.

## 8. Referencias

- [Arquitectura SASP](architecture.md)
- [Contrato mínimo experimental](../specification/draft/overview.md)
- [Decisión de nombre y alcance](../DECISION-0001-nombre-y-alcance.md)
- [Model Context Protocol](https://modelcontextprotocol.io/specification)
