# Silent Killer

*Experimental*

## What is it?

A small experiment with the [hound](https://github.com/ruuda/hound) crate audio API. It's very rough and not intended for anything resembling real use.

## What does it do?

It processes a `.wav` file, removing the silence based on certain thresholds (all the thresholds are hard-coded).

There are actually 2 implementations in the `main.rs`
 - one (`run`) that uses 2 file handles for reading, but (I think) runs in constant space avoids copying the whole file into memory.
 - the other (`run2`) uses 1 file handle for reading, but collects the samples into a `Vec` requiring allocation of the whole file.

## Usage

```shell
silent_killer <SOURCE>
```

Will generate a file "copy.wav" with the silence removed. In theory…
