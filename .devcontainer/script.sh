## update and install some things we should probably have
apt-get update
apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  zsh \
  vim \
  build-essential \
  openssl

## update and install 2nd level of packages
apt-get install -y pkg-config

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y

export PATH="/root/.cargo/bin/":$PATH

rustup toolchain install nightly
# rustup component add rustfmt
# rustup component add rustfmt --toolchain nightly
# rustup component add clippy 
# rustup component add clippy --toolchain nightly

# Download cargo-binstall to ~/.cargo/bin directory
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

cargo binstall cargo-expand cargo-edit cargo-watch -y

## setup and install oh-my-zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
cp -R /root/.oh-my-zsh /home/$USERNAME
cp /root/.zshrc /home/$USERNAME
sed -i -e "s/\/root\/.oh-my-zsh/\/home\/$USERNAME\/.oh-my-zsh/g" /home/$USERNAME/.zshrc
chown -R $USER_UID:$USER_GID /home/$USERNAME/.oh-my-zsh /home/$USERNAME/.zshrc