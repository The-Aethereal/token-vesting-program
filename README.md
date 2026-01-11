```text
VestingAccount (PDA: seed=[company_name])
    ├─→ owner: The company wallet (creates employees)
    ├─→ mint: The SPL token being vested
    ├─→ treasury_token_account: Holds all vesting tokens
    └─→ company_name: Identifier
           │
           └─→ Treasury Token Account (PDA: seed=["vesting_treasury", company_name])
                   ├─→ Authority: Self (PDA signs transfers)
                   └─→ Holds tokens for distribution
                          │
                          └─→ EmployeeAccount (PDA: seed=["employee_vesting", beneficiary, vesting_account])
                                  ├─→ beneficiary: Employee wallet
                                  ├─→ vesting_account: Parent company account
                                  ├─→ Vesting schedule params
                                  └─→ Claims tokens → Employee Token Account (ATA)
```
