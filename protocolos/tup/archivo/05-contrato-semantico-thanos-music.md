# TUP — Contrato semántico de referencia: Thanos Music

> **Estado:** diseño de referencia TUP v0.1. Los ejemplos muestran la forma propuesta del contrato; no constituyen una especificación estable. El ejemplo MCP refleja el patrón básico de `tools/list` y `tools/call`; para su adaptación a revisiones actuales, consultar [interoperabilidad SASP–MCP](../../sasp/docs/interoperability-mcp.md).

## 1. Problema y objetivo

El agente necesita comprender qué operaciones ofrece una aplicación y qué estado expone, sin depender únicamente de capturas, píxeles o posiciones de controles.

**La aplicación o su desarrollador declara las capacidades. El modelo no las inventa.** El cliente del agente descubre el contrato, interpreta los argumentos admitidos y solicita operaciones. La aplicación o la autoridad correspondiente sigue siendo responsable de validarlas y autorizarlas.

El caso de referencia es un reproductor musical en Rust, **Thanos Music**.

## 2. Ejemplo MCP: herramientas anunciadas por una aplicación

Un servidor MCP integrado en Thanos Music puede responder a una solicitud de descubrimiento de herramientas:

~~~json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/list"
}
~~~

Respuesta ilustrativa:

~~~json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tools": [
      {
        "name": "player_get_state",
        "description": "Get playback status, track and volume",
        "inputSchema": {
          "type": "object",
          "properties": {},
          "additionalProperties": false
        },
        "_meta": {
          "ui": {
            "resourceUri": "ui://music/player.html"
          }
        }
      },
      {
        "name": "player_set_volume",
        "description": "Set player volume from 0 to 100",
        "inputSchema": {
          "type": "object",
          "properties": {
            "volume": {
              "type": "integer",
              "minimum": 0,
              "maximum": 100
            }
          },
          "required": ["volume"]
        }
      }
    ]
  }
}
~~~

Un agente puede reconocer la consulta de estado, el método de modificación de volumen, el tipo de parámetro requerido y su rango permitido.

El campo `_meta.ui.resourceUri` está destinado principalmente al host compatible con **MCP Apps**, que puede obtener una interfaz asociada. El modelo no necesita interpretar esa interfaz para usar las herramientas.

Solicitud para ajustar el volumen a 70:

~~~json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "player_set_volume",
    "arguments": {
      "volume": 70
    }
  }
}
~~~

La aplicación ejecuta la operación únicamente después de aplicar los controles de entrada y permisos correspondientes. MCP también dispone de recursos y otros mecanismos de observación.

## 3. Propuesta TUP v0.1: manifiesto de aplicación

En lugar de describir solamente herramientas individuales, TUP plantea un **contrato semántico agrupado por aplicación o servicio**.

~~~json
{
  "tup_version": "0.1-draft",

  "application": {
    "id": "org.thanos.music",
    "name": "Thanos Music",
    "kind": "desktop",
    "instance_id": "player-01"
  },

  "state": {
    "playback": "paused",
    "track": "Demo Track",
    "volume": 45,
    "revision": 7
  },

  "methods": [
    {
      "name": "play",
      "description": "Resume playback",
      "inputSchema": {
        "type": "object",
        "properties": {}
      }
    },
    {
      "name": "set_volume",
      "description": "Change playback volume",
      "inputSchema": {
        "type": "object",
        "properties": {
          "volume": {
            "type": "integer",
            "minimum": 0,
            "maximum": 100
          }
        },
        "required": ["volume"]
      }
    }
  ],

  "events": [
    {
      "name": "playback.changed",
      "description": "Playback state changed"
    },
    {
      "name": "volume.changed",
      "description": "Volume value changed"
    }
  ],

  "interface": {
    "title": "Music Player",
    "views": ["now_playing"],
    "primary_actions": [
      "play",
      "set_volume"
    ]
  }
}
~~~

### Cuatro dimensiones de comprensión

| Dimensión | Información comprendida por el agente |
| --- | --- |
| **Estado** | Reproducción pausada, pista cargada, volumen actual y revisión. |
| **Métodos** | Acciones disponibles, argumentos, restricciones y resultados esperados. |
| **Eventos** | Cambios observables aun sin interacción humana directa. |
| **Contexto de interfaz** | Organización y acciones principales de la aplicación, sin interpretar píxeles. |

