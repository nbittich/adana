rm -rf pkg
npm v @nbittich/adana-script-wasm dist.tarball | xargs curl | tar -xz
mv package pkg
