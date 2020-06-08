#!/bin/bash

function get_this_dir() {
  # Taken from https://stackoverflow.com/questions/59895#246128
  local SOURCE="${BASH_SOURCE[0]}"
  while [ -h "$SOURCE" ]; do # resolve $SOURCE until the file is no longer a symlink
    DIR="$(cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd)"
    SOURCE="$(readlink "$SOURCE")"
    [[ $SOURCE != /* ]] && SOURCE="$DIR/$SOURCE" # if $SOURCE was a relative symlink, we need to resolve it relative to the path where the symlink file was located
  done
  (cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd)
}

readonly this_dir="$(get_this_dir)"

docker run -it --rm -v"${this_dir}":'/app' -w '/app' -p4567:4567 'ghgen' bash

