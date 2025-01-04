# Send Email

This is a tool to send test email via SMTP connection URL (`smtp://...`)

## Status

Working

## Getting Started

```bash
# For sending a test mail
cg run -- -f  "smtp://user:pass@smtp.ionos.com:587?tls=required"

# For help
cg run -- -h
```

## Note

For username and password, we might need to encode it if it contains characters
that are reserved for URL.

An easy way is to use NodeJS:

```bash
node -e "console.log(encodeURIComponent('<TO_ENCODE>'))"
```

## Common SMTP URL to Use

### IONOS

`smtp://username:password@smtp.ionos.com:587?tls=required`
