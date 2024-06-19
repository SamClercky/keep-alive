# Keep alive

Utility for keeping a crashing process alive.

To install:
```bash
cargo install --locked --git https://github.com/SamClercky/keep-alive.git
```

Example usage for `waybar`
```bash
keep-alive waybar
```

Options:
* `--failed`: Only restart when the application has failed

## Troubleshooting

When using this utility in a hyprland configuration file, make sure that the binary is in `/usr/bin`.
Otherwise, the binary will not be found.
