export type Saturn = {
  "version": "0.1.0",
  "name": "saturn_v1",
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
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "createrTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "destTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "destStfAccount",
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
          "name": "tokenAmount",
          "type": "u64"
        },
        {
          "name": "spotPrice",
          "type": "u64"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "finishBond",
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
      "args": [
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "Escrow",
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
      "name": "Treasury",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "treasury",
            "type": "publicKey"
          },
          {
            "name": "sstf",
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
    }
  ],
  "metadata": {
    "address": "GSQceBxFJCJBd4Wo5enf9GB4Qr1nXcXjfATbMR8EUqmK"
  }
}

export const IDL: Saturn = {
  "version": "0.1.0",
  "name": "saturn_v1",
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
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "createrTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "destTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "destStfAccount",
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
          "name": "tokenAmount",
          "type": "u64"
        },
        {
          "name": "spotPrice",
          "type": "u64"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "finishBond",
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
      "args": [
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "Escrow",
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
      "name": "Treasury",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "treasury",
            "type": "publicKey"
          },
          {
            "name": "sstf",
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
    }
  ],
  "metadata": {
    "address": "GSQceBxFJCJBd4Wo5enf9GB4Qr1nXcXjfATbMR8EUqmK"
  }
}