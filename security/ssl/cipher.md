# Cipher Types

## AES

<blockquote style="background: black;">
Strength options include <b>128</b>, <b>192</b>, and <b>256</b> bits with
corresponding 10, 12, and 14 rounds.
</blockquote>

### Modes

<!-- markdownlint-disable MD013 -->
| Mode | Description | Pros | Cons | Use Cases |
|------|-------------|------|------|-----------|
| ECB  | Encrypts each block separately | Simple | Not secure, reveals patterns | Not recommended |
| CBC  | XORs each block with the previous ciphertext block | More secure, hides patterns | Requires padding, more complex | File encryption, data transmission |
| CFB  | Converts block cipher to stream cipher | Suitable for variable-length data | Less parallelizable | Secure network protocols, real-time encryption |
| OFB  | Similar to CFB, uses IV and keystream | Pre-computable keystream | Requires synchronization | Secure network protocols |
| CTR  | Encrypts a counter value and XORs with plaintext | Highly parallelizable, efficient | Requires unique counters | High-performance encryption, disk encryption |
<!-- markdownlint-enable -->

## Depreceated

- RC4
- 3DES
- SHA1
- MD5
