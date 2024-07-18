export type SaturnV1 = {
  "version": "0.1.0",
  "name": "saturn_v_1",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "applyBond",
      "accounts": [
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creatorTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryStfTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "priceUpdate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMintAddress",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stfTokenMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "ApplyBondArgs"
          }
        }
      ]
    },
    {
      "name": "finishBond",
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "destStfAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stfTokenMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "stakeStf",
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userStakeAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stfTokenMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amountToStake",
          "type": "u64"
        }
      ]
    },
    {
      "name": "unstakeStf",
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userStakeAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stfTokenMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amountToUnstake",
          "type": "u64"
        }
      ]
    },
    {
      "name": "swap",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wbtcTreasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wbtcMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "usdtTreasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdtMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "usdcTreasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdcMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "solMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "priceUpdate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "jupiterProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "data",
          "type": "bytes"
        },
        {
          "name": "fromAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "meteoraDeposit",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "user CHECK: this is pda"
          ]
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userPoolLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userAToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "dynamicAmmProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "poolTokenAmount",
          "type": "u64"
        },
        {
          "name": "maximumTokenAAmount",
          "type": "u64"
        },
        {
          "name": "maximumTokenBAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "meteoraWithdraw",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "user CHECK: this is pda"
          ]
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userPoolLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userAToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "dynamicAmmProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "poolTokenAmount",
          "type": "u64"
        },
        {
          "name": "maximumTokenAAmount",
          "type": "u64"
        },
        {
          "name": "maximumTokenBAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initLendingAccounts",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "klendProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "seed1Account",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "seed2Account",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lendingMarket",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "obligation",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marginfiAccount",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "klendLend",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "obligation",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lendingMarket",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lendingMarketAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reserveLiquiditySupply",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reserveCollateralMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reserveDestinationDepositCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userSourceLiquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userDestinationCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "instruction",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "klendProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "klendWithdraw",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userDestinationLiquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "instruction",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "klendProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "obligation",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lendingMarket",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "withdrawReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reserveSourceCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reserveCollateralMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reserveLiquiditySupply",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lendingMarketAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userDestinationCollateral",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "marginfiLend",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userLiquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bankLiquidityVault",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "marginfiWithdraw",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marginfiProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userLiquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bankLiquidityVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bankLiquidityVaultAuthority",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "getValueInMeteora",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pool",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userPoolLp",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": "u64"
    },
    {
      "name": "getValueInKamino",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lendingMarket",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "solReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdcReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdtReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wbtcReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wethReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bonkReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "obligation",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": {
        "array": [
          "u64",
          6
        ]
      }
    },
    {
      "name": "getValueInMarginfi",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marginfiGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdcBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdtBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wbtcBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wethBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bonkBank",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": {
        "array": [
          "u64",
          6
        ]
      }
    },
    {
      "name": "calcuBalance",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "user CHECK: this is pda"
          ]
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdcTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wbtcTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "priceUpdate",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "reallocate",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "user CHECK: this is pda"
          ]
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdcTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wbtcTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "priceUpdate",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "returnRate",
          "type": {
            "array": [
              "f64",
              7
            ]
          }
        },
        {
          "name": "riskRating",
          "type": {
            "array": [
              "f64",
              7
            ]
          }
        }
      ]
    },
    {
      "name": "cashingoutReedem",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeWalletTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryStfTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenMintAddress",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stfTokenMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "escrow",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "type": "publicKey"
          },
          {
            "name": "tokenMint",
            "type": "publicKey"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "startTimestamp",
            "type": "i64"
          },
          {
            "name": "endTimestamp",
            "type": "i64"
          },
          {
            "name": "numTokenToRedeem",
            "type": "u64"
          },
          {
            "name": "isFinished",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "sequenceFlag",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "flagCalcuBalance",
            "type": "bool"
          },
          {
            "name": "flagReallocate",
            "type": "bool"
          },
          {
            "name": "flagMarginfi",
            "type": "bool"
          },
          {
            "name": "flagKamino",
            "type": "bool"
          },
          {
            "name": "flagMeteora",
            "type": "bool"
          },
          {
            "name": "flagJupiter",
            "type": "bool"
          },
          {
            "name": "flagSwap",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "treasury",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "treasuryAdmin",
            "type": "publicKey"
          },
          {
            "name": "stakingIndex",
            "type": "u64"
          },
          {
            "name": "treasuryValue",
            "type": "u64"
          },
          {
            "name": "tokenMinted",
            "type": "u64"
          },
          {
            "name": "tokenStaked",
            "type": "u64"
          },
          {
            "name": "meteoraDepositAssets",
            "type": "u64"
          },
          {
            "name": "meteoraDepositValue",
            "type": "u64"
          },
          {
            "name": "meteoraAllocation",
            "type": "f64"
          },
          {
            "name": "kaminoLendAssets",
            "type": "u64"
          },
          {
            "name": "kaminoLendValue",
            "type": "u64"
          },
          {
            "name": "kaminoAllocation",
            "type": "f64"
          },
          {
            "name": "marginfiLendAssets",
            "type": "u64"
          },
          {
            "name": "marginfiLendValue",
            "type": "u64"
          },
          {
            "name": "marginfiAllocation",
            "type": "f64"
          },
          {
            "name": "jupiterPerpsValue",
            "type": "u64"
          },
          {
            "name": "jupiterAllocation",
            "type": "f64"
          },
          {
            "name": "usdcAllocation",
            "type": "f64"
          },
          {
            "name": "wbtcAllocation",
            "type": "f64"
          },
          {
            "name": "solAllocation",
            "type": "f64"
          }
        ]
      }
    },
    {
      "name": "userStakeAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalStakedIndex",
            "type": "u64"
          },
          {
            "name": "totalPoints",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "ApplyBondArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "spotPrice",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "VaultError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "VaultIsDisabled"
          },
          {
            "name": "ExceededSlippage"
          },
          {
            "name": "StrategyIsNotExisted"
          },
          {
            "name": "UnAuthorized"
          },
          {
            "name": "MathOverflow"
          },
          {
            "name": "ProtocolIsNotSupported"
          },
          {
            "name": "UnMatchReserve"
          },
          {
            "name": "InvalidLockedProfitDegradation"
          },
          {
            "name": "MaxStrategyReached"
          },
          {
            "name": "StrategyExisted"
          },
          {
            "name": "InvalidUnmintAmount"
          },
          {
            "name": "InvalidAccountsForStrategy"
          },
          {
            "name": "InvalidBump"
          },
          {
            "name": "AmountMustGreaterThanZero"
          },
          {
            "name": "MangoIsNotSupportedAnymore"
          },
          {
            "name": "StrategyIsNotSupported"
          },
          {
            "name": "PayAmountIsExeeced"
          }
        ]
      }
    },
    {
      "name": "MarginfiError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "MathError"
          },
          {
            "name": "BankNotFound"
          },
          {
            "name": "LendingAccountBalanceNotFound"
          },
          {
            "name": "BankAssetCapacityExceeded"
          },
          {
            "name": "InvalidTransfer"
          },
          {
            "name": "MissingPythOrBankAccount"
          },
          {
            "name": "MissingPythAccount"
          },
          {
            "name": "InvalidOracleAccount"
          },
          {
            "name": "MissingBankAccount"
          },
          {
            "name": "InvalidBankAccount"
          },
          {
            "name": "BadAccountHealth"
          },
          {
            "name": "LendingAccountBalanceSlotsFull"
          },
          {
            "name": "BankAlreadyExists"
          },
          {
            "name": "IllegalLiquidation"
          },
          {
            "name": "AccountNotBankrupt"
          },
          {
            "name": "BalanceNotBadDebt"
          },
          {
            "name": "InvalidConfig"
          },
          {
            "name": "StaleOracle"
          },
          {
            "name": "BankPaused"
          },
          {
            "name": "BankReduceOnly"
          },
          {
            "name": "BankAccoutNotFound"
          },
          {
            "name": "OperationDepositOnly"
          },
          {
            "name": "OperationWithdrawOnly"
          },
          {
            "name": "OperationBorrowOnly"
          },
          {
            "name": "OperationRepayOnly"
          },
          {
            "name": "NoAssetFound"
          },
          {
            "name": "NoLiabilityFound"
          },
          {
            "name": "InvalidOracleSetup"
          },
          {
            "name": "IllegalUtilizationRatio"
          },
          {
            "name": "BankLiabilityCapacityExceeded"
          },
          {
            "name": "InvalidPrice"
          },
          {
            "name": "IsolatedAccountIllegalState"
          },
          {
            "name": "EmissionsAlreadySetup"
          },
          {
            "name": "OracleNotSetup"
          },
          {
            "name": "InvalidSwitchboardDecimalConversion"
          },
          {
            "name": "CannotCloseOutstandingEmissions"
          },
          {
            "name": "EmissionsUpdateError"
          },
          {
            "name": "AccountDisabled"
          },
          {
            "name": "AccountTempActiveBalanceLimitExceeded"
          },
          {
            "name": "AccountInFlashloan"
          },
          {
            "name": "IllegalFlashloan"
          },
          {
            "name": "IllegalFlag"
          },
          {
            "name": "IllegalBalanceState"
          },
          {
            "name": "IllegalAccountAuthorityTransfer"
          },
          {
            "name": "Unauthorized"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InsufficientFundsError",
      "msg": "Insufficient Funds"
    },
    {
      "code": 6001,
      "name": "TokenMintError",
      "msg": "Token Mint Error"
    },
    {
      "code": 6002,
      "name": "BackPriceError",
      "msg": "Get Back Price Error"
    },
    {
      "code": 6003,
      "name": "SpotPriceError",
      "msg": "Get Spot Price Error"
    },
    {
      "code": 6004,
      "name": "DeductionError",
      "msg": "Get Deduction Error"
    },
    {
      "code": 6005,
      "name": "TreasuryFundError",
      "msg": "Treasury Fund Error"
    },
    {
      "code": 6006,
      "name": "CollateralError",
      "msg": "Collateral Not in List Error"
    },
    {
      "code": 6007,
      "name": "BondNotFinished",
      "msg": "Bond Not finished"
    },
    {
      "code": 6008,
      "name": "CreatorError",
      "msg": "Not the Creator"
    },
    {
      "code": 6009,
      "name": "AlreadyRedeem",
      "msg": "Already Redeemed"
    },
    {
      "code": 6010,
      "name": "UnstakingError",
      "msg": "Not enough staked Saturn To Unstake"
    },
    {
      "code": 6011,
      "name": "IncorrectOwner",
      "msg": "IncorrectOwner"
    }
  ]
};

