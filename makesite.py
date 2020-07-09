#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# Copyright © 2020 Taylor C. Richberger
# This code is released under the license described in the LICENSE file

from pathlib import Path
import subprocess
from hashlib import sha1
import shutil
from markdown import markdown

subprocess.run(['wasm-pack', 'build', '--target', 'web', '--release'], check=True)
subprocess.run(['tsc'], check=True)

shutil.rmtree('build')

Path('build/pkg').mkdir(parents=True, exist_ok=True)

with open('main.css', 'rb') as file:
    main_css_sum = sha1(file.read()).hexdigest()

with open('pkg/imagemusic_bg.wasm', 'rb') as file:
    wasm_sum = sha1(file.read()).hexdigest()

with open('main.js') as file:
    main_js = file.read().replace('imagemusic.js', f'imagemusic_{wasm_sum}.js')

main_js_sum = sha1(main_js.encode('utf-8')).hexdigest()

shutil.copy('robots.txt', 'build/robots.txt')
shutil.copy('favicon.ico', 'build/favicon.ico')
shutil.copy('main.css', f'build/main_{main_css_sum}.css')
shutil.copy('pkg/imagemusic_bg.wasm', f'build/pkg/imagemusic_{wasm_sum}_bg.wasm')
shutil.copy('pkg/imagemusic.js', f'build/pkg/imagemusic_{wasm_sum}.js')

with open(f'build/main_{main_js_sum}.js', 'w') as file:
    file.write(main_js)

for image in Path('.').glob('*.png'):
    shutil.copy(image, 'build')

shutil.copytree('art', 'build/art')
shutil.copytree('songs', 'build/songs')

with open('HowToUse.md') as file:
    how_to_use = markdown(file.read())

with open('HowDoesItWork.md') as file:
    how_does_it_work = markdown(file.read())

with open('Copyright.md') as file:
    copyright = markdown(file.read())

with open('template.html') as file:
    template = file.read()

with open('index.html') as file:
    index = file.read()

template = template.replace('${MAIN_CSS}', f'main_{main_css_sum}.css')
index = index.replace('${MAIN_JS}', f'main_{main_js_sum}.js')

with open('build/index.html', 'w') as file:
    file.write(template.replace('${BODY}', index).replace('${TITLE}', 'imagemusic'))

with open('build/howtouse.html', 'w') as file:
    file.write(template.replace('${BODY}', how_to_use).replace('${TITLE}', 'imagemusic - How To Use'))

with open('build/howdoesitwork.html', 'w') as file:
    file.write(template.replace('${BODY}', how_does_it_work).replace('${TITLE}', 'imagemusic - How Does It Work?'))

with open('build/copyright.html', 'w') as file:
    file.write(template.replace('${BODY}', copyright).replace('${TITLE}', 'imagemusic - Copyright'))
