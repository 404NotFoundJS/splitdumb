# Splitdumb

A simple expense-sharing app. Track shared expenses, calculate balances, and settle up with minimal transactions.

## Features

- **Groups**: Organize expenses by trip, household, event, etc.
- **Expense tracking**: Record who paid and who participated
- **Balance calculation**: See who owes what at a glance
- **Smart settlements**: Minimizes the number of payments needed
- **Simplify debts**: Optional mode that consolidates debts across the group

## Quick Start

```bash
# Docker
docker compose up --build -d
open http://localhost:8080

# Manual
cargo run -- serve &
cd web && bun install && bun run dev
open http://localhost:5173
```

## CLI

```bash
# Add expense directly
cargo run -- add-expense -d "Dinner" -a 60 -p Alice -P Alice,Bob,Charlie

# View balances
cargo run -- show-balances

# View settlements
cargo run -- show-settlements
```

## Tech Stack

- **Backend**: Rust, Axum, Tokio
- **Frontend**: React 19, TypeScript, Vite
- **Storage**: JSON file (`app_data.json`)
- **Auth**: Phone number + bearer token
