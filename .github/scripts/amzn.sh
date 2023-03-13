#!/bin/sh

set -e

amazon-linux-extras install -y epel
yum groupinstall -yqq 'Development Tools'
yum install -yqq wget openssl-devel yum-utils
yum -y install http://mirror.centos.org/centos/7/extras/x86_64/Packages/centos-release-scl-rh-2-3.el7.centos.noarch.rpm
yum -y install http://mirror.centos.org/centos/7/extras/x86_64/Packages/centos-release-scl-2-3.el7.centos.noarch.rpm
yum -y install http://mirror.centos.org/centos/7/sclo/x86_64/rh/Packages/l/llvm-toolset-7-clang-4.0.1-1.el7.x86_64.rpm
yum install -yqq rh-python38-python rh-python38-python-pip