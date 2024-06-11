set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

version := if os_family() == "unix" { 
    `sed -nr 's/^version += +"([^"]+)"$/\1/p' typst.toml`
} else {
    `cat typst.toml | Select-String 'version += +"([^"]+)"' | % { $_.Matches.Groups[1].Value }`
}

local-dir := if os_family() == "unix" {
    data_directory() / "typst" / "packages" / "local" / "icu-datetime" / version
} else {
    `$Env:APPDATA` / "typst" / "packages" / "local" / "icu-datetime" / version
}

build:
    cargo b -r --target wasm32-unknown-unknown

[unix]
clean-dir dir:
    rm -rf {{dir}}
    mkdir -p {{dir}}

[windows]
clean-dir dir:
    rm -Recurse {{dir}}
    mkdir -Force {{dir}}

bundle: build (clean-dir "build")
    cp *.typ build/.
    cp typst.toml build/.
    cp README.md build/.
    cp LICENSE build/.
    wasm-opt -Oz ./target/wasm32-unknown-unknown/release/icu_typ.wasm -o ./build/icu-datetime.wasm

[unix]
deploy: bundle
    mkdir -p {{local-dir}}
    cp build/* {{local-dir}}/.

[windows]
deploy: bundle
    mkdir -Force {{local-dir}}
    cp build/* {{local-dir}}/.

