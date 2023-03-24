# random-unicode-emoji

A simple Rust crate that returns random Unicode emojis. ❤️

> Like this repo? Give it a [⭐ on GitHub!](https://github.com/NicPWNs/random-unicode-emoji-rs)

## Install

Install the crate:

```bash
cargo add random-unicode-emoji
```

## Usage

Use the crate:

```rust
// Use the Library
use random_unicode_emoji::random_emoji;

// Use the Function
println!("{}", random_emoji(1, "latest")[0]);
--> 🍭

// Change the Count
println!("{:?}", random_emoji(3, "latest"));
--> ["🏠", "🥑", "👠"]

// Change the Version
println!("{}", random_emoji(1, "15.0")[0]);
--> 🐒
```

### Parameters

- `count`: `usize` (Integer)
- `version`: `&str` (String)

### Return Type

- `Vec<String>` (Vector of Strings)

## Update

Update the crate to the latest version:

```bash
cargo update -p random-unicode-emoji
```

## Unicode

Uses Unicode Standard Emoji from [Unicode.org](https://www.unicode.org/Public/emoji/)

### Supported Unicode Versions

4.0, 5.0, 11.0, 12.0, 12.1, 13.0, 13.1, 14.0, 15.0 (latest)

> _Uses latest version by default._

## Language

This is the Rust ⚙️ version. There is also a [JavaScript 📜](https://github.com/NicPWNs/random-unicode-emoji) and [Python 🐍](https://github.com/NicPWNs/random-unicode-emoji-py) version.

## Maintainer

[Nic Jones, (NicPWNs)](https://github.com/NicPWNs)
