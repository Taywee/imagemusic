# asciimusic

Experiments in encoding music in ascii.  Might have some sort of leading data to
indicate tempo, note fade out, simultaneous voices (and maybe their
corresponding volumes), number of characters per note, bottom pitch frequency,
and division of the note (ie. a packed representation might have 5 bits for
pitch and 3 for length).  Maybe even some bits for instrument selection.

Maybe integrate with SDL or something so that it can play music on its own.
Maybe use a proper gui toolkit so that musical phrases can directly be loaded in
and other people can use it.  (perhaps making it a web-related project could be
doable.  This would probably have to be webassembly, because I'd prefer to do
the bulk in Rust).

Keep in mind for the following values that these bits will not be 8 bits per
input bytes.  If the input bytes are base-64, each character only gives 6 bits.
With base32, input characters only give 5 bits.

Base32 is clearer and easier to work with, but base64 will give 5.25 octaves for
a single character instead of just 2.58.  2.5 octaves is barely the cello's own
pitch range without thumb position.  5.25 is enough to go from the bottom of the
cello's pitch to way above the Violin's natural pitch.

## Note Range

Bits per note (0 will be a rest, so the total is always reduced by 1):

* 1: 1 semitones; 1/12 octave
* 2: 3 semitones; 1/4 octave
* 3: 7 semitones; 7/12 octave
* 4: 15 semitones; 1.25 octave (1 octave, 3 semitones)
* 5: 31 semitones; 2.58 octave (2 octave, 7 semitones)
* 6: 63 semitones; 5.25 octave (5 octaves, 3 semitones)
* 7: 127 semitones; 10.58 octave (10 octaves, 7 semitones)
* 8: 255 semitones; 21.25 octaves (21 octaves, 3 semitones)

## Tone length

Assuming a base of 16th notes. Finest subdivision is 16th notes here.  Triplets
of the higher note is possible by taking it down to sixths with the tempo.
Three sixths make a half, two make a third.

bits per length: longest encodable note.  (0 will not be possible)

* 1: 8th note
* 2: quarter note
* 3: half note
* 4: whole note
* 5: whole note (smallest is 32nd note)
* 6: whole note (smallest is 64th note)

## Other potential things to determine with bits or bytes in-line.

* fade-in
* fade-out
* damping
* distortion

# Formats to consider for polyphony

A reasonable setup is base64 input.  Initial blocks set up tempo and other such
things, including how many voices, and what instrument each voice gets.  Then
the beats come in.  Each instrument that needs a beat (its previous note or rest
has finished) will, in order, expect to pull a beat.  Voices are not specified
inline, they just pull the next applicable note at the right time.  At the
beginning, all the beginning notes will be each for one of the voices.  If one
voice had a longer note than the others, it will take longer for it to need its
own note.

Another reasonable setup is base32 input, with each voice needing its own base
tempo, allowing each voice to have an individually reasonable 2.5 octave range.
Somebody could have two voices with the same instruments to fake having a higher
range, but that would involve long-resting the instrument at all other times.
