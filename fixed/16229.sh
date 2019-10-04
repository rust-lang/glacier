#!/bin/bash

if [ "$(uname)" != Linux ]; then
  echo "Only works on Linux"
  exit 1
fi

if ! which debootstrap; then
  echo "Requires debootstrap"
  exit 1
fi

jail="/tmp/16229"

mkdir -p $jail
sudo debootstrap wheezy $jail

echo 'fn main() {}' > /tmp/16229.rs
sudo mv /tmp/16229.rs $jail/16229.rs

rustc_location=$(which rustc)

for dependency in $(ldd $rustc_location | grep '/' | cut -d'>' -f2 | cut -d'(' -f1); do
  sudo mkdir -p $jail$(dirname $dependency)
  sudo cp $dependency $jail$dependency
done


sudo cp -r $(dirname $(dirname $rustc_location))/lib $jail

sudo mkdir -p $jail/bin
sudo cp $rustc_location $jail/bin/rustc

sudo chroot $jail /bin/rustc /16229.rs
