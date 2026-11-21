# SASP Discovery como servicio MCP del sistema operativo

> **Estado:** interfaz candidata para el experimento OS-03. Utiliza métodos **MCP 2026-07-28** existentes; las herramientas `applications.search` y `applications.resolve` son propuestas de dominio del SO, no métodos del núcleo MCP.

## Propósito

Un agente o una interfaz humana necesita descubrir **aplicaciones instaladas o instancias activas**, incluso cuando no tienen ventanas abiertas. No debe mantener de forma manual una lista de sockets, rutas ni procesos.

La propuesta usa un **servidor MCP del SO** que consulta un **registro local verificado**. `server/discover` sirve para conocer las capacidades de ese servidor MCP ya localizado; `applications.search` consulta el catálogo del SO.

```text
Capsuleer / Shell (MCP Client)
            |
      tools/list y tools/call
            |
      SASP Discovery (MCP Server)
            |
       consulta autorizada
            |
   SASP Registry (catálogo local)
     /                     \
instalaciones        instancias activas
     \                     /
       referencia de destino
            |
     Connection / Policy
            |
     app o daemon proveedor
```

## Herramienta candidata: búsqueda de aplicaciones

Elemento individual que podría aparecer en el resultado de `tools/list`:

```json
{
  "name": "applications.search",
  "description": "Find installed or active applications by semantic capability",
  "inputSchema": {
    "type": "object",
    "properties": {
      "capability": {
        "type": "string",
        "description": "Semantic capability to search for"
      },
      "active_only": {
        "type": "boolean",
        "default": false
      }
    },
    "required": ["capability"],
    "additionalProperties": false
  }
}
```

**Ejemplo de solicitud MCP 2026-07-28**, con metadatos por solicitud:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "applications.search",
    "arguments": {
      "capability": "music.playback",
      "active_only": false
    },
    "_meta": {
      "io.modelcontextprotocol/protocolVersion": "2026-07-28",
      "io.modelcontextprotocol/clientInfo": {
        "name": "ExampleShell",
        "version": "0.1.0"
      },
      "io.modelcontextprotocol/clientCapabilities": {}
    }
  }
}
```

Respuesta ilustrativa con un resultado tipado:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "1 available music provider"
      }
    ],
    "structuredContent": {
      "applications": [
        {
          "provider_id": "org.example.music",
          "name": "Example Music",
          "installed": true,
          "instance_id": "music-01",
          "availability": "ready",
          "target_ref": "ref_music_01"
        }
      ]
    },
    "isError": false
  }
}
```

`target_ref` es un **handle de referencia**, no una prueba de autorización ni un token con privilegios implícitos. La operación que lo utilice debe comprobar nuevamente identidad y permisos. No es necesario que Capsuleer conozca el PID ni la ruta física del socket.

## Herramienta candidata: resolver una referencia

`applications.resolve` podría determinar si `ref_music_01` sigue siendo válido, a qué versión corresponde y qué ruta de comunicación está **autorizada para ese cliente**. La respuesta puede permitir una conexión mediada sin revelar un socket directo.

Una aplicación inactiva podría aparecer como instalada y sin instancia. Iniciarla sería **otra operación explícita**, autorizada por el gestor de servicios correspondiente; Discovery no debe arrancarla como efecto oculto de una búsqueda.

## Relación con el registro

El registro local conserva **dos planos de información**:

| Plano | Ejemplo | Fuente de confianza |
| --- | --- | --- |
| Instalación | Paquete, proveedor, versión, mecanismo de inicio | Gestor de paquetes y política del SO |
| Ejecución | Instancia, disponibilidad, endpoint, identidad real | Supervisor, credenciales verificadas del proceso y canal |

El formato `server.json` del MCP Registry es una **fuente candidata de metadatos de instalación**, pero no reemplaza la verificación de instancias activas.

El [resultado JSON de ejemplo](../../examples/os-integration/discovery.search.result.json) corresponde a un índice semántico experimental `music.playback`. Esa etiqueta no es una herramienta MCP por sí sola; Registry/Discovery deben relacionarla con capacidades declaradas como `player.play` y `player.set_volume`.

## Casos de prueba OS-03

1. **Instalado sin ejecutar:** búsqueda devuelve `installed: true`, sin `instance_id` ni endpoint activo.
2. **Instancia activa:** búsqueda devuelve disponibilidad y handle de acceso.
3. **Dos instancias:** ambas pueden distinguirse y resolverse sin colisión.
4. **Dos perfiles de cliente:** Discovery devuelve solamente proveedores accesibles para cada uno.
5. **Conexión revocada:** un handle anteriormente obtenido no permite saltarse una denegación posterior.
6. **Caída del proveedor:** la información deja de considerarse `ready` y una resolución posterior detecta la indisponibilidad.
7. **Suplantación:** un registro con `provider_id` o `pid` autodeclarados no supera las verificaciones de identidad.

## Preguntas abiertas

- ¿Debe Discovery ser un servicio independiente, una interfaz del Registry o ambos?
- ¿Cuándo se renuevan los handles y qué información puede cachear un cliente?
- ¿Quién tiene autoridad para registrar instalaciones e instancias?
- ¿Cuándo se ofrece conexión directa, mediada o lanzamiento controlado?
- ¿Cómo se concilian las consultas por capacidad con el esquema de Tools/Resources del proveedor?
- ¿Cómo se representa el estado de aplicaciones de escritorio nativas, incluidos procesos sin ventana?

**No se necesita introducir un método JSON-RPC nuevo.** La novedad a investigar es **la integración del registro y ciclo de vida del SO con las herramientas y recursos MCP existentes**.

## Referencias

- [Integración SASP con sistemas operativos](../../docs/os-integration.md)
- [Plan experimental OS-01 a OS-06](README.md)
- [MCP 2026-07-28 — Tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools)
- [MCP 2026-07-28 — server/discover](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/server/discover.mdx)
- [MCP Registry — formato server.json](https://github.com/modelcontextprotocol/registry/blob/main/docs/reference/server-json/generic-server-json.md)
