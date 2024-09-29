# Solana Deposit and Withdraw Program

This Solana program allows users to initialize an account, deposit SOL into it, and withdraw 10% of the deposited SOL at a given time.

## Breakdown 

``` 
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    system_instruction,
    program::invoke,
    clock::Clock,
    sysvar::Sysvar,
};

```

## Program Struct / Structure  ( Same as your regular language struct 🤗🤗 )

``` 
 #[derive(Clone, Debug, Default, PartialEq)]
struct AccountState {
    balance: u64,
    last_withdrawal_time: i64,
}


 ``` 

## Process Instructions 

``` 
   pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

```


## Initialize Account Function

````
      fn initialize_account(account: &AccountInfo, payer: &AccountInfo) -> ProgramResult {}
 ````


## Deposite Function 

```
fn deposit(account: &AccountInfo, payer: &AccountInfo, amount_data: &[u8]) -> ProgramResult {
 ```


 ## Withdral Function 

```
 fn withdraw(account: &AccountInfo, receiver: &AccountInfo) -> ProgramResult {
```
## Features

- Initialize a new account
- Deposit SOL into the account
- Withdraw 10% of the balance (once every 24 hours)

## Prerequisites

- Rust and Cargo
- Solana CLI tools
- A Solana wallet with some SOL for deployment and testing

## Building the Program
1. Clone the repo

2. Build the program:
   ```
   cargo build-bpf
   ```


## Disclaimer

This program is just me testing
