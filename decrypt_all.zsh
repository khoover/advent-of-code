#!/bin/env zsh

set -euo pipefail

cd "$(dirname $0)/inputs"

if [[ "$1" = "-i" ]]; then
    identity_path="$2"
    shift 2
else
    identity_path="${XDG_CONFIG_HOME}/age/advent_of_code"
fi

printf 'Using "%s" as the identity\n' "${identity_path}"

for year in *(F); do
    printf 'Year %s\n' "${year}"
    pushd "${year}"
    for encrypted_day in *.input.age(N); do
        decrypted_day="${encrypted_day%.age}"
        if [[ ! -e "${decrypted_day}" ]]; then
            age --decrypt -i "${identity_path}" -o "${decrypted_day}" "${encrypted_day}"
            printf '\tDecrypted %s into %s\n' "${encrypted_day}" "${decrypted_day}"
        else
            printf '\t%s is already decrypted\n' "${encrypted_day}"
        fi
    done
    printf '\n'
    popd
done
