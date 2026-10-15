# Changelog

## 0.1.5 (2026-10-15)

### Fixed
- **Memory bloat in long-running bots.** Every Bot API request went through
  `reqwest::blocking::get`, which builds and tears down a whole HTTP client
  (TLS config plus a background thread running its own tokio runtime) per
  call. All requests now share one process-wide `reqwest::blocking::Client`
  (60s timeout), which also reuses pooled keep-alive connections instead of
  doing a TLS handshake per request.
- **`start_polling` request storm.** It called `getUpdates` with no timeout in
  a tight loop, so Telegram answered instantly and the bot fired several
  requests per second, forever. Combined with the per-request client above,
  this fragmented glibc malloc arenas until a bot that had been up for 22 days
  sat at ~4GB RSS. `start_polling` now long-polls (`timeout=30`): updates
  still arrive immediately, and an idle bot makes one request every 30s.
- Network errors in the polling loop back off for 2s instead of busy-looping.
- The polling thread no longer panics (`assert!`) when the update receiver is
  dropped; it exits cleanly.

### Added
- Public constants `methods::LONG_POLL_TIMEOUT_SECS` and
  `methods::HTTP_TIMEOUT_SECS`.
- `make clippy` target (`cargo clippy --all-targets -- -D warnings`).

### Changed
- Test code cleaned up for current clippy (`needless_late_init`).

## 0.1.4 (2026-04-07)
- Version bump.

## 0.1.3 (2026-04-07)
- Fix panics on network errors: `res["ok"]` checks use `unwrap_or(false)`,
  and malformed HTTP response bodies are handled instead of unwrapped.
- Fix panics on unknown Telegram message entity types.
- Fix all `clippy -D warnings` issues.

## 0.1.2 (2025-11-12)
- `expand_make_request_to_message` and `expand_make_request_to_bool` no
  longer panic.

## 0.1.1 (2025-02-14)
- Add the `blockquote` `MessageEntityType`.
- reqwest 0.11.22 -> 0.12.9.
