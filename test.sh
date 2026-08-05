#! /bin/bash

set -e

rustc $1
./$2
