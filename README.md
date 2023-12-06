## autohide-tdrop

100% pure rust implementation that can automatically hide terminals or other applications managed by tdrop when they lose focus

## Installation
Cargo
```zsh
cargo install autohide-tdrop
```
Or ArchLinux

```zsh
paru -S autohide-tdrop-git
```

Or Nix

flake.nix
```flake.nix
{
  inputs = {
    autohide-tdrop = {
      url = "github:I-Want-ToBelieve/autohide-tdrop";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
}
```

home.nix
```nix
{
  inputs,
  pkgs,
  ...
}: {
  home.packages = with inputs;
    [
      autohide-tdrop.packages.${pkgs.system}.default
    ];
}
```

## Usage

### X11
~/.config/sxhkd/sxhkdrc
```zsh
ctrl + t
	tdrop -n tdrop_kitty --post-create-hook "autohide-tdrop &" -ma -h 60% -w 70% -x 15% -y 0 kitty --class=tdrop_kitty
```

### wayland
~/.config/swhkd/swhkdrc
```zsh
ctrl + t
	WAYLAND_DISPLAY_OLD=$WAYLAND_DISPLAY WAYLAND_DISPLAY=no KITTY_DISABLE_WAYLAND=1 tdrop -n tdrop_terminal --post-create-hook "autohide-tdrop &" -mta -h 60% -w 70% -x 15% -y 0 kitty --class tdrop_terminal
```
You may need to restore the WAYLAND_DISPLAY environment variable in your shell-initialized hook.
using fish shell as an example:
```fish
# Hello, this is Bing. The following is the command in the fish shell to determine the value of the environment variable CCC to the environment variable BBB when the environment variable AAA is 1 and the environment variable BBB exists and is not empty:
# if test "$AAA" = "1" -a -n "$BBB"
#     set -x CCC $BBB
# end
# Among them, the test command is used to test whether the condition is true, -a means logical and, -n means non-null. If the condition is true, use the set command to set the value of the environment variable CCC to the value of the environment variable BBB.
# Hope this helps!

if test "$KITTY_DISABLE_WAYLAND" = "1" -a -n "$WAYLAND_DISPLAY_OLD"
  set -x WAYLAND_DISPLAY $WAYLAND_DISPLAY_OLD
end
```

