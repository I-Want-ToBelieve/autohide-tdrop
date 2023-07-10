## autohide-tdrop

100% pure rust implementation that can automatically hide terminals or other applications managed by tdrop when they lose focus

## Installation

```zsh
cargo install autohide-tdrop
```

## Usage

~/.config/sxhkd/sxhkdrc
```zsh
ctrl + t
	tdrop -n tdrop_kitty --post-create-hook "autohide-tdrop &" -ma -h 60% -w 70% -x 15% -y 0 kitty --class=tdrop_kitty
```