export const IDL: SaturnV1 = {
  "version": "0.1.0",
  "name": "saturn_v_1",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "applyBond",
      "accounts": [
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creatorTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryStfTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "priceUpdate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMintAddress",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stfTokenMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "ApplyBondArgs"
          }
        }
      ]
    },
    {
      "name": "finishBond",
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "destStfAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stfTokenMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "stakeStf",
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userStakeAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stfTokenMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amountToStake",
          "type": "u64"
        }
      ]
    },
    {
      "name": "unstakeStf",
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userStakeAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stfTokenMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amountToUnstake",
          "type": "u64"
        }
      ]
    },
    {
      "name": "swap",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wbtcTreasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wbtcMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "usdtTreasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdtMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "usdcTreasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdcMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "solMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "priceUpdate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "jupiterProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "data",
          "type": "bytes"
        },
        {
          "name": "fromAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "meteoraDeposit",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "user CHECK: this is pda"
          ]
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userPoolLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userAToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "dynamicAmmProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "poolTokenAmount",
          "type": "u64"
        },
        {
          "name": "maximumTokenAAmount",
          "type": "u64"
        },
        {
          "name": "maximumTokenBAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "meteoraWithdraw",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "user CHECK: this is pda"
          ]
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userPoolLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userAToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "dynamicAmmProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "poolTokenAmount",
          "type": "u64"
        },
        {
          "name": "maximumTokenAAmount",
          "type": "u64"
        },
        {
          "name": "maximumTokenBAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initLendingAccounts",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "klendProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "seed1Account",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "seed2Account",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lendingMarket",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "obligation",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marginfiAccount",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "klendLend",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "obligation",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lendingMarket",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lendingMarketAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reserveLiquiditySupply",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reserveCollateralMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reserveDestinationDepositCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userSourceLiquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userDestinationCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "instruction",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "klendProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "klendWithdraw",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userDestinationLiquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "instruction",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "klendProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "obligation",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lendingMarket",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "withdrawReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reserveSourceCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reserveCollateralMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reserveLiquiditySupply",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lendingMarketAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userDestinationCollateral",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "marginfiLend",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userLiquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bankLiquidityVault",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "marginfiWithdraw",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marginfiProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userLiquidity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bankLiquidityVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bankLiquidityVaultAuthority",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "getValueInMeteora",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pool",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userPoolLp",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": "u64"
    },
    {
      "name": "getValueInKamino",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lendingMarket",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "solReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdcReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdtReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wbtcReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wethReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bonkReserve",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "obligation",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": {
        "array": [
          "u64",
          6
        ]
      }
    },
    {
      "name": "getValueInMarginfi",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marginfiGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marginfiAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdcBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdtBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wbtcBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wethBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bonkBank",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": {
        "array": [
          "u64",
          6
        ]
      }
    },
    {
      "name": "calcuBalance",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "user CHECK: this is pda"
          ]
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdcTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wbtcTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "priceUpdate",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "reallocate",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "user CHECK: this is pda"
          ]
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sequenceFlag",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdcTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wbtcTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "priceUpdate",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "returnRate",
          "type": {
            "array": [
              "f64",
              7
            ]
          }
        },
        {
          "name": "riskRating",
          "type": {
            "array": [
              "f64",
              7
            ]
          }
        }
      ]
    },
    {
      "name": "cashingoutReedem",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeWalletTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryStfTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenMintAddress",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stfTokenMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "escrow",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "type": "publicKey"
          },
          {
            "name": "tokenMint",
            "type": "publicKey"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "startTimestamp",
            "type": "i64"
          },
          {
            "name": "endTimestamp",
            "type": "i64"
          },
          {
            "name": "numTokenToRedeem",
            "type": "u64"
          },
          {
            "name": "isFinished",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "sequenceFlag",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "flagCalcuBalance",
            "type": "bool"
          },
          {
            "name": "flagReallocate",
            "type": "bool"
          },
          {
            "name": "flagMarginfi",
            "type": "bool"
          },
          {
            "name": "flagKamino",
            "type": "bool"
          },
          {
            "name": "flagMeteora",
            "type": "bool"
          },
          {
            "name": "flagJupiter",
            "type": "bool"
          },
          {
            "name": "flagSwap",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "treasury",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "treasuryAdmin",
            "type": "publicKey"
          },
          {
            "name": "stakingIndex",
            "type": "u64"
          },
          {
            "name": "treasuryValue",
            "type": "u64"
          },
          {
            "name": "tokenMinted",
            "type": "u64"
          },
          {
            "name": "tokenStaked",
            "type": "u64"
          },
          {
            "name": "meteoraDepositAssets",
            "type": "u64"
          },
          {
            "name": "meteoraDepositValue",
            "type": "u64"
          },
          {
            "name": "meteoraAllocation",
            "type": "f64"
          },
          {
            "name": "kaminoLendAssets",
            "type": "u64"
          },
          {
            "name": "kaminoLendValue",
            "type": "u64"
          },
          {
            "name": "kaminoAllocation",
            "type": "f64"
          },
          {
            "name": "marginfiLendAssets",
            "type": "u64"
          },
          {
            "name": "marginfiLendValue",
            "type": "u64"
          },
          {
            "name": "marginfiAllocation",
            "type": "f64"
          },
          {
            "name": "jupiterPerpsValue",
            "type": "u64"
          },
          {
            "name": "jupiterAllocation",
            "type": "f64"
          },
          {
            "name": "usdcAllocation",
            "type": "f64"
          },
          {
            "name": "wbtcAllocation",
            "type": "f64"
          },
          {
            "name": "solAllocation",
            "type": "f64"
          }
        ]
      }
    },
    {
      "name": "userStakeAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalStakedIndex",
            "type": "u64"
          },
          {
            "name": "totalPoints",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "ApplyBondArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "spotPrice",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "VaultError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "VaultIsDisabled"
          },
          {
            "name": "ExceededSlippage"
          },
          {
            "name": "StrategyIsNotExisted"
          },
          {
            "name": "UnAuthorized"
          },
          {
            "name": "MathOverflow"
          },
          {
            "name": "ProtocolIsNotSupported"
          },
          {
            "name": "UnMatchReserve"
          },
          {
            "name": "InvalidLockedProfitDegradation"
          },
          {
            "name": "MaxStrategyReached"
          },
          {
            "name": "StrategyExisted"
          },
          {
            "name": "InvalidUnmintAmount"
          },
          {
            "name": "InvalidAccountsForStrategy"
          },
          {
            "name": "InvalidBump"
          },
          {
            "name": "AmountMustGreaterThanZero"
          },
          {
            "name": "MangoIsNotSupportedAnymore"
          },
          {
            "name": "StrategyIsNotSupported"
          },
          {
            "name": "PayAmountIsExeeced"
          }
        ]
      }
    },
    {
      "name": "MarginfiError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "MathError"
          },
          {
            "name": "BankNotFound"
          },
          {
            "name": "LendingAccountBalanceNotFound"
          },
          {
            "name": "BankAssetCapacityExceeded"
          },
          {
            "name": "InvalidTransfer"
          },
          {
            "name": "MissingPythOrBankAccount"
          },
          {
            "name": "MissingPythAccount"
          },
          {
            "name": "InvalidOracleAccount"
          },
          {
            "name": "MissingBankAccount"
          },
          {
            "name": "InvalidBankAccount"
          },
          {
            "name": "BadAccountHealth"
          },
          {
            "name": "LendingAccountBalanceSlotsFull"
          },
          {
            "name": "BankAlreadyExists"
          },
          {
            "name": "IllegalLiquidation"
          },
          {
            "name": "AccountNotBankrupt"
          },
          {
            "name": "BalanceNotBadDebt"
          },
          {
            "name": "InvalidConfig"
          },
          {
            "name": "StaleOracle"
          },
          {
            "name": "BankPaused"
          },
          {
            "name": "BankReduceOnly"
          },
          {
            "name": "BankAccoutNotFound"
          },
          {
            "name": "OperationDepositOnly"
          },
          {
            "name": "OperationWithdrawOnly"
          },
          {
            "name": "OperationBorrowOnly"
          },
          {
            "name": "OperationRepayOnly"
          },
          {
            "name": "NoAssetFound"
          },
          {
            "name": "NoLiabilityFound"
          },
          {
            "name": "InvalidOracleSetup"
          },
          {
            "name": "IllegalUtilizationRatio"
          },
          {
            "name": "BankLiabilityCapacityExceeded"
          },
          {
            "name": "InvalidPrice"
          },
          {
            "name": "IsolatedAccountIllegalState"
          },
          {
            "name": "EmissionsAlreadySetup"
          },
          {
            "name": "OracleNotSetup"
          },
          {
            "name": "InvalidSwitchboardDecimalConversion"
          },
          {
            "name": "CannotCloseOutstandingEmissions"
          },
          {
            "name": "EmissionsUpdateError"
          },
          {
            "name": "AccountDisabled"
          },
          {
            "name": "AccountTempActiveBalanceLimitExceeded"
          },
          {
            "name": "AccountInFlashloan"
          },
          {
            "name": "IllegalFlashloan"
          },
          {
            "name": "IllegalFlag"
          },
          {
            "name": "IllegalBalanceState"
          },
          {
            "name": "IllegalAccountAuthorityTransfer"
          },
          {
            "name": "Unauthorized"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InsufficientFundsError",
      "msg": "Insufficient Funds"
    },
    {
      "code": 6001,
      "name": "TokenMintError",
      "msg": "Token Mint Error"
    },
    {
      "code": 6002,
      "name": "BackPriceError",
      "msg": "Get Back Price Error"
    },
    {
      "code": 6003,
      "name": "SpotPriceError",
      "msg": "Get Spot Price Error"
    },
    {
      "code": 6004,
      "name": "DeductionError",
      "msg": "Get Deduction Error"
    },
    {
      "code": 6005,
      "name": "TreasuryFundError",
      "msg": "Treasury Fund Error"
    },
    {
      "code": 6006,
      "name": "CollateralError",
      "msg": "Collateral Not in List Error"
    },
    {
      "code": 6007,
      "name": "BondNotFinished",
      "msg": "Bond Not finished"
    },
    {
      "code": 6008,
      "name": "CreatorError",
      "msg": "Not the Creator"
    },
    {
      "code": 6009,
      "name": "AlreadyRedeem",
      "msg": "Already Redeemed"
    },
    {
      "code": 6010,
      "name": "UnstakingError",
      "msg": "Not enough staked Saturn To Unstake"
    },
    {
      "code": 6011,
      "name": "IncorrectOwner",
      "msg": "IncorrectOwner"
    }
  ]
};
