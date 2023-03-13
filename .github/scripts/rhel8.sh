#!/bin/sh

set -e
yum groupinstall -yqq 'Development Tools'
yum install -yqq python38-pip python38-devel python38 wget clang clang-devel openssl-devel compat-openssl10 git
