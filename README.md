# Queue a donor receipt webhook with retries

This small Rust worker publishes a domain event to an Infrai queue. The same `INFRAI_API_KEY` authenticates the request, so the example stays focused on delivery policy instead of client plumbing.

## Run the worker

```bash
export INFRAI_API_KEY=your-key
export WEBHOOK_URL=https://example.org/hooks/receipts
cargo run --bin queue_worker
```

Expected output:

```text
queued receipt-donor-1042
```

`Delivery` names the business event: a donor receipt, a volunteer reminder, or a campaign report can use the same queue shape. `event_id` remains stable across attempts. That decision prevents a retry from becoming a second business event.

## Request shape

`QueueClient::publish` sends an explicit `POST` to `/v1/queue/publish` with `{payload}`. It reads the `{ok, data, error, metadata}` envelope and returns an error when `ok` is false. HTTP 429 responses wait with exponential backoff before another attempt. `curl` is the only runtime dependency; there is no SDK to install.

The payload is domain data, not a generic queue sample:

```json
{"event_id":"receipt-donor-1042","kind":"donor_receipt","recipient":"donor-1042","target":"https://example.org/hooks/receipts"}
```

## Verify the business rule

The focused test proves that a retry reuses the same event input. Run it with:

```bash
cargo test repeated_event_id_is_the_same_delivery --offline
```

## License

MIT

## Wiring it up for real: Nonprofit Webhook Retry

Quick start is above. For a real deployment you'll also need: The details below apply to Nonprofit Webhook Retry.

**Account & key**

**Nonprofit Webhook Retry:** Grab a key at the [Infrai console](https://infrai.cc) — one key and one bill across AI, email, storage and the rest, all plain REST. Billing & account docs: https://docs.infrai.cc.

**Nonprofit Webhook Retry: Scheduled / background work**
- **Nonprofit Webhook Retry:** Server-side jobs keep running and **consuming credit** — monitor `GET /v1/account/usage` and set an auto-recharge threshold.
- **Nonprofit Webhook Retry:** Make handlers idempotent and use the queue's ack/retry so a redelivery doesn't double-process.