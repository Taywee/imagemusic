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

    divisions = int(root.find('./part/measure/attributes/divisions').text)
    per_minute = int(root.find('./part/measure/direction/direction-type/metronome/per-minute').text)
    ticks_per_second = divisions / per_minute * 60
    print(f'ticks_per_second = {ticks_per_second}')

    beats = int(root.find('./part/measure/attributes/time/beats').text)
    beat_type = int(root.find('./part/measure/attributes/time/beat-type').text)

    # Needed to fill in voices that are missing for a measure
    divisions_per_measure = divisions * beats // beat_type * 4


    for part in root.findall('part'):
        # Set of voices in the part
        part_voices = frozenset(int(note.find('voice').text) for measure in part.findall('measure') for note in measure.findall('note'))
                
        # notes and ties are a list of voices, each voice is a list of chords, which is each a list of notes of the same length.  tieduration is just a dictionary of chords for each voice
        notes = {voice: [] for voice in part_voices}
        tieduration = {voice: {} for voice in part_voices}
        for measure in part.findall('measure'):
            measure_voices = set()
            for note in measure.findall('note'):
                voice = int(note.find('voice').text)
                measure_voices.add(voice)

                duration = int(note.find('duration').text)
                rest = note.find('rest') is not None
                chord = note.find('chord') is not None
                if not chord:
                    notes[voice].append([])

                chord_note = len(notes[voice][-1])
                    
                tie = set(tie.attrib['type'] for tie in note.findall('tie'))
                if 'start' in tie and 'stop' in tie:
                    # continuation
                    tieduration[voice][chord_note] += duration
                    continue
                elif 'stop' in tie:
                    duration += tieduration[voice][chord_note]
                    tieduriation = 0
                elif 'start' in tie:
                    tieduration[voice][chord_note] = duration
                    continue

                tie.discard('start')
                tie.discard('stop')
                if tie:
                    raise RuntimeError(f'tie had unknown attribute: {tie!r}')

                if rest:
                    notes[voice][-1].append(f'{duration}r')
                else:
                    pitch = note.find('pitch')
                    step = pitch.find('step').text.lower()
                    octave = int(pitch.find('octave').text)
                    alter = pitch.find('alter')
                    if alter is None:
                        notes[voice][-1].append(f'{duration}{step}{octave}')
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

                            notes[voice][-1].append(f'{duration}{step}{suffix}{octave}')

            for missing_voice in part_voices - measure_voices:
                notes[missing_voice].append(f'{divisions_per_measure}r')

        sorted_notes = (notes[key] for key in sorted(notes))
        for voice_notes in sorted_notes:
            print('[[voice]]')
            print("notes = '''")
            print(' '.join(voice_notes))
            print("'''")
            print()

if __name__ == '__main__':
    main()

