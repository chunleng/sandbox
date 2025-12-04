use appium_client::find::By;
use appium_client::wait::AppiumWait;
use appium_client::ClientBuilder;

use crate::utils::chromium::WebCapabilities;

mod utils;

#[tokio::test]
async fn test_dioxus_learn_link() {
    let caps = WebCapabilities::new();
    let client = ClientBuilder::native(caps)
        .connect("http://127.0.0.1:4723/")
        .await
        .expect("Failed to connect to Appium server");

    // Navigate to the Dioxus app
    client
        .goto("http://127.0.0.1:8080")
        .await
        .expect("Failed to navigate to localhost:8080");

    // Use explicit wait instead of sleep - waits until element appears after navigation
    client
        .appium_wait()
        .for_element(By::xpath("//a[contains(., 'Learn Dioxus')]"))
        .await
        .expect("Failed to find 'Learn Dioxus' link")
        .click()
        .await
        .expect("Failed to click on 'Learn Dioxus' link");

    let current_url = client
        .current_url()
        .await
        .expect("Failed to get current URL");
    let url_str = current_url.as_str();
    assert!(
        url_str.contains("dioxuslabs.com"),
        "Expected URL to contain 'dioxuslabs.com', but got: {}",
        url_str
    );
}


