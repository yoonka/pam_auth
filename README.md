# pam_auth

A simple cross-platform CLI tool written in Rust that authenticates users against the system PAM (Pluggable Authentication Modules) stack.

---

## 🔐 What It Does

`pam_auth` is a small binary that takes a username and password as command-line arguments and checks them using PAM — just like how `login`, `sudo`, or `sshd` authenticate users.

✅ Works with system accounts  
✅ Supports any PAM backend (e.g. LDAP, shadow, 2FA)  
✅ Does not require root privileges to run  

---

## 📦 Usage

```bash
pam_auth <username> <password>
```

Example:

```bash
pam_auth edwin secret123
```

Returns:
- Exit code `0`: ✅ Authentication successful
- Exit code `2`: ❌ Authentication failed

---

## 🔧 Building Locally

### Prerequisites

- [Rust](https://rustup.rs)
- `libpam` development headers (Linux/FreeBSD)

```bash
cargo build --release
./target/release/pam_auth username password
```

---

## 🚀 Cross-Platform Builds

We use [cross](https://github.com/cross-rs/cross) to compile for other OS targets:

```bash
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-unknown-freebsd
```

Outputs:

```
target/x86_64-unknown-linux-gnu/release/pam_auth
target/x86_64-unknown-freebsd/release/pam_auth
```

---

## 🛠 GitLab CI/CD

On every tag push, GitLab:
- Builds Linux and FreeBSD binaries
- Publishes them to a GitLab Release
- Provides download links

You can find the artifacts under **Deploy → Releases** in GitLab.

---

## 🧪 Testing

To test the binary:

```bash
./pam_auth root wrongpass    # Should fail
./pam_auth root correctpass  # Should succeed (if root password is known)
```

> Note: For security, run only in secure, trusted environments.

---