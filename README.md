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
---

```text
PDAs :
    VestingAccount : Derived from company name
    Treasury Token Account : Derived from "vesting_treasury" + company name
    EmployeeAccount : Derived from "employee_vesting" + beneficiary + vesting_account
Associated Token Account:
    employee_token_account : Standard ATA for the employee's wallet, created automatically when claiming
```
---

```text
claim_tokens CPI call
Calculates vested amount based on elapsed time, Uses linear vesting: (total _ amount x time_etapsed) / vested totat_vesting_period and subtracts already withdrawn tokens to get claimable amount.
```
