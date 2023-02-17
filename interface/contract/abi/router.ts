const ROUTER_CONTRACT = {
  source: {
    hash: '0xe407b08eb683e398d51e4d885962ec43e530b0926da1cb7d250de25706445fff',
    language: 'ink! 3.3.1',
    compiler: 'rustc 1.65.0-nightly',
  },
  contract: {
    name: 'router_contract',
    version: '0.1.0',
    authors: ['Shuhei Tanaka <shuhei46491@gmail.com>'],
  },
  V3: {
    spec: {
      constructors: [
        {
          args: [
            {
              label: 'factory',
              type: {
                displayName: ['AccountId'],
                type: 0,
              },
            },
            {
              label: 'wnative',
              type: {
                displayName: ['AccountId'],
                type: 0,
              },
            },
            {
              label: 'pair_code_hash',
              type: {
                displayName: ['Hash'],
                type: 3,
              },
            },
          ],
          docs: [],
          label: 'new',
          payable: false,
          selector: '0x9bae9d5e',
        },
      ],
      docs: [],
      events: [],
      messages: [
        {
          args: [
            {
              label: 'amount_out',
              type: {
                displayName: ['router_external', 'SwapTokensForExactTokensInput1'],
                type: 4,
              },
            },
            {
              label: 'amount_in_max',
              type: {
                displayName: ['router_external', 'SwapTokensForExactTokensInput2'],
                type: 4,
              },
            },
            {
              label: 'path',
              type: {
                displayName: ['router_external', 'SwapTokensForExactTokensInput3'],
                type: 5,
              },
            },
            {
              label: 'to',
              type: {
                displayName: ['router_external', 'SwapTokensForExactTokensInput4'],
                type: 0,
              },
            },
            {
              label: 'deadline',
              type: {
                displayName: ['router_external', 'SwapTokensForExactTokensInput5'],
                type: 6,
              },
            },
          ],
          docs: [],
          label: 'Router::swap_tokens_for_exact_tokens',
          mutates: true,
          payable: false,
          returnType: {
            displayName: ['router_external', 'SwapTokensForExactTokensOutput'],
            type: 7,
          },
          selector: '0xd8eafd67',
        },
        {
          args: [
            {
              label: 'amount_out',
              type: {
                displayName: ['router_external', 'SwapNativeForExactTokensInput1'],
                type: 4,
              },
            },
            {
              label: 'path',
              type: {
                displayName: ['router_external', 'SwapNativeForExactTokensInput2'],
                type: 5,
              },
            },
            {
              label: 'to',
              type: {
                displayName: ['router_external', 'SwapNativeForExactTokensInput3'],
                type: 0,
              },
            },
            {
              label: 'deadline',
              type: {
                displayName: ['router_external', 'SwapNativeForExactTokensInput4'],
                type: 6,
              },
            },
          ],
          docs: [],
          label: 'Router::swap_native_for_exact_tokens',
          mutates: true,
          payable: true,
          returnType: {
            displayName: ['router_external', 'SwapNativeForExactTokensOutput'],
            type: 7,
          },
          selector: '0x1299fdf2',
        },
        {
          args: [
            {
              label: 'token',
              type: {
                displayName: ['router_external', 'RemoveLiquidityNativeInput1'],
                type: 0,
              },
            },
            {
              label: 'liquidity',
              type: {
                displayName: ['router_external', 'RemoveLiquidityNativeInput2'],
                type: 4,
              },
            },
            {
              label: 'amount_token_min',
              type: {
                displayName: ['router_external', 'RemoveLiquidityNativeInput3'],
                type: 4,
              },
            },
            {
              label: 'amount_native_min',
              type: {
                displayName: ['router_external', 'RemoveLiquidityNativeInput4'],
                type: 4,
              },
            },
            {
              label: 'to',
              type: {
                displayName: ['router_external', 'RemoveLiquidityNativeInput5'],
                type: 0,
              },
            },
            {
              label: 'deadline',
              type: {
                displayName: ['router_external', 'RemoveLiquidityNativeInput6'],
                type: 6,
              },
            },
          ],
          docs: [],
          label: 'Router::remove_liquidity_native',
          mutates: true,
          payable: false,
          returnType: {
            displayName: ['router_external', 'RemoveLiquidityNativeOutput'],
            type: 19,
          },
          selector: '0x3448bb5c',
        },
        {
          args: [
            {
              label: 'amount_out',
              type: {
                displayName: ['router_external', 'SwapTokensForExactNativeInput1'],
                type: 4,
              },
            },
            {
              label: 'amount_in_max',
              type: {
                displayName: ['router_external', 'SwapTokensForExactNativeInput2'],
                type: 4,
              },
            },
            {
              label: 'path',
              type: {
                displayName: ['router_external', 'SwapTokensForExactNativeInput3'],
                type: 5,
              },
            },
            {
              label: 'to',
              type: {
                displayName: ['router_external', 'SwapTokensForExactNativeInput4'],
                type: 0,
              },
            },
            {
              label: 'deadline',
              type: {
                displayName: ['router_external', 'SwapTokensForExactNativeInput5'],
                type: 6,
              },
            },
          ],
          docs: [],
          label: 'Router::swap_tokens_for_exact_native',
          mutates: true,
          payable: false,
          returnType: {
            displayName: ['router_external', 'SwapTokensForExactNativeOutput'],
            type: 7,
          },
          selector: '0xb2b28f92',
        },
        {
          args: [
            {
              label: 'amount_in',
              type: {
                displayName: ['router_external', 'GetAmountOutInput1'],
                type: 4,
              },
            },
            {
              label: 'reserve_in',
              type: {
                displayName: ['router_external', 'GetAmountOutInput2'],
                type: 4,
              },
            },
            {
              label: 'reserve_out',
              type: {
                displayName: ['router_external', 'GetAmountOutInput3'],
                type: 4,
              },
            },
          ],
          docs: [],
          label: 'Router::get_amount_out',
          mutates: false,
          payable: false,
          returnType: {
            displayName: ['router_external', 'GetAmountOutOutput'],
            type: 21,
          },
          selector: '0x41e315fd',
        },
        {
          args: [
            {
              label: 'amount_out_min',
              type: {
                displayName: ['router_external', 'SwapExactNativeForTokensInput1'],
                type: 4,
              },
            },
            {
              label: 'path',
              type: {
                displayName: ['router_external', 'SwapExactNativeForTokensInput2'],
                type: 5,
              },
            },
            {
              label: 'to',
              type: {
                displayName: ['router_external', 'SwapExactNativeForTokensInput3'],
                type: 0,
              },
            },
            {
              label: 'deadline',
              type: {
                displayName: ['router_external', 'SwapExactNativeForTokensInput4'],
                type: 6,
              },
            },
          ],
          docs: [],
          label: 'Router::swap_exact_native_for_tokens',
          mutates: true,
          payable: true,
          returnType: {
            displayName: ['router_external', 'SwapExactNativeForTokensOutput'],
            type: 7,
          },
          selector: '0x0a78e251',
        },
        {
          args: [],
          docs: [],
          label: 'Router::factory',
          mutates: false,
          payable: false,
          returnType: {
            displayName: ['router_external', 'FactoryOutput'],
            type: 0,
          },
          selector: '0xac3a4c18',
        },
        {
          args: [
            {
              label: 'amount_in',
              type: {
                displayName: ['router_external', 'SwapExactTokensForNativeInput1'],
                type: 4,
              },
            },
            {
              label: 'amount_out_min',
              type: {
                displayName: ['router_external', 'SwapExactTokensForNativeInput2'],
                type: 4,
              },
            },
            {
              label: 'path',
              type: {
                displayName: ['router_external', 'SwapExactTokensForNativeInput3'],
                type: 5,
              },
            },
            {
              label: 'to',
              type: {
                displayName: ['router_external', 'SwapExactTokensForNativeInput4'],
                type: 0,
              },
            },
            {
              label: 'deadline',
              type: {
                displayName: ['router_external', 'SwapExactTokensForNativeInput5'],
                type: 6,
              },
            },
          ],
          docs: [],
          label: 'Router::swap_exact_tokens_for_native',
          mutates: true,
          payable: false,
          returnType: {
            displayName: ['router_external', 'SwapExactTokensForNativeOutput'],
            type: 7,
          },
          selector: '0xcb577423',
        },
        {
          args: [
            {
              label: 'token',
              type: {
                displayName: ['router_external', 'AddLiquidityNativeInput1'],
                type: 0,
              },
            },
            {
              label: 'amount_token_desired',
              type: {
                displayName: ['router_external', 'AddLiquidityNativeInput2'],
                type: 4,
              },
            },
            {
              label: 'amount_token_min',
              type: {
                displayName: ['router_external', 'AddLiquidityNativeInput3'],
                type: 4,
              },
            },
            {
              label: 'amount_native_min',
              type: {
                displayName: ['router_external', 'AddLiquidityNativeInput4'],
                type: 4,
              },
            },
            {
              label: 'to',
              type: {
                displayName: ['router_external', 'AddLiquidityNativeInput5'],
                type: 0,
              },
            },
            {
              label: 'deadline',
              type: {
                displayName: ['router_external', 'AddLiquidityNativeInput6'],
                type: 6,
              },
            },
          ],
          docs: [],
          label: 'Router::add_liquidity_native',
          mutates: true,
          payable: true,
          returnType: {
            displayName: ['router_external', 'AddLiquidityNativeOutput'],
            type: 22,
          },
          selector: '0x292d7221',
        },
        {
          args: [
            {
              label: 'amount_a',
              type: {
                displayName: ['router_external', 'QuoteInput1'],
                type: 4,
              },
            },
            {
              label: 'reserve_a',
              type: {
                displayName: ['router_external', 'QuoteInput2'],
                type: 4,
              },
            },
            {
              label: 'reserve_b',
              type: {
                displayName: ['router_external', 'QuoteInput3'],
                type: 4,
              },
            },
          ],
          docs: [],
          label: 'Router::quote',
          mutates: false,
          payable: false,
          returnType: {
            displayName: ['router_external', 'QuoteOutput'],
            type: 21,
          },
          selector: '0x16347b10',
        },
        {
          args: [],
          docs: [],
          label: 'Router::wnative',
          mutates: false,
          payable: false,
          returnType: {
            displayName: ['router_external', 'WnativeOutput'],
            type: 0,
          },
          selector: '0x5593eab6',
        },
        {
          args: [
            {
              label: 'token_a',
              type: {
                displayName: ['router_external', 'AddLiquidityInput1'],
                type: 0,
              },
            },
            {
              label: 'token_b',
              type: {
                displayName: ['router_external', 'AddLiquidityInput2'],
                type: 0,
              },
            },
            {
              label: 'amount_a_desired',
              type: {
                displayName: ['router_external', 'AddLiquidityInput3'],
                type: 4,
              },
            },
            {
              label: 'amount_b_desired',
              type: {
                displayName: ['router_external', 'AddLiquidityInput4'],
                type: 4,
              },
            },
            {
              label: 'amount_a_min',
              type: {
                displayName: ['router_external', 'AddLiquidityInput5'],
                type: 4,
              },
            },
            {
              label: 'amount_b_min',
              type: {
                displayName: ['router_external', 'AddLiquidityInput6'],
                type: 4,
              },
            },
            {
              label: 'to',
              type: {
                displayName: ['router_external', 'AddLiquidityInput7'],
                type: 0,
              },
            },
            {
              label: 'deadline',
              type: {
                displayName: ['router_external', 'AddLiquidityInput8'],
                type: 6,
              },
            },
          ],
          docs: [],
          label: 'Router::add_liquidity',
          mutates: true,
          payable: false,
          returnType: {
            displayName: ['router_external', 'AddLiquidityOutput'],
            type: 22,
          },
          selector: '0xa5b7d597',
        },
        {
          args: [
            {
              label: 'token_a',
              type: {
                displayName: ['router_external', 'RemoveLiquidityInput1'],
                type: 0,
              },
            },
            {
              label: 'token_b',
              type: {
                displayName: ['router_external', 'RemoveLiquidityInput2'],
                type: 0,
              },
            },
            {
              label: 'liquidity',
              type: {
                displayName: ['router_external', 'RemoveLiquidityInput3'],
                type: 4,
              },
            },
            {
              label: 'amount_a_min',
              type: {
                displayName: ['router_external', 'RemoveLiquidityInput4'],
                type: 4,
              },
            },
            {
              label: 'amount_b_min',
              type: {
                displayName: ['router_external', 'RemoveLiquidityInput5'],
                type: 4,
              },
            },
            {
              label: 'to',
              type: {
                displayName: ['router_external', 'RemoveLiquidityInput6'],
                type: 0,
              },
            },
            {
              label: 'deadline',
              type: {
                displayName: ['router_external', 'RemoveLiquidityInput7'],
                type: 6,
              },
            },
          ],
          docs: [],
          label: 'Router::remove_liquidity',
          mutates: true,
          payable: false,
          returnType: {
            displayName: ['router_external', 'RemoveLiquidityOutput'],
            type: 19,
          },
          selector: '0xd3abe5a3',
        },
        {
          args: [
            {
              label: 'amount_in',
              type: {
                displayName: ['router_external', 'SwapExactTokensForTokensInput1'],
                type: 4,
              },
            },
            {
              label: 'amount_out_min',
              type: {
                displayName: ['router_external', 'SwapExactTokensForTokensInput2'],
                type: 4,
              },
            },
            {
              label: 'path',
              type: {
                displayName: ['router_external', 'SwapExactTokensForTokensInput3'],
                type: 5,
              },
            },
            {
              label: 'to',
              type: {
                displayName: ['router_external', 'SwapExactTokensForTokensInput4'],
                type: 0,
              },
            },
            {
              label: 'deadline',
              type: {
                displayName: ['router_external', 'SwapExactTokensForTokensInput5'],
                type: 6,
              },
            },
          ],
          docs: [],
          label: 'Router::swap_exact_tokens_for_tokens',
          mutates: true,
          payable: false,
          returnType: {
            displayName: ['router_external', 'SwapExactTokensForTokensOutput'],
            type: 7,
          },
          selector: '0xaf0a8836',
        },
        {
          args: [
            {
              label: 'amount_out',
              type: {
                displayName: ['router_external', 'GetAmountInInput1'],
                type: 4,
              },
            },
            {
              label: 'reserve_in',
              type: {
                displayName: ['router_external', 'GetAmountInInput2'],
                type: 4,
              },
            },
            {
              label: 'reserve_out',
              type: {
                displayName: ['router_external', 'GetAmountInInput3'],
                type: 4,
              },
            },
          ],
          docs: [],
          label: 'Router::get_amount_in',
          mutates: false,
          payable: false,
          returnType: {
            displayName: ['router_external', 'GetAmountInOutput'],
            type: 21,
          },
          selector: '0xea4ac85d',
        },
        {
          args: [
            {
              label: 'amount_in',
              type: {
                displayName: ['router_external', 'GetAmountsOutInput1'],
                type: 4,
              },
            },
            {
              label: 'path',
              type: {
                displayName: ['router_external', 'GetAmountsOutInput2'],
                type: 5,
              },
            },
          ],
          docs: [],
          label: 'Router::get_amounts_out',
          mutates: false,
          payable: false,
          returnType: {
            displayName: ['router_external', 'GetAmountsOutOutput'],
            type: 7,
          },
          selector: '0x7170b8f6',
        },
        {
          args: [
            {
              label: 'amount_out',
              type: {
                displayName: ['router_external', 'GetAmountsInInput1'],
                type: 4,
              },
            },
            {
              label: 'path',
              type: {
                displayName: ['router_external', 'GetAmountsInInput2'],
                type: 5,
              },
            },
          ],
          docs: [],
          label: 'Router::get_amounts_in',
          mutates: false,
          payable: false,
          returnType: {
            displayName: ['router_external', 'GetAmountsInOutput'],
            type: 7,
          },
          selector: '0x707998fc',
        },
      ],
    },
    storage: {
      struct: {
        fields: [
          {
            layout: {
              struct: {
                fields: [
                  {
                    layout: {
                      cell: {
                        key: '0x7e833de500000000000000000000000000000000000000000000000000000000',
                        ty: 0,
                      },
                    },
                    name: 'factory',
                  },
                  {
                    layout: {
                      cell: {
                        key: '0x7f833de500000000000000000000000000000000000000000000000000000000',
                        ty: 0,
                      },
                    },
                    name: 'wnative',
                  },
                  {
                    layout: {
                      cell: {
                        key: '0x80833de500000000000000000000000000000000000000000000000000000000',
                        ty: 3,
                      },
                    },
                    name: 'pair_code_hash',
                  },
                ],
              },
            },
            name: 'router',
          },
        ],
      },
    },
    types: [
      {
        id: 0,
        type: {
          def: {
            composite: {
              fields: [
                {
                  type: 1,
                  typeName: '[u8; 32]',
                },
              ],
            },
          },
          path: ['ink_env', 'types', 'AccountId'],
        },
      },
      {
        id: 1,
        type: {
          def: {
            array: {
              len: 32,
              type: 2,
            },
          },
        },
      },
      {
        id: 2,
        type: {
          def: {
            primitive: 'u8',
          },
        },
      },
      {
        id: 3,
        type: {
          def: {
            composite: {
              fields: [
                {
                  type: 1,
                  typeName: '[u8; 32]',
                },
              ],
            },
          },
          path: ['ink_env', 'types', 'Hash'],
        },
      },
      {
        id: 4,
        type: {
          def: {
            primitive: 'u128',
          },
        },
      },
      {
        id: 5,
        type: {
          def: {
            sequence: {
              type: 0,
            },
          },
        },
      },
      {
        id: 6,
        type: {
          def: {
            primitive: 'u64',
          },
        },
      },
      {
        id: 7,
        type: {
          def: {
            variant: {
              variants: [
                {
                  fields: [
                    {
                      type: 8,
                    },
                  ],
                  index: 0,
                  name: 'Ok',
                },
                {
                  fields: [
                    {
                      type: 9,
                    },
                  ],
                  index: 1,
                  name: 'Err',
                },
              ],
            },
          },
          params: [
            {
              name: 'T',
              type: 8,
            },
            {
              name: 'E',
              type: 9,
            },
          ],
          path: ['Result'],
        },
      },
      {
        id: 8,
        type: {
          def: {
            sequence: {
              type: 4,
            },
          },
        },
      },
      {
        id: 9,
        type: {
          def: {
            variant: {
              variants: [
                {
                  fields: [
                    {
                      type: 10,
                      typeName: 'PSP22Error',
                    },
                  ],
                  index: 0,
                  name: 'PSP22Error',
                },
                {
                  fields: [
                    {
                      type: 12,
                      typeName: 'FactoryError',
                    },
                  ],
                  index: 1,
                  name: 'FactoryError',
                },
                {
                  fields: [
                    {
                      type: 13,
                      typeName: 'PairError',
                    },
                  ],
                  index: 2,
                  name: 'PairError',
                },
                {
                  fields: [
                    {
                      type: 17,
                      typeName: 'HelperError',
                    },
                  ],
                  index: 3,
                  name: 'HelperError',
                },
                {
                  fields: [
                    {
                      type: 18,
                      typeName: 'TransferHelperError',
                    },
                  ],
                  index: 4,
                  name: 'TransferHelperError',
                },
                {
                  index: 5,
                  name: 'PairNotFound',
                },
                {
                  index: 6,
                  name: 'InsufficientAmount',
                },
                {
                  index: 7,
                  name: 'InsufficientAAmount',
                },
                {
                  index: 8,
                  name: 'InsufficientOutputAmount',
                },
                {
                  index: 9,
                  name: 'ExcessiveInputAmount',
                },
                {
                  index: 10,
                  name: 'InsufficientBAmount',
                },
                {
                  index: 11,
                  name: 'InsufficientLiquidity',
                },
                {
                  index: 12,
                  name: 'ZeroAddress',
                },
                {
                  index: 13,
                  name: 'IdenticalAddresses',
                },
                {
                  index: 14,
                  name: 'Expired',
                },
                {
                  index: 15,
                  name: 'SubUnderFlow',
                },
                {
                  index: 16,
                  name: 'MulOverFlow',
                },
                {
                  index: 17,
                  name: 'DivByZero',
                },
                {
                  index: 18,
                  name: 'TransferFailed',
                },
                {
                  index: 19,
                  name: 'InvalidPath',
                },
              ],
            },
          },
          path: ['uniswap_v2', 'traits', 'router', 'RouterError'],
        },
      },
      {
        id: 10,
        type: {
          def: {
            variant: {
              variants: [
                {
                  fields: [
                    {
                      type: 11,
                      typeName: 'String',
                    },
                  ],
                  index: 0,
                  name: 'Custom',
                },
                {
                  index: 1,
                  name: 'InsufficientBalance',
                },
                {
                  index: 2,
                  name: 'InsufficientAllowance',
                },
                {
                  index: 3,
                  name: 'ZeroRecipientAddress',
                },
                {
                  index: 4,
                  name: 'ZeroSenderAddress',
                },
                {
                  fields: [
                    {
                      type: 11,
                      typeName: 'String',
                    },
                  ],
                  index: 5,
                  name: 'SafeTransferCheckFailed',
                },
              ],
            },
          },
          path: ['openbrush_contracts', 'traits', 'errors', 'psp22', 'PSP22Error'],
        },
      },
      {
        id: 11,
        type: {
          def: {
            primitive: 'str',
          },
        },
      },
      {
        id: 12,
        type: {
          def: {
            variant: {
              variants: [
                {
                  fields: [
                    {
                      type: 13,
                      typeName: 'PairError',
                    },
                  ],
                  index: 0,
                  name: 'PairError',
                },
                {
                  index: 1,
                  name: 'CallerIsNotFeeSetter',
                },
                {
                  index: 2,
                  name: 'ZeroAddress',
                },
                {
                  index: 3,
                  name: 'IdenticalAddresses',
                },
                {
                  index: 4,
                  name: 'PairExists',
                },
              ],
            },
          },
          path: ['uniswap_v2', 'traits', 'factory', 'FactoryError'],
        },
      },
      {
        id: 13,
        type: {
          def: {
            variant: {
              variants: [
                {
                  fields: [
                    {
                      type: 10,
                      typeName: 'PSP22Error',
                    },
                  ],
                  index: 0,
                  name: 'PSP22Error',
                },
                {
                  fields: [
                    {
                      type: 14,
                      typeName: 'OwnableError',
                    },
                  ],
                  index: 1,
                  name: 'OwnableError',
                },
                {
                  fields: [
                    {
                      type: 15,
                      typeName: 'PausableError',
                    },
                  ],
                  index: 2,
                  name: 'PausableError',
                },
                {
                  fields: [
                    {
                      type: 16,
                      typeName: 'ReentrancyGuardError',
                    },
                  ],
                  index: 3,
                  name: 'ReentrancyGuardError',
                },
                {
                  index: 4,
                  name: 'K',
                },
                {
                  index: 5,
                  name: 'InsufficientLiquidityMinted',
                },
                {
                  index: 6,
                  name: 'InsufficientLiquidityBurned',
                },
                {
                  index: 7,
                  name: 'InsufficientOutputAmount',
                },
                {
                  index: 8,
                  name: 'InsufficientLiquidity',
                },
                {
                  index: 9,
                  name: 'InsufficientInputAmount',
                },
                {
                  index: 10,
                  name: 'SafeTransferFailed',
                },
                {
                  index: 11,
                  name: 'InvalidTo',
                },
                {
                  index: 12,
                  name: 'Overflow',
                },
                {
                  index: 13,
                  name: 'Locked',
                },
                {
                  index: 14,
                  name: 'SubUnderFlow1',
                },
                {
                  index: 15,
                  name: 'SubUnderFlow2',
                },
                {
                  index: 16,
                  name: 'SubUnderFlow3',
                },
                {
                  index: 17,
                  name: 'SubUnderFlow4',
                },
                {
                  index: 18,
                  name: 'SubUnderFlow5',
                },
                {
                  index: 19,
                  name: 'SubUnderFlow6',
                },
                {
                  index: 20,
                  name: 'SubUnderFlow7',
                },
                {
                  index: 21,
                  name: 'SubUnderFlow8',
                },
                {
                  index: 22,
                  name: 'SubUnderFlow9',
                },
                {
                  index: 23,
                  name: 'SubUnderFlow10',
                },
                {
                  index: 24,
                  name: 'SubUnderFlow11',
                },
                {
                  index: 25,
                  name: 'SubUnderFlow12',
                },
                {
                  index: 26,
                  name: 'SubUnderFlow13',
                },
                {
                  index: 27,
                  name: 'SubUnderFlow14',
                },
                {
                  index: 28,
                  name: 'MulOverFlow1',
                },
                {
                  index: 29,
                  name: 'MulOverFlow2',
                },
                {
                  index: 30,
                  name: 'MulOverFlow3',
                },
                {
                  index: 31,
                  name: 'MulOverFlow4',
                },
                {
                  index: 32,
                  name: 'MulOverFlow5',
                },
                {
                  index: 33,
                  name: 'MulOverFlow6',
                },
                {
                  index: 34,
                  name: 'MulOverFlow7',
                },
                {
                  index: 35,
                  name: 'MulOverFlow8',
                },
                {
                  index: 36,
                  name: 'MulOverFlow9',
                },
                {
                  index: 37,
                  name: 'MulOverFlow10',
                },
                {
                  index: 38,
                  name: 'MulOverFlow11',
                },
                {
                  index: 39,
                  name: 'MulOverFlow12',
                },
                {
                  index: 40,
                  name: 'MulOverFlow13',
                },
                {
                  index: 41,
                  name: 'MulOverFlow14',
                },
                {
                  index: 42,
                  name: 'DivByZero1',
                },
                {
                  index: 43,
                  name: 'DivByZero2',
                },
                {
                  index: 44,
                  name: 'DivByZero3',
                },
                {
                  index: 45,
                  name: 'DivByZero4',
                },
                {
                  index: 46,
                  name: 'DivByZero5',
                },
                {
                  index: 47,
                  name: 'AddOverflow1',
                },
                {
                  index: 48,
                  name: 'CastOverflow1',
                },
                {
                  index: 49,
                  name: 'CastOverflow2',
                },
              ],
            },
          },
          path: ['uniswap_v2', 'traits', 'pair', 'PairError'],
        },
      },
      {
        id: 14,
        type: {
          def: {
            variant: {
              variants: [
                {
                  index: 0,
                  name: 'CallerIsNotOwner',
                },
                {
                  index: 1,
                  name: 'NewOwnerIsZero',
                },
              ],
            },
          },
          path: ['openbrush_contracts', 'traits', 'errors', 'ownable', 'OwnableError'],
        },
      },
      {
        id: 15,
        type: {
          def: {
            variant: {
              variants: [
                {
                  index: 0,
                  name: 'Paused',
                },
                {
                  index: 1,
                  name: 'NotPaused',
                },
              ],
            },
          },
          path: ['openbrush_contracts', 'traits', 'errors', 'pausable', 'PausableError'],
        },
      },
      {
        id: 16,
        type: {
          def: {
            variant: {
              variants: [
                {
                  index: 0,
                  name: 'ReentrantCall',
                },
              ],
            },
          },
          path: ['openbrush_contracts', 'traits', 'errors', 'reentrancy_guard', 'ReentrancyGuardError'],
        },
      },
      {
        id: 17,
        type: {
          def: {
            variant: {
              variants: [
                {
                  index: 0,
                  name: 'IdenticalAddresses',
                },
                {
                  index: 1,
                  name: 'ZeroAddress',
                },
                {
                  index: 2,
                  name: 'InsufficientAmount',
                },
                {
                  index: 3,
                  name: 'InsufficientLiquidity',
                },
                {
                  index: 4,
                  name: 'DivByZero',
                },
                {
                  index: 5,
                  name: 'CastOverflow',
                },
                {
                  index: 6,
                  name: 'MulOverFlow',
                },
                {
                  index: 7,
                  name: 'AddOverFlow',
                },
                {
                  index: 8,
                  name: 'DivByZero2',
                },
                {
                  index: 9,
                  name: 'CastOverflow2',
                },
                {
                  index: 10,
                  name: 'InvalidPath',
                },
                {
                  index: 11,
                  name: 'SubUnderFlow',
                },
              ],
            },
          },
          path: ['uniswap_v2', 'helpers', 'helper', 'HelperError'],
        },
      },
      {
        id: 18,
        type: {
          def: {
            variant: {
              variants: [
                {
                  index: 0,
                  name: 'TransferFailed',
                },
              ],
            },
          },
          path: ['uniswap_v2', 'helpers', 'transfer_helper', 'TransferHelperError'],
        },
      },
      {
        id: 19,
        type: {
          def: {
            variant: {
              variants: [
                {
                  fields: [
                    {
                      type: 20,
                    },
                  ],
                  index: 0,
                  name: 'Ok',
                },
                {
                  fields: [
                    {
                      type: 9,
                    },
                  ],
                  index: 1,
                  name: 'Err',
                },
              ],
            },
          },
          params: [
            {
              name: 'T',
              type: 20,
            },
            {
              name: 'E',
              type: 9,
            },
          ],
          path: ['Result'],
        },
      },
      {
        id: 20,
        type: {
          def: {
            tuple: [4, 4],
          },
        },
      },
      {
        id: 21,
        type: {
          def: {
            variant: {
              variants: [
                {
                  fields: [
                    {
                      type: 4,
                    },
                  ],
                  index: 0,
                  name: 'Ok',
                },
                {
                  fields: [
                    {
                      type: 9,
                    },
                  ],
                  index: 1,
                  name: 'Err',
                },
              ],
            },
          },
          params: [
            {
              name: 'T',
              type: 4,
            },
            {
              name: 'E',
              type: 9,
            },
          ],
          path: ['Result'],
        },
      },
      {
        id: 22,
        type: {
          def: {
            variant: {
              variants: [
                {
                  fields: [
                    {
                      type: 23,
                    },
                  ],
                  index: 0,
                  name: 'Ok',
                },
                {
                  fields: [
                    {
                      type: 9,
                    },
                  ],
                  index: 1,
                  name: 'Err',
                },
              ],
            },
          },
          params: [
            {
              name: 'T',
              type: 23,
            },
            {
              name: 'E',
              type: 9,
            },
          ],
          path: ['Result'],
        },
      },
      {
        id: 23,
        type: {
          def: {
            tuple: [4, 4, 4],
          },
        },
      },
    ],
  },
}

export default ROUTER_CONTRACT
