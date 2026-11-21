# TUP histórico — Ejemplos JSON y JSON-RPC

> **Reconstrucción técnica, no transcripción textual.** Los siguientes JSON son **ejemplos ilustrativos** compuestos a partir de los conceptos que discutimos el 27 y 28 de octubre de 2026. No se afirma que la versión original utilizara exactamente estos nombres de campos, payloads o métodos. Se preservan como conocidos los conceptos **Task / View / App**, JSON-RPC 2.0, **task.progress**, **task.completed**, **view.publish**, y las acciones **project.open**, **project.explore**, **project.test.run**.

## 1. Transporte de la propuesta inicial

La aproximación discutida fue **JSON-RPC 2.0 sobre socket Unix**, con delimitación de longitud por mensaje.

~~~text
[longitud del mensaje][objeto JSON-RPC UTF-8]
[longitud del mensaje][objeto JSON-RPC UTF-8]
...
~~~

**Todavía no había una decisión normativa verificable sobre el tamaño, endianess ni interpretación exacta del prefijo**, ni sobre usar un socket directo frente a un broker.

## 2. Ejemplo de manifiesto de aplicación (AppManifest)

Ejemplo reconstruido de cómo una app podría presentar su identidad, estado, acciones, eventos y contexto de interfaz:

~~~json
{
  "protocol": "tup/0.1-draft",
  "app": {
    "id": "org.thanos.project",
    "name": "Project Workspace",
    "instance_id": "workspace-01",
    "kind": "desktop",
    "lifecycle": "running",
    "interface_status": "ready"
  },
  "capabilities": {
    "actions": [
      {
        "name": "project.open",
        "description": "Open a local project workspace",
        "inputSchema": {
          "type": "object",
          "properties": {
            "path": { "type": "string" }
          },
          "required": ["path"]
        }
      },
      {
        "name": "project.explore",
        "description": "Inspect project files and metadata",
        "inputSchema": {
          "type": "object",
          "properties": {
            "project_id": { "type": "string" }
          },
          "required": ["project_id"]
        }
      },
      {
        "name": "project.test.run",
        "description": "Run tests through an authorized worker",
        "inputSchema": {
          "type": "object",
          "properties": {
            "project_id": { "type": "string" },
            "suite": { "type": "string" }
          },
          "required": ["project_id"]
        }
      }
    ],
    "events": ["task.progress", "task.completed", "view.publish"]
  }
}
~~~

**Precaución:** que una aplicación publique **project.test.run** no significa que pueda ejecutar comandos arbitrarios ni que exista permiso automático; la ejecución propuesta se somete a políticas y a un worker aislado.

## 3. Descubrimiento y registro de app

El siguiente método **app.register** es ilustrativo; lo conocido de la discusión es que se planteó un registro de aplicaciones e instancias.

~~~json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "app.register",
  "params": {
    "app_id": "org.thanos.project",
    "instance_id": "workspace-01",
    "protocol": "tup/0.1-draft",
    "endpoint": "unix:///run/thanos/apps/project.sock"
  }
}
~~~

~~~json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "accepted": true,
    "instance_id": "workspace-01"
  }
}
~~~

El registro debería verificar identidad de proceso/cliente mediante mecanismos del sistema. Un identificador declarado en JSON no es prueba de identidad por sí solo.

## 4. Estado de una aplicación y revisión

Ejemplo de snapshot ilustrativo. La idea de revisiones se discutió para evitar que un agente actúe sobre información obsoleta.

~~~json
{
  "app_id": "org.thanos.project",
  "instance_id": "workspace-01",
  "revision": 12,
  "state": {
    "current_project": "/workspace/demo",
    "status": "ready",
    "tests_running": false
  }
}
~~~

Un cliente podría recibir un nuevo snapshot tras reconectar; **cómo se solicita ese snapshot era una decisión pendiente**.

## 5. Publicación de una View declarativa

**view.publish** es un nombre de evento conocido de la propuesta inicial. El esquema concreto siguiente es ilustrativo.

~~~json
{
  "jsonrpc": "2.0",
  "method": "view.publish",
  "params": {
    "view_id": "project.overview",
    "app_instance_id": "workspace-01",
    "revision": 12,
    "title": "Project overview",
    "tree": {
      "type": "column",
      "children": [
        {
          "type": "text",
          "id": "project.name",
          "value": "demo"
        },
        {
          "type": "button",
          "id": "project.run_tests",
          "label": "Run tests",
          "action": "project.test.run"
        }
      ]
    }
  }
}
~~~

El nodo **button** anuncia una acción registrada. **No ejecuta por sí solo** comandos del sistema ni debe llevar scripts arbitrarios capaces de saltarse la política.

## 6. Solicitud de una acción tipada

Ejemplo reconstruido de **ActionRequest**. La operación **project.test.run** sí se discutió; el método de envoltura **action.request** se utiliza aquí solo para ilustrar la estructura.

