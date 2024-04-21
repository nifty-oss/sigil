#!/usr/bin/env zx
import "zx/globals";
import * as k from "@metaplex-foundation/kinobi";
import { getAllProgramIdls } from "./utils.mjs";

// Instanciate Kinobi.
const kinobi = k.createFromIdls(getAllProgramIdls());

// Update programs.
kinobi.update(
  k.updateProgramsVisitor({
    niftyOssTokenLite: { name: "tokenLite" }
  })
);

kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: "[programNode]tokenLite",
      transform: (node) => {
        k.assertIsNode(node, "programNode");
        return {
          ...node,
          accounts: [
            ...node.accounts,
            // Token Account
            k.accountNode({
              name: "tokenAccount",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "namespace",
                  type: k.publicKeyTypeNode()
                }),
                k.structFieldTypeNode({
                  name: "user",
                  type: k.publicKeyTypeNode()
                })
              ])
            })
          ]
        };
      }
    }
  ])
);

// Update accounts.
kinobi.update(
  k.updateAccountsVisitor({
    tokenAccount: {
      seeds: [
        k.constantPdaSeedNodeFromString("token_account"),
        k.variablePdaSeedNode(
          "user",
          k.publicKeyTypeNode(),
          "The user of the token account"
        ),
        k.variablePdaSeedNode(
          "namespace",
          k.publicKeyTypeNode(),
          "The namespace of the token account"
        )
      ]
    },
    mintAccount: {
      seeds: [
        k.constantPdaSeedNodeFromString("token_account"),
        k.variablePdaSeedNode(
          "authority",
          k.publicKeyTypeNode(),
          "The authority of the token account"
        ),
        k.variablePdaSeedNode(
          "namespace",
          k.publicKeyTypeNode(),
          "The namespace of the token account"
        )
      ]
    }
  })
);

// Render JavaScript.
const jsClient = path.join(__dirname, "..", "clients", "js");
kinobi.accept(
  k.renderJavaScriptExperimentalVisitor(
    path.join(jsClient, "src", "generated"),
    { prettier: require(path.join(jsClient, ".prettierrc.json")) }
  )
);

// Render Rust.
const rustClient = path.join(__dirname, "..", "clients", "rust");
kinobi.accept(
  k.renderRustVisitor(path.join(rustClient, "src", "generated"), {
    formatCode: true,
    crateFolder: rustClient
  })
);
