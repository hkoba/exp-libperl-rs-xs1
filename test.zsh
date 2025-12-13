#!/bin/zsh
set -e

cd $0:a:h

set -x

perl -Mblib -le 'use Mytest; print $_, " ", Mytest::is_even($_) for 0, 1, 2'
