# PRYSM — Verificación local sin GitHub Actions

El verificador ejecuta pruebas del proyecto desde Windows, Linux o macOS, sin depender de los runners ni del sistema de facturación de GitHub.

## Requisitos
- Python 3.10+ (solo biblioteca estándar).
- Git.
- Rust/Cargo.
- Dependencias disponibles para Cargo (o en caché usando \`--offline\`).

## Uso desde la raíz del repositorio

**Windows PowerShell**

    py -3 .\scripts\prysm_verify.py

Si no hay lanzador \`py\`, utilizar:

    python .\scripts\prysm_verify.py

**Linux/macOS**

    python3 ./scripts/prysm_verify.py

Opciones:

    py -3 .\scripts\prysm_verify.py --offline --format
    py -3 .\scripts\prysm_verify.py --out .\reporte-prysm

El modo offline requiere dependencias Cargo precargadas en la caché.

## Resultado verificable

Registra versiones de herramientas, SHA de commit, estado del worktree y SHA-256 de Cargo.toml y Cargo.lock. Ejecuta \`cargo check --locked --all-targets\` y \`cargo test --locked --all-targets\`, y opcionalmente \`cargo fmt --all -- --check\`. Genera logs y \`manifest.json\` en una carpeta temporal o en la ruta seleccionada con \`--out\`.

Códigos de salida: \`0 PASS\` = se ejecutaron y terminaron correctamente todas las comprobaciones solicitadas; \`1 FAIL\` = una comprobación falló; \`2 BLOCKED\` = falta una herramienta o alguna comprobación no pudo ejecutarse.

Pruebas *del verificador* (no son pruebas Rust):

    py -3 -m unittest discover -s scripts -p test_prysm_verify.py -v

En el entorno donde se preparó este script se completaron 5/5 pruebas unitarias del verificador (Python 3.13.5). Aún no se ejecutaron los tests Rust del proyecto porque el entorno de preparación no dispone de Cargo.

## Causa conocida del bloqueo de GitHub Actions

La captura proporcionada el 11 de noviembre de 2026 muestra textualmente: «El trabajo no se ha iniciado porque su cuenta está bloqueada debido a un problema de facturación». Por ello no se asignaron runners. No es evidencia de fallo del código Rust.

**Límites:** los hashes son referencias de integridad, no firmas digitales. El manifiesto es evidencia local autodeclarada, no una auditoría independiente ni certificación de seguridad. No fusionar ni publicar release sin revisar los resultados.

Autor: Jorge Humberto Davalos Gonzalez (ORION-VOX). Implementación/documentación asistidas por ChatGPT; sin coautoría jurídica.
