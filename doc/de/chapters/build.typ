= Build

Für den Build von Splice sind nur wenige Schritte notwendig.

1. Download und Installation von Rust - dem Compiler in dem das Programm geschrieben ist.
2. Download der Sourcedateien
3. Build des Programms

== Linux
Linuxnutzer können den Compiler über ihre jeweilige Paketverwaltung erhalten:

Arch:
```bash
yay -S rustup
pacman -S rustup
```

Die Sourcedateien sind auf Github erhältlich

```bash
git clone git@github.com:Voidheart88/splice.git 
```

und lassen sich mit 

```bash
cd splice
cargo build --release
```

kompilieren.