El ejemplo combina descripción de capacidades y estado actual. La separación entre manifiesto estable y estado dinámico queda como evolución de diseño, actualmente explorada en [el contrato mínimo SASP](../../sasp/specification/draft/overview.md).

## 4. Descubrimiento por Capsuleer

~~~text
Aplicación Rust o daemon
    | publica estado y capacidades
    v
Registro de aplicaciones
    | identifica proveedor, instancia y disponibilidad
    v
Capsuleer
    | descubre, consulta, observa y solicita métodos
    v
Aplicación / Ship según la operación
    | valida autorización y ejecuta cuando corresponda
    v
Resultado y eventos observables por el agente y Thanos Shell
~~~

El registro permite descubrir una aplicación sin que exista una ventana abierta. **El descubrimiento no concede permisos.** La política de acceso corresponde al proveedor y, en Thanos OS, a Ship cuando se trate de operaciones protegidas.

Thanos Shell puede usar el mismo contrato para crear una representación humana, sin convertir esa presentación en la única interfaz de la aplicación.

## 5. Ejemplo TUP: daemon de copias de seguridad

~~~json
{
  "tup_version": "0.1-draft",
  "application": {
    "id": "org.thanos.backup",
    "name": "Backup Service",
    "kind": "daemon"
  },
  "state": {
    "status": "idle",
    "last_backup": "completed"
  },
  "methods": [
    {
      "name": "verify_backup",
      "description": "Verify a backup archive"
    }
  ],
  "events": [
    {
      "name": "backup.completed"
    },
    {
      "name": "backup.failed"
    }
  ],
  "interface": {
    "title": "Backup Service"
  }
}
~~~

El daemon expone semántica sin necesitar GTK, Qt, HTML, ventanas ni reemplazar su API nativa o su mecanismo de supervisión. Los esquemas formales de métodos y eventos siguen abiertos.

## 6. Comparación conceptual

| Dimensión | MCP | TUP propuesto |
| --- | --- | --- |
| Descubrimiento | Herramientas y recursos de servidor | Entidades de aplicación o servicio |
| Contrato | Definiciones individuales de herramientas y recursos | Descripción semántica agrupada |
| Estado | Recursos, resultados y convenciones | Modelo común de estado por diseñar |
| Eventos | Notificaciones y suscripciones | Eventos asociados a entidades por definir |
| UI | MCP Apps añade recursos visuales | Contexto de interfaz opcional |
| Ejecución | `tools/call` | Método del proveedor sujeto a autorización |

**MCP puede transportar representaciones semejantes:** el manifiesto TUP podría publicarse como un recurso MCP. La diferencia potencial está en **estandarizar la semántica de la aplicación o servicio**, no necesariamente en diseñar otro transporte JSON.

## 7. Evolución hacia SASP

Este contrato de TUP es un antecedente directo de **SASP — Semantic Application & Service Protocol**. SASP generaliza el objetivo a aplicaciones y servicios independientes de Thanos OS, y estudia separar:

- **Manifiesto:** identidad y capacidades declaradas.
- **Instantánea:** estado actual de una instancia.
- **Eventos:** cambios observables y su posible recuperación.
- **Contexto de interacción:** representación opcional, no un requisito de GUI.

Los ejemplos SASP [Music Player](../../sasp/examples/music-player/manifest.json) y [Backup Daemon](../../sasp/examples/backup-daemon/manifest.json) preservan los mismos conceptos con un contrato experimental más explícito.

**Criterio central:** el agente debe comprender la aplicación o daemon por **lo que declara sobre sí mismo**, sin depender de su interfaz gráfica y sin recibir privilegios directos del sistema.

## 8. Referencias

- [SASP — evolución conceptual](04-origen-hallazgos-evolucion.md)
- [SASP — arquitectura](../../sasp/docs/architecture.md)
- [SASP — comparación actual con MCP](../../sasp/docs/interoperability-mcp.md)
- [Model Context Protocol](https://modelcontextprotocol.io/specification)
- [MCP Apps](https://github.com/modelcontextprotocol/ext-apps)
