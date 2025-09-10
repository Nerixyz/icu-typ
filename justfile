set windows-shell := ["pwsh.exe", "-NoLogo", "-NoProfile", "-Command"]

version := if os_family() == "unix" { 
    `sed -nr 's/^version += +"([^"]+)"$/\1/p' typst/typst.toml`
} else {
    `cat typst/typst.toml | Select-String 'version += +"([^"]+)"' | % { $_.Matches.Groups[1].Value }`
}

local-dir := if os_family() == "unix" {
    data_directory() / "typst" / "packages" / "local" / "icu-datetime" / version
} else {
    `$Env:APPDATA` / "typst" / "packages" / "local" / "icu-datetime" / version
}

[unix]
symlink dst target:
    ln -fs {{target}} {{dst}}

[windows]
symlink dst target:
    New-item -Force -ItemType SymbolicLink {{dst}} -Target {{target}}

build:
    cargo b -r --target wasm32-unknown-unknown

[unix]
clean-dir dir:
    rm -rf {{dir}}
    mkdir -p {{dir}}/res

[windows]
clean-dir dir:
    mkdir -Force {{dir}}
    rm -Recurse {{dir}}
    mkdir -Force {{dir}}/res

bundle: build (clean-dir "build")
    cp typst/*.typ build/.
    cp typst/typst.toml build/.
    cp README.md build/.
    cp LICENSE build/.
    cp res/example.png build/res/.
    # wasm-opt -Oz ./target/wasm32-unknown-unknown/release/icu_typ.wasm -o ./build/icu-datetime.wasm
    cp ./target/wasm32-unknown-unknown/release/icu_typ.wasm ./build/icu-datetime.wasm

[unix]
deploy: bundle
    mkdir -p {{local-dir}}
    cp -r build/* {{local-dir}}/.

[windows]
deploy: bundle
    mkdir -Force {{local-dir}}
    cp -Force -Recurse build/* {{local-dir}}/.

local-wasm: (symlink "typst/icu-datetime.wasm" "../target/wasm32-unknown-unknown/release/icu_typ.wasm") build

example: local-wasm
    typst c res/example.typ res/example.png --root .
    oxipng -Z -o max res/example.png

test: local-wasm
    typst query --root . tests/main.typ --one "<ok>"
