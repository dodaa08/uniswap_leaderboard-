
# Zora Leaderboard

A leaderboard app inspired by Uniswap, showing a sorted list of trader addresses based on trade volume, built with **Rust (Axum)** on the backend and **Next.js** on the frontend. It interacts with the **Zora token** on **Base mainnet**, extracting trade data via **GraphQL**, and provides endpoints for syncing and retrieving leaderboard data.

token : https://dexscreener.com/base/0xedc625b74537ee3a10874f53d170e9c17a906b9c


<img width="1669" height="741" alt="image" src="https://github.com/user-attachments/assets/d748bad5-4724-4f33-88a2-93ec17535c21" />



## Features

- 📊 **Leaderboard**: Sorted list of trader addresses based on trade volume (highest to lowest).
- 🔄 **Sync Endpoint**: Fetches and stores the latest Zora token trades from Base mainnet.
- 📥 **Leaderboard Endpoint**: Serves synced leaderboard data from the database.
- 🔍 **Trader Lookup**: View individual trader stats by appending their address to the URL.
- 🧠 **Buy/Sell Count**: Displays total number of buy and sell transactions per trader.

## Tech Stack

- **Backend**: [Rust](https://www.rust-lang.org/), [Axum](https://docs.rs/axum)
- **Frontend**: [Next.js](https://nextjs.org/)
- **Data Layer**: [GraphQL](https://graphql.org/) (for querying Zora token trades)
- **Blockchain**: [Base Mainnet](https://base.org/)
- **Database**: PostgreSQL (or your choice)

## API Endpoints

### `POST /sync`

Triggers a sync to fetch latest trades and update the database.

### `GET /leaderboard`

Returns a sorted list of trader addresses with trade volume, buy/sell counts.

### `GET /leaderboard/:address`

Returns leaderboard data with the specified address highlighted.

## Setup

1. Clone the repo
2. Configure your `.env` with database and GraphQL endpoint
3. Run the backend (Axum + Rust)
4. Run the frontend (Next.js)

## License

MIT




