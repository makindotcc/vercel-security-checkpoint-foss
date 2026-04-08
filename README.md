# Vercel security checkpoint foss reimplementation
Open-source Rust implementation of Vercel's security checkpoint attestator (their worse clone of cloudflare's IUAM).

Vercel serves a SHA256-based proof of work challenge to visitors before granting access to sites when ``Under Attack`` mode is enabled.
The original solver runs as obfuscated javascript with a .wasm component (where pow is implemented).

This project reimplements .wasm logic in Rust in a bit more efficient way saving energy for better future for all of us.
