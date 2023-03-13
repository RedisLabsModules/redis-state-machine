#!/bin/sh

set -e

apt-get update -qq
apt-get install -yqq curl python3 python3-pip clang make git libssl-dev