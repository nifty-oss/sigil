#!/usr/bin/env zx
import "zx/globals";
import { workingDirectory } from "../utils.mjs";

// Run the tests.
cd(path.join(workingDirectory, "clients", "rust"));
const hasSolfmt = await which("solfmt", { nothrow: true });

const options = require("minimist")(process.argv.slice(2), {
  "--": true,
});

const args = [];

if (options.test) {
  args.push("--test");
  args.push(options.test);
}

argv._.filter((a) => a !== path.basename(__filename)).forEach((a) =>
  args.push(a)
);

if (hasSolfmt) {
  await $`cargo test-sbf ${args} 2>&1 | solfmt`;
} else {
  await $`cargo test-sbf ${args}`;
}
