```bash
cargo build -p auth --features standalone
```

---

## Trade-offs

| Aspect                 | Separate `-app` Crates                    | Feature-Flagged Binary                    |
|------------------------|-------------------------------------------|-------------------------------------------|
| **File organization**  | More crates to manage                     | Fewer crates, simpler workspace           |
| **Dependency clarity** | Binary deps isolated naturally            | Must carefully gate deps behind features  |
| **Compile times**      | Cleaner—lib users don't see bin deps      | Risk of leaking bin deps if misconfigured |
| **Discoverability**    | Obvious what's runnable                   | Requires knowing the feature exists       |
| **CI/CD**              | Straightforward `cargo build -p auth-app` | Must remember `--features standalone`     |
| **Code proximity**     | Entry point separated from logic          | Entry point lives with its module         |
| **Refactoring**        | Two places to update                      | One crate to manage                       |

---

## When to Prefer Each

**Separate `-app` crates work well when:**
- You have substantial CLI/server setup code
- Binary dependencies (clap, tokio, etc.) are heavy and you want clean separation
- Multiple teams work on different parts
- You want crystal-clear "this is a runnable thing" signals

**Feature-flagged binaries work well when:**
- The entry point is thin (just wiring up the library)
- You want fewer crates to manage
- The module is often run standalone during development
- You're disciplined about gating dependencies properly

---

## A Hybrid Approach

Some projects use a **single top-level application crate** that composes all modules:
```
my-project/
├── auth/src/lib.rs
├── payments/src/lib.rs
├── app/
│   └── src/main.rs   # Composes auth + payments into one binary
