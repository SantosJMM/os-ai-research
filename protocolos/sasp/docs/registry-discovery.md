# SASP Registry y Discovery — Modelo de integración local

> **Estado:** arquitectura de investigación para OS-02 y OS-03. Los esquemas y operaciones ilustrativos de este documento no son una especificación aprobada.

## 1. Dos responsabilidades diferentes

**SASP Registry** mantiene una representación verificable de **software instalado** y **proveedores/instancias activos** en un sistema operativo.

**SASP Discovery** es la interfaz por la cual clientes autorizados **buscan, filtran y resuelven** proveedores e instancias. Su primera implementación candidata es un **servidor MCP del sistema operativo**, con herramientas y recursos ordinarios de MCP.

El registro **no reemplaza a MCP Registry** ni al gestor de paquetes. El descubrimiento **no reemplaza `server/discover`**: este último describe las capacidades de **un servidor MCP ya localizado**.

## 2. Fuentes de verdad

| Información | Fuente de verdad propuesta | Responsable |
| --- | --- | --- |
| Paquete instalado, versión, origen | Gestor de paquetes, instalador o registro confiable del host | Plano de instalación |
| Identidad semántica del proveedor y capacidades declaradas | Metadatos del paquete y contrato publicado | Proveedor + validador |
| Instancia activa | Supervisor, runtime confiable o proceso autenticado | Plano de ejecución |
| PID y handles del proceso | Kernel/supervisor; no texto libre del manifiesto | SO |
| Disponibilidad del endpoint MCP | Prueba de conectividad/estado, con vigencia limitada | Runtime/monitor de instancia |
| Autorización de lectura y conexión | Credenciales y políticas vigentes del host | Autoridad de seguridad |
| Estado funcional del servicio | MCP Resources/Tools del proveedor | Aplicación/servicio |

El catálogo de instalación y el directorio de ejecución pueden vivir en una base de datos o servicio común, pero **mantienen semánticas diferentes**: *instalado* no es *activo*, *proceso vivo* no es *endpoint MCP listo*, y *endpoint listo* no implica *cliente autorizado*.

### Reutilización de MCP Registry

Los metadatos **`server.json`** del MCP Registry merecen evaluación para distribución e instalación de proveedores MCP. **No contienen por sí mismos una identidad runtime autenticada**, una asociación fiable con el PID ni el directorio de instancias activas de un host.

SASP debe estudiar cómo incorporar metadatos existentes sin exigir otro manifiesto de distribución para aplicaciones que ya publican MCP.

## 3. Modelo local candidato

**El registro de instalaciones** contiene registros independientes de instancias, por ejemplo:

```json
{
  "provider_id": "org.example.music",
  "package_id": "org.example.music",
  "package_version": "0.1.0",
  "installed": true,
  "launch_profile": "desktop",
  "declared_capabilities": ["music.playback", "player.set_volume"]
}
```

**El registro de instancias** puede reflejar el estado operativo:

```json
{
  "instance_id": "music-01",
  "provider_id": "org.example.music",
  "generation": "boot-7-instance-3",
  "process": {
    "pid": 4250,
    "verified_by": "supervisor"
  },
  "endpoint": {
    "kind": "unix",
    "availability": "ready",
    "connection_ref": "music-handle-01"
  }
}
```

Estos ejemplos son **snapshots informativos de un registro de prueba**, no respuestas directamente confiables para un cliente ni campos normativos. El PID solo resulta útil cuando está asociado a una identidad de proceso verificada y al ciclo de vida correcto; no basta con guardarlo.

### Identidades que no se deben confundir

- **Paquete:** artefacto instalado y administrado por el gestor del SO.
- **Proveedor:** entidad lógica que publica un conjunto de capacidades.
- **Instancia:** proceso o ejecución concreta del proveedor.
- **Endpoint:** canal de comunicación que puede desaparecer o reconectarse.
- **Cliente:** quien consulta o invoca; su identidad debe comprobarse independientemente.
- **Referencia resuelta:** handle lógico que permite localizar una ruta de conexión sujeto a nueva autorización.

Un proveedor puede tener varias instancias. Una instancia podría tener más de un endpoint. Un proceso puede seguir vivo con su endpoint caído.

## 4. Alta, cambio y baja de instancias

### Instalación

1. El gestor de paquetes instala software y registra metadatos de origen.
2. El sistema valida formato, autoría según la política local y asociación paquete/proveedor.
3. El catálogo publica la existencia del proveedor como **instalado**, aunque no haya proceso.
4. Si las capacidades se derivan de un manifiesto incluido en el paquete, se marcan como **declaradas**, no como capacidades runtime ya comprobadas.

### Inicio

1. El supervisor inicia el proceso con identidad y perfil de seguridad específicos.
2. El registro vincula el proceso autenticado con el paquete/proveedor permitido.
3. El endpoint se publica o se descubre según el transporte elegido.
4. La instancia pasa a **lista para conexión** solo tras verificar su disponibilidad y compatibilidad según la política.
5. Discovery puede anunciarla a clientes **autorizados**.

