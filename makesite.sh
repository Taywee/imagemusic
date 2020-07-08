#!/bin/sh
# Copyright © 2020 Taylor C. Richberger
# This code is released under the license described in the LICENSE file

set -ve

getsum() {
  sha1sum ${1:-getsum needs input} | awk '{print $1}'
}

wasm-pack build --target web --release

wasm_sum=$(getsum pkg/imagemusic_bg.wasm)
main_css_sum=$(getsum main.css)

if [ -e build ]; then
  rm -r build
fi

mkdir -p build/pkg
cp main.css "build/main_${main_css_sum}.css"
cp pkg/imagemusic_bg.wasm build/pkg/imagemusic_${wasm_sum}_bg.wasm
cp pkg/imagemusic.js build/pkg/imagemusic_${wasm_sum}.js
cp *.html *.png build

tsc
sed "s/imagemusic\.js/imagemusic_${wasm_sum}.js/g" main.js > build/main.js
main_js_sum=$(getsum build/main.js)
mv build/main.js "build/main_${main_js_sum}.js"
cp -r art songs build

sed -e "s/main\.js/main_${main_js_sum}.js/g" -e "s/main\.css/main_${main_css_sum}.css/g" index.html > build/index.html
