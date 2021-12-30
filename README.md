![logo](doc/logo.png)

# soundboi

cross platform minimalistic soundboard that lives in your terminal

## features

Play sound samples from your CLI using one keystroke per sample.

* cross platform (TODO: have someone test with macOS)
* minimal (<100 lines of Rust)
* zero config
* supports mp3, wav and ogg samples

## build instructions

Right now, you need to build `soundboi` yourself.
You need Rust [installed](https://www.rust-lang.org/learn/get-started)
on your machine if you don't have already.

```
git clone https://github.com/wullewutz/soundboi
cd soundboi
cargo build --release
```

## usage

Place all your sound samples in a folder called `soundboi` inside your XDG
config directory. Depending on your OS and settings usually:

* Linux: `/home/alice/.config/soundboi`
* Windows: `C:\Users\Alice\AppData\Roaming\soundboi`
* Mac: `/Users/Alice/Library/Application Support/soundboi`

First character of each sound file will be used as the trigger for this sample.
Therefore, first characters have to be unique for all files.
To achive this you can (and should) prefix each file with the desired trigger 
key followed by an underscore: 

```
~/.config/soundboi> ls                                                      
a_applause.wav	d_laser.wav  f_laughter.mp3  s_gunshot.wav
```
Now run soundboi

```
cargo run --release
a: applause
d: laser
f: laughter
s: gunshot
```

You will get a map of the keys and samples and are ready to have fun with your 
coworkers.

Simply press Ctrl-C to exit `soundboi`.
