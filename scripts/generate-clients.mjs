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

// Update accounts.
kinobi.update(
  k.updateAccountsVisitor({
    tokenAccount: {
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

// Update instructions.
// kinobi.update(
//   k.updateInstructionsVisitor({
//     create: {
//       byteDeltas: [k.instructionByteDeltaNode(k.accountLinkNode("counter"))],
//       accounts: {
//         counter: { defaultValue: k.pdaValueNode("counter") },
//         payer: { defaultValue: k.accountValueNode("authority") }
//       }
//     }
//   })
// );

// Set account discriminators.
// const key = (name) => ({ field: "key", value: k.enumValueNode("Key", name) });
// kinobi.update(
//   k.setAccountDiscriminatorFromFieldVisitor({
//     tokenAccount: key("token_account")
//   })
// );

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
