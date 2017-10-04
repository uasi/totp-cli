# totp-cli

A tiny CLI to generate Time-based One-Time Password (TOTP) passwords.

## Synopsis

```
cat > ~/.totp.toml <<EOS
google = "otpauth://totp/Google%3Ame%40example.com?secret=xxxxxxxx&issuer=Google"
EOS

chmod 600 ~/.totp.toml

totp google # => 123456
```
