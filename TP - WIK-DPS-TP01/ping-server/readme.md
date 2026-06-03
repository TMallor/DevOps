# WIK-DPS-TP01 — Ping Server

API HTTP en Rust pur (zéro dépendance externe) qui retourne les headers de la requête au format JSON.

## Prérequis

- [Rust](https://rustup.rs)

## Lancer le projet

### Port par défaut (8080)
```bash
cargo run
```

### Port personnalisé via variable d'environnement
```powershell
# Windows
$env:PING_LISTEN_PORT="3000"; cargo run
```
```bash
# Linux / Mac
PING_LISTEN_PORT=3000 cargo run
```

## Tester

### GET /ping — retourne 200 + JSON des headers
```powershell
curl -i http://localhost:8080/ping
```

Exemple de réponse :
```json
{
  "Host": "localhost:8080",
  "User-Agent": "curl/8.4.0",
  "Accept": "*/*"
}
```

### Tout le reste — retourne 404
```powershell
curl -i http://localhost:8080/autre
curl -i -X POST http://localhost:8080/ping
```

## Structure du projet

```
ping-server/
├── src/
│   └── main.rs
└── README.md
```