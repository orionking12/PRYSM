# Changelog · Historial de cambios

## v0.1.1

### Español

- `prysm merge` indica qué archivos quedaron en conflicto, en qué worktree resolverlos y qué commit conserva la rama de destino.
- La prueba de regresión reproduce un conflicto en la misma línea. Verifica el error, el diagnóstico y que el commit de destino permanezca intacto.
- El README bilingüe documenta la instalación, el flujo de integración y la licencia MIT.
- Limitación conocida. Después de un squash, `clean --all` puede conservar el worktree de origen y su alcance puede seguir apareciendo en las advertencias.

### English

- `prysm merge` reports conflicted files, the worktree where they must be resolved, and the commit the target branch still points to.
- The regression test reproduces a same-line conflict and verifies the failure, the diagnostics, and the unchanged target commit.
- The bilingual README covers installation, the merge workflow, and the MIT license.
- Known limitation. After a squash, `clean --all` may retain the source worktree and its scopes may continue to trigger warnings.
