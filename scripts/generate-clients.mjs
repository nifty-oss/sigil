#!/usr/bin/env zx
import 'zx/globals';
import * as k from '@metaplex-foundation/kinobi';
import { getAllProgramIdls } from './utils.mjs';

// Instanciate Kinobi.
const kinobi = k.createFromIdls(getAllProgramIdls());

// Update programs.
kinobi.update(
  k.updateProgramsVisitor({
    niftyOssTokenLite: { name: 'tokenLite' },
  }),
);

kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: '[programNode]tokenLite',
      transform: (node) => {
        k.assertIsNode(node, 'programNode');
        return {
          ...node,
          accounts: [
            ...node.accounts,
            k.accountNode({
              name: 'mint',
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: 'authority',
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: 'ticker',
                  type: k.stringTypeNode({
                    size: k.fixedSizeNode(4),
                  }),
                }),
                k.structFieldTypeNode({
                  name: 'decimals',
                  type: k.numberTypeNode('u32'),
                }),
                k.structFieldTypeNode({
                  name: 'supply',
                  type: k.numberTypeNode('u64'),
                }),
                k.structFieldTypeNode({
                  name: 'maxSupply',
                  type: k.numberTypeNode('u64'),
                }),
              ]),
            }),
            k.accountNode({
              name: 'tokenAccount',
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: 'authority',
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: 'user',
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: 'tree',
                  type: k.definedTypeLinkNode('tree'),
                }),
              ]),
            }),
          ],
          definedTypes: [
            ...node.definedTypes,
            k.definedTypeNode({
              name: 'tree',
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: 'allocator',
                  type: k.arrayTypeNode(k.numberTypeNode('u8'), k.fixedSizeNode(8)),
                }),
                k.structFieldTypeNode({
                  name: 'nodes',
                  type: k.arrayTypeNode(k.definedTypeLinkNode('node'), k.remainderSizeNode()),
                }),
              ]),
            }),
            k.definedTypeNode({
              name: 'node',
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: 'pointer',
                  type: k.arrayTypeNode(k.numberTypeNode('u8'), k.fixedSizeNode(4)),
                }),
                k.structFieldTypeNode({
                  name: 'ticker',
                  type: k.stringTypeNode({
                    size: k.fixedSizeNode(4),
                  }),
                }),
                k.structFieldTypeNode({
                  name: 'amount',
                  type: k.numberTypeNode('u32'),
                }),
              ]),
            }),
          ],
        };
      },
    },
  ]),
);

// Update accounts.
kinobi.update(
  k.updateAccountsVisitor({
    tokenAccount: {
      seeds: [
        k.constantPdaSeedNodeFromString('token_account'),
        k.variablePdaSeedNode('user', k.publicKeyTypeNode(), 'The user of the token account'),
        k.variablePdaSeedNode(
          'authority',
          k.publicKeyTypeNode(),
          'The authority of the token account',
        ),
      ],
    },
    mint: {
      seeds: [
        k.constantPdaSeedNodeFromString('mint'),
        k.variablePdaSeedNode(
          'ticker',
          k.arrayTypeNode(k.numberTypeNode('u8'), k.fixedSizeNode(4)),
          'The ticker of the mint',
        ),
        k.variablePdaSeedNode(
          'authority',
          k.publicKeyTypeNode(),
          'The authority of the mint account',
        ),
      ],
    },
  }),
);

// Render JavaScript.
const jsClient = path.join(__dirname, '..', 'clients', 'js');
kinobi.accept(
  k.renderJavaScriptExperimentalVisitor(path.join(jsClient, 'src', 'generated'), {
    prettier: require(path.join(jsClient, '.prettierrc.json')),
  }),
);

// Render Rust.
const rustClient = path.join(__dirname, '..', 'clients', 'rust');
kinobi.accept(
  k.renderRustVisitor(path.join(rustClient, 'src', 'generated'), {
    formatCode: true,
    crateFolder: rustClient,
  }),
);
