## autohide-tdrop

100% pure rust implementation that can automatically hide terminals or other applications managed by tdrop when they lose focus

## Installation

```zsh
cargo install autohide-tdrop
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
	WAYLAND_DISPLAY=no KITTY_DISABLE_WAYLAND=1 tdrop -n tdrop_terminl --post-create-hook "autohide-tdrop &" -mta -h 60% -w 70% -x 15% -y 0 kitty --class tdrop_terminl
```
