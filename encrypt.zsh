#!/bin/env zsh

set -euo pipefail

cd "$(dirname $0)/inputs"

if [[ $# -gt 0 && "$1" = "-R" ]]; then
    recipient_path="$2"
    shift 2
else
    recipient_path="${XDG_CONFIG_HOME}/age/advent_of_code.pub"
fi

printf 'Using %s as the recipient\n' "${recipient_path}"

function encrypt_year() {
    pushd "$1"
    printf 'Encrypting all days in %s\n' "$1"

    local unencrypted_day
    for unencrypted_day in *.input(N); do
        local encrypted_day="${unencrypted_day}.age"
        if [[ ! -e "${encrypted_day}" ]]; then
            age -R "${recipient_path}" -o "${encrypted_day}" "${unencrypted_day}"
            printf '\tEncrypted %s to %s\n' "${unencrypted_day}" "${encrypted_day}"
        else
            printf '\t%s is already encrypted\n' "${unencrypted_day}"
        fi
    done

    popd
}

if [[ $# == 2 ]]; then
    unencrypted_path="$1/${(l:2::0:)2}.input"
    encrypted_path="${unencrypted_path}.age"
    if [[ -e "${encrypted_path}" ]]; then
        printf 'Year %s day %s is already encrypted\n' "$1" "$(($2))"
    else
        age -R "${recipient_path}" -o "${encrypted_path}" "${unencrypted_path}"
        printf 'Encrypted year %s day %s into %s\n' "$1" "$(($2))" "${encrypted_path}"
    fi
elif [[ $# == 1 ]]; then
    encrypt_year $1
else
    for year in *(F); do
        encrypt_year "${year}"
    done
fi
