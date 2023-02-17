export const ERC20_ADDRESS = 'bGX6n89QaDLr1mfXtpK7n2b6CqpXn2DsfH7vdZH2FmV5PmZ'
export const ERC20 = {
  source: {
    hash: '0xfc8f1123836e2fd811a79ccd40855cfeb2c73c0e8aa4c99a8ec995a480e22c8a',
    language: 'ink! 3.3.1',
    compiler: 'rustc 1.65.0-nightly',
  },
  contract: {
    name: 'erc20',
    version: '0.1.0',
    authors: ['[your_name] <[your_email]>'],
  },
  V3: {
    spec: {
      constructors: [
        {
          args: [
            {
              label: 'initial_supply',
              type: {
                displayName: ['Balance'],
                type: 0,
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
      events: [
        {
          args: [
            {
              docs: [],
              indexed: true,
              label: 'from',
              type: {
                displayName: ['Option'],
                type: 9,
              },
            },
            {
              docs: [],
              indexed: true,
              label: 'to',
              type: {
                displayName: ['Option'],
                type: 9,
              },
            },
            {
              docs: [],
              indexed: true,
              label: 'value',
              type: {
                displayName: ['Balance'],
                type: 0,
              },
            },
          ],
          docs: [],
          label: 'Transfer',
        },
        {
          args: [
            {
              docs: [],
              indexed: true,
              label: 'owner',
              type: {
                displayName: ['AccountId'],
                type: 2,
              },
            },
            {
              docs: [],
              indexed: true,
              label: 'spender',
              type: {
                displayName: ['AccountId'],
                type: 2,
              },
            },
            {
              docs: [],
              indexed: true,
              label: 'value',
              type: {
                displayName: ['Balance'],
                type: 0,
              },
            },
          ],
          docs: [],
          label: 'Approval',
        },
      ],
      messages: [
        {
          args: [],
          docs: [],
          label: 'total_supply',
          mutates: false,
          payable: false,
          returnType: {
            displayName: ['Balance'],
            type: 0,
          },
          selector: '0xdb6375a8',
        },
        {
          args: [
            {
              label: 'owner',
              type: {
                displayName: ['AccountId'],
                type: 2,
              },
            },
          ],
          docs: [],
          label: 'balance_of',
          mutates: false,
          payable: false,
          returnType: {
            displayName: ['Balance'],
            type: 0,
          },
          selector: '0x0f755a56',
        },
        {
          args: [
            {
              label: 'spender',
              type: {
                displayName: ['AccountId'],
                type: 2,
              },
            },
            {
              label: 'value',
              type: {
                displayName: ['Balance'],
                type: 0,
              },
            },
          ],
          docs: [],
          label: 'approve',
          mutates: true,
          payable: false,
          returnType: {
            displayName: ['bool'],
            type: 8,
          },
          selector: '0x681266a0',
        },
        {
          args: [
            {
              label: 'owner',
              type: {
                displayName: ['AccountId'],
                type: 2,
              },
            },
            {
              label: 'spender',
              type: {
                displayName: ['AccountId'],
                type: 2,
              },
            },
          ],
          docs: [],
          label: 'allowance',
          mutates: false,
          payable: false,
          returnType: {
            displayName: ['Balance'],
            type: 0,
          },
          selector: '0x6a00165e',
        },
        {
          args: [
            {
              label: 'from',
              type: {
                displayName: ['AccountId'],
                type: 2,
              },
            },
            {
              label: 'to',
              type: {
                displayName: ['AccountId'],
                type: 2,
              },
            },
            {
              label: 'value',
              type: {
                displayName: ['Balance'],
                type: 0,
              },
            },
          ],
          docs: [],
          label: 'transfer_from',
          mutates: true,
          payable: false,
          returnType: {
            displayName: ['bool'],
            type: 8,
          },
          selector: '0x0b396f18',
        },
        {
          args: [
            {
              label: 'to',
              type: {
                displayName: ['AccountId'],
                type: 2,
              },
            },
            {
              label: 'value',
              type: {
                displayName: ['Balance'],
                type: 0,
              },
            },
          ],
          docs: [],
          label: 'transfer',
          mutates: true,
          payable: false,
          returnType: {
            displayName: ['bool'],
            type: 8,
          },
          selector: '0x84a15da1',
        },
      ],
    },
    storage: {
      struct: {
        fields: [
          {
            layout: {
              cell: {
                key: '0x0000000000000000000000000000000000000000000000000000000000000000',
                ty: 0,
              },
            },
            name: 'total_supply',
          },
          {
            layout: {
              cell: {
                key: '0x0100000000000000000000000000000000000000000000000000000000000000',
                ty: 1,
              },
            },
            name: 'balances',
          },
          {
            layout: {
              cell: {
                key: '0x0200000000000000000000000000000000000000000000000000000000000000',
                ty: 6,
              },
            },
            name: 'allowances',
          },
        ],
      },
    },
    types: [
      {
        id: 0,
        type: {
          def: {
            primitive: 'u128',
          },
        },
      },
      {
        id: 1,
        type: {
          def: {
            composite: {
              fields: [
                {
                  name: 'offset_key',
                  type: 5,
                  typeName: 'Key',
                },
              ],
            },
          },
          params: [
            {
              name: 'K',
              type: 2,
            },
            {
              name: 'V',
              type: 0,
            },
          ],
          path: ['ink_storage', 'lazy', 'mapping', 'Mapping'],
        },
      },
      {
        id: 2,
        type: {
          def: {
            composite: {
              fields: [
                {
                  type: 3,
                  typeName: '[u8; 32]',
                },
              ],
            },
          },
          path: ['ink_env', 'types', 'AccountId'],
        },
      },
      {
        id: 3,
        type: {
          def: {
            array: {
              len: 32,
              type: 4,
            },
          },
        },
      },
      {
        id: 4,
        type: {
          def: {
            primitive: 'u8',
          },
        },
      },
      {
        id: 5,
        type: {
          def: {
            composite: {
              fields: [
                {
                  type: 3,
                  typeName: '[u8; 32]',
                },
              ],
            },
          },
          path: ['ink_primitives', 'Key'],
        },
      },
      {
        id: 6,
        type: {
          def: {
            composite: {
              fields: [
                {
                  name: 'offset_key',
                  type: 5,
                  typeName: 'Key',
                },
              ],
            },
          },
          params: [
            {
              name: 'K',
              type: 7,
            },
            {
              name: 'V',
              type: 0,
            },
          ],
          path: ['ink_storage', 'lazy', 'mapping', 'Mapping'],
        },
      },
      {
        id: 7,
        type: {
          def: {
            tuple: [2, 2],
          },
        },
      },
      {
        id: 8,
        type: {
          def: {
            primitive: 'bool',
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
                  index: 0,
                  name: 'None',
                },
                {
                  fields: [
                    {
                      type: 2,
                    },
                  ],
                  index: 1,
                  name: 'Some',
                },
              ],
            },
          },
          params: [
            {
              name: 'T',
              type: 2,
            },
          ],
          path: ['Option'],
        },
      },
    ],
  },
}
