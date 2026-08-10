mod webhook_queue;

use webhook_queue::{Delivery, QueueClient};

fn main() {
    let client = QueueClient::from_env();
    let delivery = Delivery {
        event_id: "receipt-donor-1042".into(),
        kind: "donor_receipt".into(),
        recipient: "donor-1042".into(),
        target: std::env::var("WEBHOOK_URL").unwrap_or_else(|_| "https://example.org/hooks/receipts".into()),
    };
    let result = client.publish(&delivery).expect("queue publish failed");
    println!("queued {}", result);
}

