/// Demonstrates Akasha's causal graph: tracking event chains through a distributed system.
///
/// Scenario: A user request triggers a cascade of events across microservices.
/// Akasha captures the full causal chain, letting you answer:
///   "What caused this payment failure?"
///   "What downstream effects did this DB error have?"
use akasha::{Akasha, RecordBuilder};

#[tokio::main]
async fn main() -> akasha::Result<()> {
    let db = Akasha::open_memory().await?;

    // === Simulate a distributed trace as a causal chain ===

    // 1. User request comes in
    let user_request = RecordBuilder::new()
        .schema("http.request.v1")
        .data(serde_json::json!({
            "method": "POST",
            "path": "/checkout",
            "user_id": "user_42",
            "session": "sess_abc"
        }))
        .tag("service", "api-gateway")
        .timestamp(1_000_000_000)
        .build()?;
    let req_id = db.insert(user_request).await?;
    println!("1. HTTP Request:    {}", hex::encode(req_id));

    // 2. Auth service validates the request (caused by HTTP request)
    let auth_check = RecordBuilder::new()
        .schema("auth.check.v1")
        .data(serde_json::json!({
            "user_id": "user_42",
            "result": "authorized",
            "token_valid": true
        }))
        .caused_by(vec![req_id])
        .tag("service", "auth-service")
        .timestamp(1_001_000_000)
        .build()?;
    let auth_id = db.insert(auth_check).await?;
    println!("2. Auth Check:      {}", hex::encode(auth_id));

    // 3. Inventory service checks stock (also caused by HTTP request)
    let inventory_check = RecordBuilder::new()
        .schema("inventory.check.v1")
        .data(serde_json::json!({
            "product_id": "prod_99",
            "quantity_available": 5,
            "requested": 1
        }))
        .caused_by(vec![req_id])
        .tag("service", "inventory-service")
        .timestamp(1_002_000_000)
        .build()?;
    let inv_id = db.insert(inventory_check).await?;
    println!("3. Inventory Check: {}", hex::encode(inv_id));

    // 4. Payment service processes charge (caused by auth + inventory passing)
    let payment = RecordBuilder::new()
        .schema("payment.charge.v1")
        .data(serde_json::json!({
            "amount": 49.99,
            "currency": "USD",
            "gateway": "stripe",
            "status": "failed",
            "error": "insufficient_funds"
        }))
        .caused_by(vec![auth_id, inv_id])
        .tag("service", "payment-service")
        .timestamp(1_003_000_000)
        .build()?;
    let payment_id = db.insert(payment).await?;
    println!("4. Payment Charge:  {}", hex::encode(payment_id));

    // 5. Email notification triggered by payment failure
    let notification = RecordBuilder::new()
        .schema("notification.email.v1")
        .data(serde_json::json!({
            "to": "user42@example.com",
            "template": "payment_failed",
            "sent": true
        }))
        .caused_by(vec![payment_id])
        .tag("service", "notification-service")
        .timestamp(1_004_000_000)
        .build()?;
    let notif_id = db.insert(notification).await?;
    println!("5. Notification:    {}", hex::encode(notif_id));

    // 6. Analytics event logged
    let analytics = RecordBuilder::new()
        .schema("analytics.event.v1")
        .data(serde_json::json!({
            "event": "checkout_failed",
            "user_id": "user_42",
            "reason": "payment_failure"
        }))
        .caused_by(vec![payment_id])
        .tag("service", "analytics-service")
        .timestamp(1_005_000_000)
        .build()?;
    let _analytics_id = db.insert(analytics).await?;
    println!("6. Analytics:       {}", hex::encode(_analytics_id));

    // === Causal Traversal ===
    println!("\n=== All effects of the original HTTP request (depth 5) ===");
    let effects = db.find_effects(&req_id, 5).await?;
    for e in &effects {
        println!("  → [{}] {}", e.schema, e.data["status"].as_str().or(e.data["result"].as_str()).unwrap_or("—"));
    }

    println!("\n=== What caused the payment failure? (ancestors) ===");
    let causes = db.find_causes(&payment_id, 3).await?;
    for c in &causes {
        println!("  ← [{}] {}", c.schema, &c.data.to_string()[..60.min(c.data.to_string().len())]);
    }

    println!("\n=== Shortest causal path: request → notification ===");
    let path = db.causal.shortest_path(&req_id, &notif_id);
    match path {
        Some(p) => println!("  Path length: {} hops", p.len() - 1),
        None => println!("  No path found"),
    }

    println!("\n=== AQSL: find events caused by the HTTP request ===");
    let aqsl = format!(
        "FIND records WHERE caused_by \"{}\" WITH depth 10",
        hex::encode(req_id)
    );
    let results = db.query(&aqsl).await?;
    println!("  {} records in causal subtree", results.len());

    Ok(())
}
