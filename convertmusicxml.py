#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# Copyright © 2020 Taylor C. Richberger
# This code is released under the license described in the LICENSE file

import locale
import argparse
import xml.etree.ElementTree as ET

def main():
    locale.setlocale(locale.LC_ALL, '')
    parser = argparse.ArgumentParser(description='Do Something')
    parser.add_argument('-i', '--input', help='input musicxml file', required=True)
    args = parser.parse_args()

    doc = ET.parse(args.input)
    root = doc.getroot()

    for part in root.findall('part'):
        notes = []
        for measure in part.findall('measure'):
            for note in measure.findall('note'):
                duration = int(note.find('duration').text)
                rest = note.find('rest') is not None
                if rest:
                    notes.append(f'{duration}r')
                else:
                    pitch = note.find('pitch')
                    step = pitch.find('step').text.lower()
                    octave = int(pitch.find('octave').text)
                    alter = pitch.find('alter')
                    if alter is None:
                        notes.append(f'{duration}{step}{octave}')
                    else:
                        alter = int(alter.text)
                        if alter != 0:
                            if alter < 0:
                                if step in {'a', 'e'}:
                                    suffix = 's'
                                else:
                                    suffix = 'es'

                                for i in range(1, -alter):
                                    suffix += 'es'
                            else:
                                suffix = ''.join('is' for i in range(alter))

                            notes.append(f'{duration}{step}{suffix}{octave}')
        print('[[voice]]')
        print("notes = '''")
        print(' '.join(notes))
        print("'''")
        print()

if __name__ == '__main__':
    main()

