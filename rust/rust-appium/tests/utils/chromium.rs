use std::ops::{Deref, DerefMut};

use appium_client::capabilities::AppiumCapability;

type Capabilities = serde_json::Map<String, serde_json::Value>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct WebCapabilities {
    inner: Capabilities,
}

impl AppiumCapability for WebCapabilities {}

impl WebCapabilities {
    pub fn new() -> Self {
        Self {
            inner: serde_json::json!({
                "platformName": "mac",
                "browserName": "chrome",
                "appium:automationName": "Chromium",
                "appium:newCommandTimeout": 5
            })
            .as_object()
            .unwrap()
            .clone(),
        }
    }
}

impl Deref for WebCapabilities {
    type Target = Capabilities;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for WebCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
