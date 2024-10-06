#!/bin/bash

if [ -n "$BASH_VERSION" ]; then
    BASH_SOURCE="${BASH_SOURCE[0]}"
elif [ -n "$ZSH_VERSION" ]; then
    BASH_SOURCE="${(%):-%x}"
else
    echo "Unsupported shell. Please use Bash or Zsh."
    exit 1
fi

SWD="$(dirname "$(realpath "$BASH_SOURCE")")"

export LIBTORCH_USE_PYTORCH=1

# export LIBTORCH="${LIBTORCH:-"$SWD/shorcuts/libtorch"}"
# export LIBTORCH_LIB="${LIBTORCH_LIB:-"$LIBTORCH"}"
# export LIBTORCH_INCLUDE="${LIBTORCH_INCLUDE:-"$LIBTORCH"}"
# export DYLD_LIBRARY_PATH="$LIBTORCH/lib:$DYLD_LIBRARY_PATH"
