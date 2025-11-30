use axum::http::StatusCode;
use rules5e_service::server::RulesServer;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

async fn start_test_server() -> u16 {
    let server = RulesServer::new().unwrap();
    let port = 7004;
    
    // Start server in background
    let server_clone = server;
    tokio::spawn(async move {
        let _ = server_clone.start(port).await;
    });
    
    // Wait for server to start
    sleep(Duration::from_millis(500)).await;
    port
}

#[tokio::test]
async fn test_health_endpoint() {
    let port = start_test_server().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get(&format!("http://localhost:{}/health", port))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "ok");
    assert_eq!(body["service"], "rules5e-service");
}

#[tokio::test]
async fn test_roll_endpoint() {
    let port = start_test_server().await;
    let client = reqwest::Client::new();
    
    let request_body = json!({
        "expression": "1d20",
        "seed": 12345
    });
    
    let response = client
        .post(&format!("http://localhost:{}/roll", port))
        .json(&request_body)
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["result"]["total"].as_i64().unwrap() >= 1);
    assert!(body["result"]["total"].as_i64().unwrap() <= 20);
}

#[tokio::test]
async fn test_roll_endpoint_2d8_plus_3() {
    let port = start_test_server().await;
    let client = reqwest::Client::new();
    
    let request_body = json!({
        "expression": "2d8+3",
        "seed": 12345
    });
    
    let response = client
        .post(&format!("http://localhost:{}/roll", port))
        .json(&request_body)
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let total = body["result"]["total"].as_i64().unwrap();
    assert!(total >= 5 && total <= 19);
}

#[tokio::test]
async fn test_attack_endpoint() {
    let port = start_test_server().await;
    let client = reqwest::Client::new();
    
    let request_body = json!({
        "attack_bonus": 5,
        "ac": 15,
        "advantage": false,
        "disadvantage": false,
        "seed": 100
    });
    
    let response = client
        .post(&format!("http://localhost:{}/attack", port))
        .json(&request_body)
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["roll"].as_i64().is_some());
    assert!(body["hit"].as_bool().is_some());
}

#[tokio::test]
async fn test_ability_check_endpoint() {
    let port = start_test_server().await;
    let client = reqwest::Client::new();
    
    let request_body = json!({
        "ability": "Strength",
        "ability_modifier": 3,
        "proficiency_bonus": 2,
        "has_proficiency": false,
        "has_expertise": false,
        "dc": 15,
        "advantage": false,
        "disadvantage": false,
        "seed": 100
    });
    
    let response = client
        .post(&format!("http://localhost:{}/ability-check", port))
        .json(&request_body)
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["roll"].as_i64().is_some());
    assert!(body["success"].as_bool().is_some());
}

#[tokio::test]
async fn test_saving_throw_endpoint() {
    let port = start_test_server().await;
    let client = reqwest::Client::new();
    
    let request_body = json!({
        "ability": "dex",
        "ability_modifier": 2,
        "proficiency_bonus": 3,
        "has_proficiency": false,
        "dc": 15,
        "advantage": false,
        "disadvantage": false,
        "seed": 100
    });
    
    let response = client
        .post(&format!("http://localhost:{}/saving-throw", port))
        .json(&request_body)
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["roll"].as_i64().is_some());
    assert!(body["success"].as_bool().is_some());
}

#[tokio::test]
async fn test_endpoint_error_handling() {
    let port = start_test_server().await;
    let client = reqwest::Client::new();
    
    // Invalid dice expression
    let request_body = json!({
        "expression": "invalid",
        "seed": 12345
    });
    
    let response = client
        .post(&format!("http://localhost:{}/roll", port))
        .json(&request_body)
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_endpoint_concurrent_requests() {
    let port = start_test_server().await;
    let client = reqwest::Client::new();
    
    let mut handles = vec![];
    for i in 0..100 {
        let client_clone = client.clone();
        let port_clone = port;
        let handle = tokio::spawn(async move {
            let request_body = json!({
                "expression": "1d20",
                "seed": i
            });
            
            let response = client_clone
                .post(&format!("http://localhost:{}/roll", port_clone))
                .json(&request_body)
                .send()
                .await
                .unwrap();
            
            assert_eq!(response.status(), StatusCode::OK);
        });
        handles.push(handle);
    }
    
    // Wait for all requests
    for handle in handles {
        handle.await.unwrap();
    }
}


