# Appium Cross Platform

This sandbox project tests the possibility of using Appium across Web, MacOS,
iOS and Android.

## Status

WIP

## Getting Started

To run the server, use the following command:

```bash
npx appium@3.1.1 --allow-insecure "*:session_discovery" --use-plugins inspector

# To use the inspector visit:
# http://127.0.0.1:4723/inspector
```

### Appium Setup

As the default Appium does not come preinstalled with driver, we can do the
following to install the necessary drivers.

```bash
# Android (WIP)
# npx appium@3.1.1 driver install uiautomator2@6.6.1

# iOS/tvOS (WIP)
# npx appium@3.1.1 driver install xcuitest@10.9.0

# Mac
npx appium@3.1.1 driver install mac2@3.2.8

# Chromuim
npx appium@3.1.1 driver install chromium@2.0.3
npx appium@3.1.1 driver run chromium install-chromedriver
```

While there are several plugin as well, let's install the following for easier
debugging:

```bash
# For inspecting elements on different devices
npx appium@3.1.1 plugin install inspector@2025.11.1
```

## Notes

- The Google Chrome and the chrome driver will need to match before it works. In
  this sandbox, I simply use the latest driver with the latest chrome, maybe it
  will be easier to use/download a custom chromium binary and use
  `CHROMEDRIVER_VERSION` to fix the chromedriver for testing.
