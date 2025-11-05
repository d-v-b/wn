# wn

A command-line tool for generating audible noise.

## About

Listening to white noise can help me sleep, especially when there's a more annoying type of noise 
that needs to be drowned out. There are 10+ hour-long YouTube videos for this, but 
it's a bit overkill to use a video streaming site for something as simple as white noise.

`wn` is a command-line tool that lets you generate your own white, (or brown, or pink, or ...) noise.

## Examples

### 5 seconds of blue noise

```zsh
➜ wn --noise blue --amplitude 0.5 --duration 5
```

### getting help

```zsh
➜ wn --help 
Generate noise

Usage: wn --noise <NOISE> --amplitude <AMPLITUDE> --duration <DURATION>

Options:
  -n, --noise <NOISE>          [possible values: white, gaussian, white-triangular, pink, blue, violet, brownian, velvet]
  -a, --amplitude <AMPLITUDE>  
  -d, --duration <DURATION>    
  -h, --help                   Print help
  -V, --version                Print version
```