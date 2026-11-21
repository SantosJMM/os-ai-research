# Gestión de OS Research en GitHub

## Flujo de estudio

1. **Pendiente**: pregunta registrada, sin trabajo iniciado.
2. **Estudiando**: fundamentos y fuentes primarias.
3. **Experimentando**: código, mediciones y comprobaciones reproducibles.
4. **Documentando**: resultados, límites y referencias.
5. **Completado**: evidencia revisada, publicada y vinculada al Issue.

Usar las etiquetas `estado:pendiente`, `estado:estudiando`, `estado:experimentando`, `estado:documentando` y `estado:completado` como convención. Para cambiar el estado, quitar la anterior.

## Etiquetas por área

- `area:fundamentos`, `area:ipc`, `area:seguridad`, `area:kernel-rust`, `area:comparativas`, `area:thanos-os`.
- Tipos: `tipo:estudio`, `tipo:experimento`, `tipo:investigacion`.

## GitHub Projects — tablero previsto

Proyecto creado: [OS Research — GitHub Project #4](https://github.com/users/SantosJMM/projects/4/views/1). Actualmente aparece con el nombre provisional de GitHub y sin elementos. Renombrarlo **OS Research — Learning & Experiments**, agregar los Issues #1 a #9 y configurar estados Pendiente, Estudiando, Experimentando, Documentando y Completado.

Vistas propuestas:
- **Tablero**: agrupar por Estado.
- **Mapa de estudio**: tabla por Área, Tipo, Issue y evidencia.
- **Investigaciones activas**: filtro de estados diferentes a Completado y Pendiente.

GitHub Projects usa su propio campo Estado; las etiquetas del repositorio no se sincronizan automáticamente con ese campo sin automatización explícita. Evitar duplicidad: una vez activo el proyecto, elegir el campo Estado de Projects como fuente principal.

## Cómo registrar resultados

Cada Issue debe enlazar un directorio en el repositorio con:
- pregunta e hipótesis falsable;
- entorno, versiones, comandos y procedimiento;
- salidas observadas;
- fuentes y límites de las conclusiones;
- resultados y cuestiones abiertas.

Un Issue completado sin evidencia no equivale a aprendizaje verificado.

## Separación de Thanos OS

No editar Thanos OS automáticamente a raíz de resultados del laboratorio. Elaborar propuestas y registrarlas en el repositorio de Thanos OS cuando proceda.