### Fallos y desconexión

- La caída de un endpoint no borra automáticamente la instalación.
- El cierre del proceso invalida o marca como obsoleta su entrada de instancia.
- Si el supervisor reinicia el proceso, debe renovarse su generación/identidad de ejecución.
- El registro se reconcilia con el supervisor y los endpoints; no depende únicamente de mensajes de despedida o heartbeats.
- Un PID reciclado **no** recupera la identidad de una instancia anterior.
- Los clientes que ya resolvieron una instancia deben revalidar referencias y permisos cuando corresponda.

### Desinstalación

El gestor invalida la asociación de instalación y coordina la baja de instancias dependientes según las políticas del SO. No debe desaparecer el catálogo sin invalidar las referencias runtime correspondientes.

## 5. Discovery expuesto mediante MCP

La implementación candidata utiliza **Tools y Resources MCP** de un servidor del host.

| Consulta | Operación de dominio candidata | Alternativa con MCP |
| --- | --- | --- |
| Buscar por capacidad | `applications.search` | Herramienta publicada por `tools/list` |
| Obtener proveedor | `applications.get` | Herramienta o recurso de catálogo |
| Resolver instancia accesible | `applications.resolve` | Herramienta con salida tipada |
| Consultar disponibilidad | `applications.status` | Recurso con notificación de actualización |
| Observar cambios del catálogo | Recurso de registro | `subscriptions/listen` si el servidor lo implementa |

Los nombres anteriores son **nombres de herramientas SASP propuestas**, no RPC nuevos del núcleo MCP.

Para detalles de los mensajes JSON y casos de prueba, véase [Discovery como servicio MCP](../experiments/os-integration/discovery-mcp.md).

### Flujo de búsqueda por capacidad

```text
Usuario: «Busca un reproductor de música disponible»
              |
        Capsuleer o Shell
              |
         MCP tools/call
       applications.search
              |
         SASP Discovery
              |
        [filtro por permisos]
              |
          SASP Registry
      instalación + instancias
              |
  resultado: proveedor + estado + handle
              |
     resolución y conexión MCP
      con autorización renovada
              |
           Music Player
```

Un resultado puede mostrar un proveedor **instalado pero sin instancia**. **Discovery no debe iniciar el programa de forma implícita**: iniciar requiere una operación explícita del gestor/supervisor y autorización independiente.

### Resolución y conexión

La resolución puede devolver una **referencia opaca**, no necesariamente una ruta física a un socket. La implementación elige conexión directa o mediada conforme a los permisos y al aislamiento.

En Thanos OS, Capsuleer no debe recibir por resolución un endpoint que eluda las restricciones efectivas de Ship.

## 6. Seguridad

**Registro y descubrimiento son superficies de ataque**, no simples índices de nombres.

- La publicación de una entrada requiere identidad vinculada al proceso o al instalador autorizado.
- Los campos `provider_id`, `pid` y `instance_id` recibidos de clientes se tratan como **declaraciones no autenticadas** hasta verificarse.
- Los resultados de Discovery se filtran por sujeto y política; no todos los clientes ven todas las capacidades o instancias.
- Las referencias a endpoints deben tener control de alcance, vigencia y revocación.
- Resolver una referencia no concede permisos futuros: el proveedor sigue autorizando cada método protegido.
- El registro no almacena secretos de sesión en resultados visibles por agentes.
- Un broker, si se introduce, debe demostrar aislamiento efectivo y evitar suplantación de proveedor y escalada de privilegios.

## 7. Qué tenemos que experimentar

| ID | Escenario | Aceptación |
| --- | --- | --- |
| REG-01 | Aplicación instalada sin proceso | Visible como instalada y no activa |
| REG-02 | Una app y dos instancias | Identidades y generaciones inequívocas |
| REG-03 | Proceso muerto o PID reutilizado | Entrada desactivada, sin autenticación accidental |
| REG-04 | Endpoint desconectado, proceso vivo | Estado de conectividad separado del proceso |
| DIS-01 | Búsqueda por capacidad con cliente autorizado | Solo proveedores permitidos |
| DIS-02 | Búsqueda con cliente restringido | Resultados filtrados, sin rutas sensibles |
| DIS-03 | Resolve de handle revocado | Conexión denegada |
| DIS-04 | Proveedor instalado pero inactivo | No se inicia por efecto de búsqueda |
| DIS-05 | Reinicio de instancia | El cliente detecta la nueva generación |

**Secuencia recomendada:** probar primero el modelo de datos local como componente Rust sin root, después el servicio Discovery basado en MCP; solo entonces comparar socket directo y broker.

## 8. Relaciones

- [SASP — integración del sistema operativo](os-integration.md)
- [Arquitectura de SASP](architecture.md)
- [Plan de experimentos OS-01 a OS-06](../experiments/os-integration/README.md)
- [MCP Registry — repositorio](https://github.com/modelcontextprotocol/registry)
- [MCP 2026-07-28 — protocolo](https://modelcontextprotocol.io/specification/2026-07-28)
