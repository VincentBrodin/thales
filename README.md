# thales

`thales` is a tiny tool that reacts to monitor updates in **Hyprland**.
Instead of cloning displays or manually toggling configs, you can define per monitor rules in a simple TOML file.

When a monitor connects or disconnects, `thales` runs the monitor config you’ve set for example, disabling your laptop screen when an external monitor is plugged in.

## Install

**From Source**

```bash
git clone https://github.com/VincentBrodin/thales.git
cd thales
cargo build --release
```

Then add this to your Hyprland config:

```
exec-once = PATH/TO/thales/target/release/thales
```

## Usage

Thales uses a single TOML config found at `~/.config/thales/config.json`.
Here’s an example (my personal setup):

```toml
[[monitors]]
name = "HDMI-A-1"
on_added = ["eDP-1,disabled"]
on_removed = ["ePD-1,1920x1080@60,0x0,1"]
```

### What it does

- When `HDMI-A-1` gets plugged in, it runs the `on_added` command -> disables `eDP-1`.
- On startup or config reload, `thales` checks all monitors listed in your config.
  - If a monitor is connected, it runs `on_added`.
  - If not, it runs `on_removed`.

## Syntax

The commands use Hyprland’s built in monitor syntax. 
-> [Read more here](https://wiki.hypr.land/Configuring/Monitors/).