~~~json
{
  "jsonrpc": "2.0",
  "id": 21,
  "method": "action.request",
  "params": {
    "action": "project.test.run",
    "args": {
      "project_id": "demo",
      "suite": "unit"
    },
    "origin": {
      "app_instance_id": "workspace-01",
      "view_id": "project.overview",
      "view_revision": 12,
      "session_id": "main"
    },
    "request_id": "test-0001"
  }
}
~~~

**La validación no puede confiar exclusivamente en los valores de origin**, que son datos declarados por el cliente. Debe vincularse a una identidad autenticada por la conexión y a una política de permisos.

## 7. La acción crea una tarea

**task.create** fue parte de la API estudiada para el Task Runtime (no necesariamente un método TUP normativo).

~~~json
{
  "jsonrpc": "2.0",
  "id": 22,
  "method": "task.create",
  "params": {
    "kind": "project.test.run",
    "title": "Run unit tests",
    "request_id": "test-0001",
    "input": {
      "project_id": "demo",
      "suite": "unit"
    }
  }
}
~~~

~~~json
{
  "jsonrpc": "2.0",
  "id": 22,
  "result": {
    "task_id": "task-0042",
    "state": "queued"
  }
}
~~~

El estado **queued** es una elección ilustrativa; el contrato exacto de estados quedó pendiente.

## 8. Suscripción a tareas

En el estudio del Task Runtime aparecieron los métodos **task.get**, **task.list**, **task.cancel**, **task.subscribe** y **task.complete**, además de **task.create**. Aquí se da un ejemplo de **task.subscribe**:

~~~json
{
  "jsonrpc": "2.0",
  "id": 23,
  "method": "task.subscribe",
  "params": {
    "task_id": "task-0042"
  }
}
~~~

Una suscripción necesita definir semántica de desconexión, duplicados, reinicio, orden y recuperación: no basta con tener un método llamado *subscribe*.

## 9. Evento task.progress

~~~json
{
  "jsonrpc": "2.0",
  "method": "task.progress",
  "params": {
    "task_id": "task-0042",
    "sequence": 3,
    "state": "running",
    "percent": 60,
    "message": "6 of 10 checks finished"
  }
}
~~~

**task.progress** es un nombre históricamente discutido; **sequence**, **percent** y el resto de campos son decisiones de ejemplo, no una afirmación de esquema literal.

## 10. Evento terminal task.completed

~~~json
{
  "jsonrpc": "2.0",
  "method": "task.completed",
  "params": {
    "task_id": "task-0042",
    "state": "completed",
    "result": {
      "passed": 10,
      "failed": 0
    }
  }
}
~~~

Las discusiones identificaron estados finales como *completed*, *failed*, *denied*, *cancelled* e *interrupted*. El esquema del evento terminal y su persistencia no quedaron fijados aquí.

## 11. Cancelación

~~~json
{
  "jsonrpc": "2.0",
  "id": 24,
  "method": "task.cancel",
  "params": {
    "task_id": "task-0042",
    "reason": "user_requested"
  }
}
~~~

Solicitar cancelación no equivale a garantizar que un efecto ya producido se deshaga. El ejecutor del trabajo debe tener un comportamiento explícito de cancelación.

## 12. Denegación por política

Una respuesta ilustrativa de la autoridad para una acción que no está permitida:

~~~json
{
  "jsonrpc": "2.0",
  "id": 21,
  "error": {
    "code": -32003,
    "message": "Action denied by policy",
    "data": {
      "action": "project.test.run",
      "reason": "approval_required",
      "audit_ref": "atlas-demo-17"
    }
  }
}
~~~

El código de error **-32003** y los campos son **ilustrativos**. El punto histórico es que la acción propuesta por la IA **no** elude las políticas de **Ship**, y que una denegación puede producir evidencia de **Atlas**.

## 13. Un daemon sin interfaz gráfica

La discusión sobre aplicaciones y servicios avanzó hacia representar también software sin ventanas. Ejemplo ilustrativo de un servicio de backups:

~~~json
{
  "app": {
    "id": "org.thanos.backup",
    "name": "Backup Service",
    "kind": "daemon",
    "instance_id": "backup-01"
  },
  "state": {
    "status": "idle",
    "last_result": "completed"
  },
  "methods": ["backup.verify"],
  "events": ["backup.completed", "backup.failed"],
  "interface": null
}
~~~

Este caso demuestra que el modelo semántico no necesita reemplazar señales, syscalls, supervisores, cgroups ni protocolos nativos del servicio.

## 14. Qué faltaba para que fuera una especificación

Para convertir estos ejemplos en un protocolo implementable hace falta definir tipos de datos, códigos de error, negociación de versiones, transporte y delimitación, identidad del emisor, scopes, autorización, garantías de orden y entrega, almacenamiento, recuperación, interoperabilidad y pruebas de conformidad.

Por eso este archivo se conserva como **archivo de diseño** y no como **especificación TUP v0.1 publicada**.
