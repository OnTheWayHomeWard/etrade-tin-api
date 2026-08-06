# Ethiopian TIN Lookup Service

A small Rust web service that looks up Ethiopian business registration information by **TIN** (Taxpayer Identification Number). It acts as a proxy to the official Ethiopian eTrade portal (`etrade.gov.et`) and returns the registration data as JSON.

## What it does

- Exposes an HTTP endpoint that takes a TIN and an optional language.
- Calls the eTrade registration API (`GetRegistrationInfoByTin`) on your behalf, with browser-like headers so the upstream request succeeds.
- Returns the full JSON response from eTrade, unmodified — including any owner photo, which eTrade embeds as a Base64 string under `AssociateShortInfos[].Photo`.
- Stores nothing on disk.

## Built with

- [axum](https://github.com/tokio-rs/axum) — web framework
- [tokio](https://tokio.rs/) — async runtime
- [reqwest](https://github.com/seanmonstar/reqwest) — HTTP client
- [serde](https://serde.rs/) / serde_json — JSON handling
- base64 — decoding the embedded photo

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024 — use a recent stable toolchain)
- Internet access (the service reaches out to `etrade.gov.et`)

## Running

From the project root:

```bash
cargo run
```

The server starts on:

```
http://0.0.0.0:3000
```

## Usage

Make a GET request to `/tin/{tin}`, optionally passing a `lang` query parameter.

### Endpoint

```
GET /tin/{tin}?lang={en|am|or}
```

| Part   | Description                                  | Default |
|--------|----------------------------------------------|---------|
| `tin`  | The Taxpayer Identification Number to look up | —       |
| `lang` | Response language: `en`, `am`, or `or`        | `en`    |

### Examples

Replace `<TIN>` with the Taxpayer Identification Number you want to look up.

```bash
# Default (English)
curl http://localhost:3000/tin/<TIN>

# English (explicit)
curl http://localhost:3000/tin/<TIN>?lang=en

# Amharic
curl http://localhost:3000/tin/<TIN>?lang=am

# Oromifa
curl http://localhost:3000/tin/<TIN>?lang=or
```

The response is the JSON registration record returned by eTrade, passed through
as-is. If the record contains an owner photo, it comes back inside that JSON as a
Base64 string — decode it client-side if you need the image file.

> You can also use the included `request.http` file with the VS Code REST Client
> extension to try the endpoint with sample TINs.

## Responses

- **200 OK** — JSON registration data from eTrade.
- **502 Bad Gateway** — the upstream request failed or the response could not be parsed as JSON.

## Notes

- This service relies on the public eTrade endpoint; changes to that API may affect behavior.
- The service is stateless — it writes nothing to disk and caches nothing.
- Intended for legitimate lookups against publicly available business registration data.
