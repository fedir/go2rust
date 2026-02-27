# Records API - Documentation Agent

API REST en Go (Gin) permettant la réception, le traçage et le stockage de données JSON.

## Architecture
- **Framework** : `gin-gonic/gin`
- **Stockage** : Fichiers JSON locaux dans `./data/`
- **Spécification** : OpenAPI 3.0 (`openapi.yaml`)

## Endpoints

### 1. Gestion des données
- `POST /api/v1/records` : Crée un enregistrement.
    - Header optionnel : `X-Trace-ID`
    - Payload : JSON arbitraire
- `GET /api/v1/records/{uuid}` : Récupère un enregistrement stocké.

### 2. Auto-documentation
- `GET /api/v1/openapi.yaml` : Spécification au format YAML.
- `GET /api/v1/openapi.json` : Spécification au format JSON (conversion dynamique).

## Développement
Lancer le serveur :
```bash
CGO_ENABLED=0 go run main.go
```

Les fichiers sont stockés dans le dossier `./data` sous la structure suivante :
- `uuid` : Identifiant unique v4.
- `trace_id` : ID de traçabilité (header ou auto-généré).
- `timestamp` : Heure de réception (UTC).
- `payload` : Données brutes reçues.
