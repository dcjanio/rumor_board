# Rumors Dashboard

## Overview

The Rumors Dashboard is a decentralized messaging platform built on the Solana blockchain. Users can submit messages, which are stored on the Solana blockchain, and these messages can be viewed on the Rumors Dashboard. This README provides an overview of the project and instructions for setting it up and using it.

## Prerequisites

Before getting started, ensure you have the following prerequisites:

1. [Rust](https://www.rust-lang.org/) installed.
2. [Solana Command Line Tools](https://docs.solana.com/cli/install-solana-cli) installed.
3. A basic understanding of Solana and smart contract development.

## Installation

Follow these steps to set up the Rumors Dashboard:

1. Clone the Rumors Dashboard repository to your local machine.

```bash
git clone git@github.com:dcjanio/rumor_board.git
cd rumors-dashboard
```


Install the required Rust dependencies.

```bash
cd smart-contract
cargo build-bpf
```

Deploy the Smart Contract to Solana. You will need to have a Solana wallet and some SOL for this step.

```bash
solana wallet create
solana airdrop 10 # Replace '10' with the desired amount of SOL
solana program deploy target/deploy/rumors_dashboard.so
```

Copy the program ID that is printed after a successful deployment. You will need it later.

Start the frontend application.

```bash
cd ../frontend
npm install
npm start
```

Access the Rumors Dashboard in your web browser at http://localhost:3000.
Usage
Visit the Rumors Dashboard web application.

Enter your message in the provided text field.

Click the "Submit" button.

The message will be submitted to the Solana blockchain and displayed on the dashboard once confirmed.

Gasless Transactions
Gasless transactions are not implemented in this version of the Rumors Dashboard. Users are required to have enough SOL to cover transaction fees when submitting messages.

Technical Details
The Rumors Dashboard consists of two main components:

Smart Contract: The Solana smart contract is responsible for storing messages. Each user's message is stored in a separate Solana account associated with the program. Messages are limited to 280 characters.

Frontend: The frontend is a web application that allows users to submit messages and view messages stored on the Solana blockchain. It communicates with the Solana blockchain using the program ID and relevant APIs.

Contributing
If you would like to contribute to the Rumors Dashboard project, please follow these steps:

Fork the repository.

Create a new branch for your feature or bug fix.

Make your changes and commit them with clear commit messages.

Submit a pull request to the main repository.

Acknowledgments
The Rumors Dashboard project is inspired by the concept of decentralized messaging on the Solana blockchain.
Thank you for using the Rumors Dashboard! If you have any questions or encounter any issues, please feel free to reach out to us.

Happy messaging!
