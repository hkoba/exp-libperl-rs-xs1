#!/bin/zsh
set -e

cd $0:a:h

set -x

perl -Mblib -le 'use Mytest; Mytest::mySum(0)'
