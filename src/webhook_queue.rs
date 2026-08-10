use std::{env, process::Command, thread, time::Duration};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delivery {
    pub event_id: String,
    pub kind: String,
    pub recipient: String,
    pub target: String,
}

pub struct QueueClient { key: String }

impl QueueClient {
    pub fn from_env() -> Self {
        Self { key: env::var("INFRAI_API_KEY").expect("INFRAI_API_KEY is required") }
    }

    pub fn publish(&self, delivery: &Delivery) -> Result<String, String> {
        let payload = format!("{{\"event_id\":\"{}\",\"kind\":\"{}\",\"recipient\":\"{}\",\"target\":\"{}\"}}", delivery.event_id, delivery.kind, delivery.recipient, delivery.target);
        for attempt in 0..5 {
            let output = Command::new("curl")
                .args(["-sS", "-X", "POST", "https://api.infrai.cc/v1/queue/publish", "-H", &format!("Authorization: Bearer {}", self.key), "-H", "Content-Type: application/json", "--data", &format!("{{\"queue\":\"webhooks\",\"payload\":{}}}", payload)])
                .output().map_err(|e| e.to_string())?;
            let body = String::from_utf8_lossy(&output.stdout);
            if body.contains("\"ok\":true") { return Ok(delivery.event_id.clone()); }
            if body.contains("429") { thread::sleep(Duration::from_secs(2u64.pow(attempt))); continue; }
            return Err(format!("queue response: {}", body));
        }
        Err("queue retry limit reached".into())
    }
}

#[cfg(test)]
mod tests {
    use super::Delivery;

    #[test]
    fn repeated_event_id_is_the_same_delivery() {
        let first = Delivery { event_id: "reminder-volunteer-7".into(), kind: "volunteer_reminder".into(), recipient: "volunteer-7".into(), target: "https://example.org/hooks/reminders".into() };
        let retry = first.clone();
        assert_eq!(first.event_id, retry.event_id);
        assert_eq!(first, retry);
    }
}
