# Quickstart template for nRF52

A template for starting an RTFM-based project targeting the Nordic nRF52 series.

## Usage

1. Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate):

```bash
cargo install cargo-generate
```

2. Generate your project:

```bash
cargo generate --git https://github.com/chocol4te/nrf52-quickstart --name projectname
```

3. Configure GDB script

If you are using OpenOCD:

```bash
cd projectname
cp .gdbinit-openocd .gdbinit
```

Or if you are using a Black Magic Probe:

```bash
cd projectname
cp .gdbinit-bmp .gdbinit
```

And then modify the target port in `.gdbinit` to match your probe.

4. Run generated template project:

Compile, load and run your project:

```bash
cargo run
```

## License

GNU General Public License v3.0 ([LICENSE](https://www.gnu.org/licenses/gpl-3.0.txt)).
