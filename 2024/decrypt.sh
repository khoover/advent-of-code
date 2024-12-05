#!/bin/sh

cd ./input/2024/

# Decrypt the file
# --batch to prevent interactive command
# --yes to assume "yes" for questions
for file in *.gpg; do
    [ -f "$file" ] || break
    gpg --quiet --batch --yes --decrypt --passphrase="$INPUT_PASSPHRASE" \
        --output "./${file%.*}"
done