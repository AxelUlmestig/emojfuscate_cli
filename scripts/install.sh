#!/bin/sh

build_output_dir=target/release/build/emojfuscate_cli-*
rm -r $build_output_dir

cargo build --release
sudo cp target/release/emojfuscate /usr/bin/

bash_completion_dir=/etc/bash_completion.d
if [ -d "$bash_completion_dir" ]; then
  sudo cp $build_output_dir/out/emojfuscate.bash "$bash_completion_dir/emojfuscate"
fi

zsh_completion_dir=/usr/local/share/zsh/site-functions
if [ -d "$zsh_completion_dir" ]; then
  sudo cp $build_output_dir/out/_emojfuscate "$zsh_completion_dir/_emojfuscate"
fi
