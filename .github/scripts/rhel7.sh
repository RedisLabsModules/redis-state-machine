#!/bin/sh

set -e

yum groupinstall -yqq 'Development Tools'
yum install -yqq curl centos-release-scl centos-release-scl-rh
yum install -yqq devtoolset-9 llvm-toolset-7
yum install -yqq rh-python38-python rh-python38-python-pip openssl-devel
