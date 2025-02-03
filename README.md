# CLI-Finance_Tracker

A simple **command-line finance tracker** built in **Rust**.  
This tool helps you track **expenses and income**, stores data locally, and provides an overview of your financial transactions.

## 🚀 Features
✅ Add expenses with **category & amount**  
✅ Add income entries  
✅ View all transactions (income & expenses)  
✅ Automatically saves transactions in `transactions.json`  
✅ Loads past transactions when restarting the app  
✅ Displays **current date** on launch  

## 🛠 Installation
1. **Clone the repository:** 
```bash
   git clone git@github.com:YOUR_GITHUB_USERNAME/CLI-Finance_Tracker.git
   cd CLI-Finance_Tracker
```
Build the project:

```
cargo build --release
```

Run the CLI app:

```
cargo run
```


📌 Usage

Once you run the program, use these options:

1 - Add Expense
2 - Add Income
3 - Show Transactions
4 - Exit & Save

All transactions are saved in transactions.json (but are ignored from GitHub for privacy).

🔒 Data Privacy

Your transactions are stored locally and never uploaded anywhere.
transactions.json is ignored from GitHub to keep your data private.

🌱 Planned Features
📌 Total balance display (income - expenses)
📌 Transaction categories filter
📌 Better UI formatting
📌 Export transactions as CSV

📜 License
This project is open-source under the MIT License.
