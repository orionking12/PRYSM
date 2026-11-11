# PRYSM — Release Readiness / Preparación de publicación

> Checklist de auditoría. Su existencia **no equivale a una certificación** ni acredita que CI haya terminado correctamente.

## Evidencia verificable / Verifiable evidence
- [ ] CI de Rust en Linux y Windows completado en el commit de liberación.
- [ ] `cargo test --locked --test convergence -- --nocapture` reproducible en un repositorio Git temporal.
- [ ] Prueba manual de `switch -c`, `list --long`, `merge --dry-run` y `merge` sin pérdida de commits.
- [ ] Caso de conflicto real: confirmar que la rama destino y su último commit permanecen intactos.
- [ ] Prueba de `clean --all`: no eliminar worktrees con cambios no confirmados.
- [ ] Escaneo de dependencias y secretos realizado; hallazgos resueltos o documentados.
- [ ] Verificación de licencia MIT y terceros en los artefactos distribuidos.
- [ ] Verificar ejemplos en Windows PowerShell, Bash y/o shells que se indiquen como compatibles.

## Alcance y límites / Scope and limitations
PRYSM coordina worktrees y metadatos declarados de agentes; **no** verifica semánticamente la calidad del código generado por modelos. La advertencia de alcance compartido depende de las etiquetas de los agentes y es informativa; no sustituye a los conflictos reales que detecta Git. El merge requiere intervención del usuario cuando hay conflictos.

## Publicación progresiva / Progressive release
1. Mantener el repositorio abierto únicamente con documentación y código revisados.
2. Integrar CI tras revisión del pull request.
3. Crear un tag firmado o anotado `v0.1.1` **solo después** de verificar el commit de la release.
4. Adjuntar notas de versión: funciones, limitaciones, pruebas ejecutadas, SO probados y hashes de cualquier binario.
5. Adjuntar una captura/GIF de una ejecución real etiquetada como demostración, sin tokens ni rutas privadas.
6. Si se publica un binario, generarlo desde CI, identificar claramente los artefactos experimentales y documentar cómo reproducir el build.

## Puertas de salida / Release gates
- **Bloqueante:** datos sensibles en Git history, pruebas fallidas, pérdida silenciosa de cambios Git o ausencia de licencia clara.
- **Pendiente no bloqueante:** demo visual, badges, distribución binaria automatizada, publicación en crates.io.
- **Estado actual:** auditoría documental inicial; ejecución CI por verificar.

Autor: Jorge Humberto Davalos Gonzalez (ORION-VOX). Asistencia de ChatGPT en revisión técnica y documentación; sin coautoría jurídica.
