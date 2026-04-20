#!/bin/sh
echo "trying network access..."

curl https://example.com 2>/dev/null || echo "network blocked or tool unavailable"