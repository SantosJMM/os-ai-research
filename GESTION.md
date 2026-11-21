# Gestión de OS AI Research en GitHub

## Flujo de estudio

1. **Pendiente**: pregunta registrada, sin trabajo iniciado.
2. **Estudiando**: fundamentos y fuentes primarias.
3. **Experimentando**: código, mediciones y comprobaciones reproducibles.
4. **Documentando**: resultados, límites y referencias.
5. **Completado**: evidencia revisada, publicada y vinculada al Issue.

Usar las etiquetas `estado:pendiente`, `estado:estudiando`, `estado:experimentando`, `estado:documentando` y `estado:completado` como convención. Para cambiar el estado, quitar la anterior.

## Etiquetas por área

- `area:fundamentos`, `area:ipc`, `area:seguridad`, `area:kernel-rust`, `area:comparativas`, `area:agentes-ai`, `area:thanos-os`.
- Tipos: `tipo:estudio`, `tipo:experimento`, `tipo:investigacion`.

## GitHub Projects — tablero previsto

[OS AI Research — Learning & Experiments (GitHub Project #4)](https://github.com/users/SantosJMM/projects/4/views/1). El tablero fue configurado en una sesión separada; el cambio de nombre visible en GitHub Projects está pendiente de verificación. Los Issues #1 a #9 son las unidades originales; el [Issue #10](https://github.com/SantosJMM/os-ai-research/issues/10) añadió Arquitectura de computadores y los Issues [#11](https://github.com/SantosJMM/os-ai-research/issues/11) y [#12](https://github.com/SantosJMM/os-ai-research/issues/12) incorporaron agentes de IA, y el [Issue #13](https://github.com/SantosJMM/os-ai-research/issues/13) formalizó la línea SASP (antes TUP). Verificar que los cuatro Issues añadidos estén incluidos en Projects.

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
