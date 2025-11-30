# Maestro Simple

This is a sandbox project for trying out the cross platform UI testing framework
Maestro.

## Status

Working

## Getting Started

```bash
maestro test maestro/web.yaml
maestro test maestro/ios.yaml
maestro test maestro/android.yaml

# For continuous mode
maestro test -c <file>
```

## Note

- Tested on v2.0.10 and my opinion in general:
    * Simple to use, just need to setup yaml file and install their CLI.
      However, this also means that it might be too simple for some other use
      case and harder to create composition for code reusability.
    * Maestro studio is also nice for inspecting Android and iOS.
    * Supports web, Android and iOS. Lacks desktop app support and does not seem
      to be working on it any time soon.
      (<https://github.com/mobile-dev-inc/maestro/issues/1137>)
    * Developer tools are lacking: no JSON/YAML schema, nor any LSP tools out
      there that I can use.
    * Test run rather slowly. Artifacts appeared visually but you can see that
      it takes significant time for actions to be performed.
- Seems like if both Android and iOS emulator are on and you try to run a test
  that begin with `appId`, it will direct to Android emulator. There doesn't
  seem to be any way you can set the correct emulator apart from stopping the
  Android emulator if you want to run the test on iOS emulator.
- As of version, there seems to be no Cypress's "intercept"-like command and we
  will have to set up wiremock to test.
