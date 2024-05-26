#!/usr/bin/env zx
import "zx/globals";
import * as k from "@metaplex-foundation/kinobi";
import { getAllProgramIdls } from "./utils.mjs";

// Instanciate Kinobi.
const kinobi = k.createFromIdls(getAllProgramIdls());

// Update programs.
kinobi.update(
  k.updateProgramsVisitor({
    sigilProgram: { name: "sigil" },
  })
);

kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: "[programNode]sigil",
      transform: (node) => {
        k.assertIsNode(node, "programNode");
        return {
          ...node,
          accounts: [
            ...node.accounts,
            k.accountNode({
              name: "mint",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "tag",
                  type: k.definedTypeLinkNode("Tag"),
                }),
                k.structFieldTypeNode({
                  name: "bump",
                  type: k.numberTypeNode("u8"),
                }),
                k.structFieldTypeNode({
                  name: "decimals",
                  type: k.numberTypeNode("u8"),
                }),
                k.structFieldTypeNode({
                  name: "empty",
                  type: k.numberTypeNode("u8"),
                }),
                k.structFieldTypeNode({
                  name: "ticker",
                  type: k.stringTypeNode({
                    size: k.fixedSizeNode(4),
                  }),
                }),
                k.structFieldTypeNode({
                  name: "authority",
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "supply",
                  type: k.numberTypeNode("u64"),
                }),
                k.structFieldTypeNode({
                  name: "maxSupply",
                  type: k.numberTypeNode("u64"),
                }),
              ]),
            }),
            k.accountNode({
              name: "pocket",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "tag",
                  type: k.definedTypeLinkNode("Tag"),
                }),
                k.structFieldTypeNode({
                  name: "padding",
                  type: k.numberTypeNode("u8"),
                }),
                k.structFieldTypeNode({
                  name: "authority",
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "user",
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "tokens",
                  type: k.arrayTypeNode(
                    k.definedTypeLinkNode("token"),
                    k.prefixedSizeNode(k.numberTypeNode("u16"))
                  ),
                }),
              ]),
            }),
          ],
          definedTypes: [
            ...node.definedTypes,
            k.definedTypeNode({
              name: "token",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "ticker",
                  type: k.stringTypeNode({
                    size: k.fixedSizeNode(4),
                  }),
                }),
                k.structFieldTypeNode({
                  name: "amount",
                  type: k.numberTypeNode("u32"),
                }),
              ]),
            }),
            k.definedTypeNodeFromIdl({
              name: "tag",
              type: {
                kind: "enum",
                variants: [
                  { name: "Uninitialized" },
                  { name: "Mint" },
                  { name: "Pocket" },
                ],
              },
            }),
          ],
        };
      },
    },
  ])
);

// Update accounts.
kinobi.update(
  k.updateAccountsVisitor({
    pocket: {
      size: 68,
      seeds: [
        k.constantPdaSeedNodeFromString("pocket"),
        k.variablePdaSeedNode(
          "authority",
          k.publicKeyTypeNode(),
          "The authority of the token account"
        ),
        k.variablePdaSeedNode(
          "user",
          k.publicKeyTypeNode(),
          "The user of the token account"
        ),
      ],
    },
    mint: {
      size: 56,
      seeds: [
        k.constantPdaSeedNodeFromString("mint"),
        k.variablePdaSeedNode(
          "authority",
          k.publicKeyTypeNode(),
          "The authority of the mint account"
        ),
        k.variablePdaSeedNode("ticker", k.bytesTypeNode(k.fixedSizeNode(4))),
      ],
    },
  })
);

kinobi.update(
  k.updateInstructionsVisitor({
    transfer: {},
  })
);

// Render JavaScript.
const jsClient = path.join(__dirname, "..", "clients", "js");
kinobi.accept(
  k.renderJavaScriptExperimentalVisitor(
    path.join(jsClient, "src", "generated"),
    {
      prettier: require(path.join(jsClient, ".prettierrc.json")),
    }
  )
);

// Render Rust.
const rustClient = path.join(__dirname, "..", "clients", "rust");
kinobi.accept(
  k.renderRustVisitor(path.join(rustClient, "src", "generated"), {
    formatCode: true,
    crateFolder: rustClient,
  })
);
