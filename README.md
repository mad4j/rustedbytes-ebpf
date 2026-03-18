# rustedbytes-ebpf

Template di modulo kernel eBPF implementato in Rust con Aya.

## Struttura del Progetto

```
rustedbytes-ebpf/           Progetto principale (loader userspace)
rustedbytes-ebpf-ebpf/      Programma eBPF (kernel space, XDP)
rustedbytes-ebpf-common/    Tipi condivisi tra kernel e userspace
xtask/                      Helper per build e run
test-env/                   Ambiente virtuale per i test (Vagrant)
```

## Prerequisiti

1. **Rust stable**: `rustup toolchain install stable`
2. **Rust nightly** (per la compilazione eBPF): `rustup toolchain install nightly --component rust-src`
3. **bpf-linker**: `cargo install bpf-linker`
4. **Vagrant** (per i test in VM): https://www.vagrantup.com/downloads
5. **VirtualBox** (provider VM): https://www.virtualbox.org/

## Build

```shell
# Compila sia il programma eBPF che il loader userspace
cargo build --package rustedbytes-ebpf

# Oppure con xtask
cargo xtask build

# Build di release
cargo xtask build --release
```

## Esecuzione

Richiede privilegi di root per caricare il programma eBPF:

```shell
# Esegui sulla interfaccia eth0 (default)
cargo xtask run

# Oppure specifica l'interfaccia
cargo xtask run --iface lo

# Direttamente con cargo (richiede sudo)
sudo -E cargo run --release -- --iface eth0
```

## Test in Ambiente Virtuale

Il progetto include un ambiente di test basato su Vagrant per eseguire il modulo
in una macchina virtuale isolata.

```shell
# Avvia la VM e lancia i test automaticamente
cargo xtask test-vm

# Oppure usa lo script direttamente
cd test-env
./run-tests.sh

# Accedi alla VM manualmente
cd test-env
vagrant up
vagrant ssh
```

## Cross-compilation (macOS)

```shell
CC=${ARCH}-linux-musl-gcc cargo build --package rustedbytes-ebpf --release \
  --target=${ARCH}-unknown-linux-musl \
  --config=target.${ARCH}-unknown-linux-musl.linker=\"${ARCH}-linux-musl-gcc\"
```

## Licenza

Ad eccezione del codice eBPF, rustedbytes-ebpf è distribuito sotto i termini
della [licenza MIT] o della [Apache License] (versione 2.0), a scelta.

Il codice eBPF è distribuito sotto i termini della
[GNU General Public License, Version 2] o della [MIT license], a scelta.

[Apache license]: LICENSE-APACHE
[MIT license]: LICENSE-MIT
[GNU General Public License, Version 2]: LICENSE-GPL2
