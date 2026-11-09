# 💎 PRYSM CLI

[Español](#español) · [English](#english)

## Español

**Dos agentes, un repo. PRYSM muestra dónde se cruzan sus tareas antes de integrar.**

PRYSM es una CLI local que organiza el trabajo paralelo de agentes en Git. Cada agente usa su propia rama y carpeta de trabajo (worktree). Tú decides cuándo integrar.

- **Visibilidad.** Consulta qué agente ocupa cada worktree, qué pretende hacer, qué áreas declaró y qué nivel de riesgo indicó.
- **Alcance compartido.** Antes de integrar, PRYSM avisa si dos worktrees comparten una etiqueta declarada, como `api`. La advertencia es informativa. Git gestiona los conflictos de archivos.
- **Integración.** PRYSM aplica los cambios en el worktree de la rama de destino. Si hay conflictos, indica qué archivos quedaron pendientes, en qué carpeta resolverlos y qué commit conserva la rama de destino.

### Instalación

Necesitas Git en el `PATH` y una versión reciente de Rust estable con Cargo. Desde la raíz del código fuente:

```sh
cargo install --locked --path .
```

### Inicio rápido

Desde un repositorio Git con al menos un commit en `main`:

```sh
prysm switch -c auth --agent Claude --intent "Add authentication middleware" --scope api,auth --risk med
prysm switch -c api --agent Codex --intent "Add cursor pagination" --scope api --risk low
prysm list --long
```

Abre las rutas que imprime `switch` y ejecuta cada agente en su propia carpeta. PRYSM prepara los worktrees y registra las tareas. Cada agente debe guardar sus cambios en un commit.

Desde el worktree `auth`, revisa el alcance compartido y luego integra la rama:

```sh
prysm merge main --dry-run
prysm merge main
```

El dry-run compara las etiquetas declaradas y deja `main` intacta. La integración real requiere que `main` esté abierta en otro worktree y que ambas carpetas estén limpias. Repite el proceso desde `api` y ejecuta las integraciones hacia el mismo destino una a la vez.

### Comandos

| Comando | Uso |
| --- | --- |
| `prysm switch <rama>` | Abre o crea un worktree para una rama existente. Con `-c` también crea la rama. |
| `prysm list --long` | Muestra los worktrees y los datos de sus tareas. Usa `--json` para obtener salida estructurada. |
| `prysm merge <destino>` | Integra los cambios confirmados desde el worktree actual. Agrupa los cambios en un commit por defecto. Con `--squash=false` conserva el historial de la rama de origen. |
| `prysm clean --all` | Quita worktrees elegibles y limpios cuyo último commit esté en el historial de la rama principal. |

PRYSM guarda los metadatos en `<git-dir>/prysm-intent.json` de cada worktree y también lee el formato anterior en `.prysm/intent.json`. Los valores válidos para `--risk` son `low`, `med` y `high`.

Tras un squash, el commit original de la rama normalmente queda fuera del historial del destino. En ese caso, `clean --all` conserva el worktree y su alcance puede seguir apareciendo en las advertencias.

### Pruebas

```sh
cargo test --locked --test convergence -- --nocapture
```

Las pruebas usan un repositorio temporal. Cubren la advertencia por alcance compartido, la integración de dos ramas y un conflicto en la misma línea que deja intacto el último commit del destino. La demo de PowerShell reproduce el flujo con cambios y commits generados por el script bajo las etiquetas Claude y Codex.

Consulta la [referencia de la CLI](docs/UX.md) para más detalles.

## English

**Two agents, one repo. PRYSM shows where their tasks overlap before you merge.**

PRYSM is a local CLI for organizing parallel agent work in Git. Each agent gets its own branch and worktree. You decide when to merge.

- **Visibility.** See the agent, intent, declared scopes, and risk hint for each worktree.
- **Shared scope.** Before merging, PRYSM warns when two worktrees share a declared label such as `api`. The warning is advisory. Git handles file conflicts.
- **Integration.** PRYSM applies changes in the target branch's worktree. If conflicts occur, it reports the affected files, where to resolve them, and the commit the target branch still points to.

### Install

You need Git on your `PATH` and a recent stable Rust toolchain with Cargo. From the source root:

```sh
cargo install --locked --path .
```

### Quick start

From a Git repository with at least one commit on `main`:

```sh
prysm switch -c auth --agent Claude --intent "Add authentication middleware" --scope api,auth --risk med
prysm switch -c api --agent Codex --intent "Add cursor pagination" --scope api --risk low
prysm list --long
```

Open the paths printed by `switch` and run each agent in its own directory. PRYSM prepares the worktrees and records their tasks. Each agent must commit its changes.

From the `auth` worktree, check for shared scopes and then merge the branch:

```sh
prysm merge main --dry-run
prysm merge main
```

The dry run compares declared labels and leaves `main` untouched. The merge requires `main` to be checked out in a separate worktree and both worktrees to be clean. Repeat from `api` and run merges into the same target one at a time.

### Commands

| Command | Use |
| --- | --- |
| `prysm switch <branch>` | Opens or creates a worktree for an existing branch. Add `-c` to create the branch as well. |
| `prysm list --long` | Shows worktrees and task metadata. Use `--json` for structured output. |
| `prysm merge <target>` | Integrates committed changes from the current worktree. Squashes changes into one commit by default. Use `--squash=false` to retain the source branch's history. |
| `prysm clean --all` | Removes eligible, clean worktrees whose tip is reachable from the primary worktree's branch. |

PRYSM stores metadata in each worktree's `<git-dir>/prysm-intent.json` and still reads legacy `.prysm/intent.json` files. The valid `--risk` values are `low`, `med`, and `high`.

After a squash, the source tip usually remains outside the target's history. In that case, `clean --all` retains the worktree and its scope may continue to trigger warnings.

### Tests

```sh
cargo test --locked --test convergence -- --nocapture
```

The tests use a temporary repository. They cover a shared-scope warning, integration of two branches, and a same-line conflict that preserves the target's last commit. The PowerShell demo replays the workflow with changes and commits generated by the script under the Claude and Codex labels.

See the [CLI reference](docs/UX.md) for details.

## Autoría y licencia · Authorship and license

**Jorge Humberto Davalos Gonzalez** (ORION-VOX / VOX-114)

- Web · Website — [www.orzworknetai.com](https://www.orzworknetai.com)
- Contacto · Contact — [orionchinnar@gmail.com](mailto:orionchinnar@gmail.com)

Asistencia de ChatGPT en análisis técnico, revisión y documentación.  
ChatGPT assisted with technical analysis, review, and documentation.

[Licencia MIT · MIT License](LICENSE) · [Historial de cambios · Changelog](CHANGELOG.md)
