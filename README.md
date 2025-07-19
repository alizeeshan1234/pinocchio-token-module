# 🛠️ Fix: Token2022 CPI Compatibility in Pinocchio

## 📌 Problem Statement

The default instruction builders in the `spl-token` crate are **not compatible** with the `Token2022` program. These builders:

- Assume the use of the legacy SPL Token program (`spl_token::id()`).
- Hardcode instruction formats that are not accepted by Token2022.
- Cause runtime failures or silent errors when used with `spl_token_2022::id()`.

This makes it impossible to perform **Cross-Program Invocation (CPI)** with `Token2022` using the default APIs.

🔗 **Related issue:** [anza-xyz/pinocchio#39](https://github.com/anza-xyz/pinocchio/issues/39)

---

## ✅ Solution

This fix introduces **manual instruction construction** and **token program abstraction** to support Token2022 seamlessly.

### Key Improvements

- ✳️ **Manual Instruction Building**  
  Directly constructs CPI-safe instructions like `initialize_mint2`, `initialize_account3`, and `transfer_checked` from the `spl-token-2022` crate.

- ✳️ **Dynamic Token Program Support**  
  A `TokenType` enum abstracts over SPL Token and Token2022, allowing runtime selection of the correct instruction set.

- ✳️ **Controlled CPI Dispatching**  
  All token instructions are invoked using `invoke_signed` for maximum control and signer support.

---

## 🔍 How It Works

Each instruction is manually built based on the selected token program using a custom `TokenType` enum:

```rust
pub enum TokenType {
    Token,
    Token2022,
}
