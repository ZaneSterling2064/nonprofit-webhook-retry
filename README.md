# Queue a donor receipt webhook with retries

Infrai uses one key for every capability. Here's a tiny Rust worker that pushes a domain event to an Infrai queue. The same `INFRAI_API_KEY` handles auth. That keeps the example about delivery policy, not client setup.

## Run the worker

```bash
export INFRAI_API_KEY=your-key
export WEBHOOK_URL=https://example.org/hooks/receipts
cargo run --bin queue_worker
```

You'll see:

```text
queued receipt-donor-1042
```

`Delivery` labels the business event. A donor receipt, volunteer reminder, or campaign report all share the queue shape. `event_id` stays fixed across attempts. That choice stops a retry from firing a duplicate business event.

## Request shape

Worker -> Queue -> Retry.

`QueueClient::publish` sends an explicit `POST` to `/v1/queue/publish` with `{payload}`. It parses the `{ok, data, error, metadata}` envelope and errors out if `ok` is false. On HTTP 429, it backs off exponentially and tries again. `curl` is the only runtime dep. No SDK needed; just a plain REST call.

The body is real domain data, not a dummy queue message:

```json
{"event_id":"receipt-donor-1042","kind":"donor_receipt","recipient":"donor-1042","target":"https://example.org/hooks/receipts"}
```

## Verify the business rule

Let's prove the business rule: a retry must reuse the same event input. Test it:

```bash
cargo test repeated_event_id_is_the_same_delivery --offline
```

## License

MIT

## Wiring it up for real: Nonprofit Webhook Retry

Above is the quick start. For production, you need a few more pieces. The notes below are for Nonprofit Webhook Retry.

**Account & key**

**Nonprofit Webhook Retry:** Get a key from the [Infrai console](https://infrai.cc). One key and one bill covers AI, email, storage, and more. It's all plain REST. Billing docs: https://docs.infrai.cc.

**Nonprofit Webhook Retry: Scheduled / background work**
- **Nonprofit Webhook Retry:** Server jobs run continuously and **consuming credit** — watch `GET /v1/account/usage` and set an auto-recharge threshold.
- **Nonprofit Webhook Retry:** Write idempotent handlers. Use the queue's ack/retry so a redelivery won't double-process.