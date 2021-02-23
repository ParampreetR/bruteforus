PROJECT_NAME=bruteforus

sudo cp ./target/release/bruteforus /usr/bin/

sudo install -Dm644 target/release/build/$PROJECT_NAME-*/out/$PROJECT_NAME.bash "$tempdir/usr/share/bash-completion/completions/${PROJECT_NAME}"
sudo install -Dm644 target/release/build/$PROJECT_NAME-*/out/$PROJECT_NAME.fish "$tempdir/usr/share/fish/completions/$PROJECT_NAME.fish"
sudo install -Dm644 target/release/build/$PROJECT_NAME-*/out/_$PROJECT_NAME "$tempdir/usr/share/zsh/vendor-completions/_$PROJECT_NAME"