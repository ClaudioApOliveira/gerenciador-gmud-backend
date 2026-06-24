# Gerencia GMUD Backend

API REST em Rust para gerenciamento de GMUD usando Actix Web e MongoDB.

## Padrao de resposta

Sucesso:

```json
{
  "success": true,
  "requestId": "req-123",
  "message": "operacao realizada",
  "data": {}
}
```

Erro:

```json
{
  "success": false,
  "requestId": "req-123",
  "message": "descricao do erro",
  "errorType": "validation_error"
}
```

Paginacao:

```json
{
  "success": true,
  "requestId": "req-123",
  "message": "lista carregada",
  "data": [],
  "pagination": {
    "page": 1,
    "limit": 10,
    "totalItems": 100,
    "totalPages": 10
  }
}
```

## Estrutura por recurso

```text
src/
  auth/
    dtos.rs
    extractor.rs
    handlers.rs
    services.rs
    mod.rs
  config/
    db.rs
    mod.rs
  errors/
    api_error.rs
    mod.rs
  gmud/
    handlers.rs
    models.rs
    dtos.rs
    services.rs
    mod.rs
  user/
    handlers.rs
    models.rs
    services.rs
    mod.rs
  routes.rs
  main.rs
```

Cada recurso segue o mesmo padrao:
- `models`: entidades persistidas
- `dtos`: contratos de entrada/saida da API
- `handlers`: endpoints HTTP
- `services`: regra de negocio e acesso ao banco

## Variaveis de ambiente

- `HOST` (opcional, default `0.0.0.0`)
- `PORT` (opcional, default `8080`)
- `CORS_ALLOWED_ORIGINS` (opcional, lista separada por vírgula de origens permitidas)
- `MONGODB_URI` (obrigatoria)
- `DATABASE_NAME` (obrigatoria)
- `JWT_SECRET` (obrigatoria)
- `JWT_ACCESS_EXP_MINUTES` (opcional, default `15`)
- `JWT_REFRESH_EXP_HOURS` (opcional, default `72`)
- `AUTH_USERNAME` (obrigatoria)
- `AUTH_PASSWORD` (obrigatoria)
- `AUTH_ROLE` (opcional, default `admin`)
- `COOKIE_SECURE` (opcional, default `false`)
- `DEFAULT_USER_PASSWORD` (opcional, usado apenas como fallback em bootstrap)

Observacao:
- para rodar atrás de proxy, container ou domínio público, use `HOST=0.0.0.0`
- se o frontend estiver em outra origem, defina `CORS_ALLOWED_ORIGINS` com a URL exata, por exemplo `http://localhost:3000,http://gmud.192.168.2.144.sslip.io`
- o backend aceita automaticamente origens de `localhost`, `127.0.0.1` e `sslip.io`

## Autenticacao

- `POST /api/v1/auth/login`: valida usuario/senha e cria cookie HTTP-only com JWT
- `POST /api/v1/auth/refresh`: renova o access token usando o refresh cookie
- `POST /api/v1/auth/logout`: remove cookie de autenticacao
- `GET /api/v1/auth/me`: retorna usuario autenticado
- endpoints de `gmud` exigem cookie valido

Controle de acesso por role:
- `gmud`:
  - leitura (`GET`) -> `admin`, `developer`, `approver`
  - escrita (`POST/PUT`) -> `admin`, `developer`
  - exclusao (`DELETE`) -> `admin`
- `users`:
  - criacao (`POST`) -> `admin`
  - listagem (`GET`) -> `admin`, `approver`

Exemplo de login:

```bash
curl -i -X POST "http://localhost:8080/api/v1/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}'
```

## Usuarios

- `POST /api/v1/users`: cria um novo usuario
- `GET /api/v1/users`: lista usuarios
- `role` aceitos: `developer`, `approver`, `admin`
- a senha de usuario é armazenada com hash bcrypt
- login passa a aceitar usuarios cadastrados no MongoDB

Exemplo de criacao:

```bash
curl -i -X POST "http://localhost:8080/api/v1/users" \
  -H "Content-Type: application/json" \
  -d '{"name":"Maria Silva","role":"developer"}'
```

## Execucao rapida

```bash
cd "/Users/claudiodeoliveira/Documents/Projetos/gerencia-gmud/gerencia-gmud-backend"
export MONGODB_URI="mongodb://localhost:27017"
export DATABASE_NAME="gmud"
cargo run
```

## Testes

```bash
cd "/Users/claudiodeoliveira/Documents/Projetos/gerencia-gmud/gerencia-gmud-backend"
cargo test
```

## Collection Postman

Arquivos gerados em `postman/`:

- `GMUD-API.postman_collection.json`
- `GMUD-API.postman_environment.json`

Importe ambos no Postman e selecione o ambiente `GMUD Local`.

