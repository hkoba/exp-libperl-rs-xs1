# How to write XS in Rust using libperl-sys

## BUILD and RUN

```sh
cargo build &&
make &&
perl -Mblib -le 'use Mytest; print $_, " ", Mytest::is_even($_) for 0, 1, 2'
```

