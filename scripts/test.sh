#!/bin/bash
set -eux -o pipefail

main() {
  find . | grep -v /target | grep -v "/\." | entr -s \
    'date && cargo test'
}

main "$@"
