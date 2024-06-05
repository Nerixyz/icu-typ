async function runCommand(...args: string[]) {
  const cmd = new Deno.Command(args[0], {
    args: args.slice(1),
    stderr: "inherit",
    stdout: "inherit",
  });
  const { code } = await cmd.output();
  if (code !== 0) {
    console.error(`'${args.join(" ")}' exited with status ${code}`);
    Deno.exit(1);
  }
}

await runCommand("cargo", "b", "-r", "--target", "wasm32-unknown-unknown");

try {
  await Deno.remove("./build", { recursive: true });
} catch {
  // ignored
}
await Deno.mkdir("./build");
await Deno.mkdir("./build/res");

await runCommand(
  "wasm-opt",
  "-Oz",
  "./target/wasm32-unknown-unknown/release/icu_typ.wasm",
  "-o",
  "./build/icu-datetime.wasm",
);

const includedFiles = ["typst.toml", "README.md", "LICENSE"];
for await (const entry of Deno.readDir(".")) {
  if (
    entry.isFile &&
    (entry.name.endsWith(".typ") || includedFiles.includes(entry.name))
  ) {
    await Deno.copyFile(entry.name, `./build/${entry.name}`);
  }
}
await Deno.copyFile("./res/example.png", "./build/res/example.png");
