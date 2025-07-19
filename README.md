# 🧸 Making Token Transfers Work with Token2022 in Solana

## 🐣 What’s the Problem?

Imagine you have two toy machines:

- 🧸 **Old Token Machine**: This one is called `SPL Token`.
- 🤖 **New Token Machine**: This one is called `Token2022`. It's newer, smarter, and has more buttons.

Now, you want to tell these machines to do things like:

- 🎁 "Make a new coin" (mint)
- 📦 "Make a place to hold coins" (token account)
- 💸 "Send coins to a friend" (transfer)

But here’s the problem:

> The talking instructions (`instruction builders`) you have only work with the old machine (SPL Token).  
> When you try to use them with the new machine (Token2022), the new machine just stares at you… 😐  
> It doesn’t understand the words. So nothing works.

---

## 🧠 What’s the Fix?

We decided to speak the new machine’s language! Instead of using the old instructions, we:

1. 📝 Wrote **custom instructions** that the new machine (Token2022) understands.
2. 🎮 Used something called `invoke_signed`, which is like pressing the buttons ourselves.
3. 🔄 Added a switch called `TokenType` so we can choose which machine to talk to (old or new).

---

## 🛠️ How It Works

We made a new enum (a fancy word for a switch) called `TokenType`:

```rust
pub enum TokenType {
    Token,      // Use the old SPL Token machine
    Token2022,  // Use the new Token2022 machine
}

Then, when we want to send coins, we say:

let ix = match token_type {
    TokenType::Token => spl_token::instruction::transfer_checked(
        &token_program.key(),
        &source.key(),
        &mint.key(),
        &destination.key(),
        &authority.key(),
        &[],
        amount,
        decimals,
    )?,
    TokenType::Token2022 => spl_token_2022::instruction::transfer_checked(
        &token_program.key(),
        &source.key(),
        &mint.key(),
        &destination.key(),
        &authority.key(),
        &[],
        amount,
        decimals,
    )?,
};

invoke_signed(&ix, &accounts, signer_seeds)?;
