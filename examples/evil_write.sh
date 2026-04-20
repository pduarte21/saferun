#!/bin/sh
echo "trying to read sensitive file..."

cat ~/.ssh/id_rsa 2>/dev/null || echo "could not read ssh key"