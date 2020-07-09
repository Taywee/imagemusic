# How do I use this?

Note that this does not work in all browsers.  I couldn't get it working on
Chrome on an iPhone or on Safari on a Mac, but that's no huge surprise.

## Quickstart

* Scroll to the very bottom of the main page
* choose a sample image (I recommend Beethoven's Moonlight Sonata (Movement III))
  * You should see the song field fill with some TOML
* Choose a sample image (I recommend Starry Night)
  * The image will fill the bottom image field
* Click "Bake Song Into Image" and wait for the image to appear at the top
  * The top image will have a target on the top left corner, indicating that it
    is a music image
* Click "Play Music Image" and wait a few seconds.  You should start hearing
  music in a bit.

## How do I play a music image I have found?

Sometimes you can drag and drop a music image directly into the top image field,
but this won't always work.  Imgur, for instance, allows this, but many sites do
not.  This is something I can't work around, as it's the source site that allows
or denies this. (The problem is
[CORS](https://en.wikipedia.org/wiki/Cross-origin_resource_sharing), if you're
curious)

If you can't drag and drop, you'll have to download the image, and then click
"Load Music Image..." to load the image from your device.  Then you can click
"Play Music Image", wait a few seconds, and start hearing the music.

## How do I make a music image?

You can make your own by getting your own image and loading it with "Load Input
Image..." or dragging and dropping it over the bottom image (the same caveats
apply here about cross-site drag and drop as above).  You can convert musicxml
music to the TOML format that imagemusic uses, but it only supports a limited
format, and specifically needs partwise musicxml, not timewise.  You can use
[musescore](https://musescore.org/) to generate musicxml that should work from
midis.  You can find midis by going to
[musescore](https://musescore.com/sheetmusic), or for video game music to
[VGMusic](https://vgmusic.com/). If you find a site that lets you either get
musicxml or midi, you should (counter-intuitively) probably go for the midi,
because it's a simpler format and musicxml is more likely to have constructs
that imagemusic won't convert well, like grace notes.  Also **make sure to
remove all percussion tracsk, as that isn't yet supported.  Keeping percussion
tracks in the mix will likely cause errors.**

The musicxml converter is also still pretty fresh and likely still quite buggy.
If you can find a small musicxml file that produces bad behavior with the
converter, you can submit it as a bug report on the issue tracker (see the
bottom of this page).

When your music is converted, you can review the TOML and maybe change some
instruments and stuff.  The conversion is very simple and will not try to take
anything like volume or instruments into account, so you may want to tweak it
and try it multiple times to get the best results.  If you have trouble with
getting good results because one voice is too complex (like volume changes),
your best bet is to use musescore or some other music editor to separate them
into separate voices before importing them again.

Once you have an image in the bottom image and valid music TOML in the text box,
you can "Bake Song Into Image" to get your baked image, and "Play Music Image"
to test it out.

Once you have your baked music image, you can share it wherever you like, and
people can use your image to hear whatever song you baked into it.

## The Music format

The music format is fairly self-descriptive.  A song is composed of
ticks\_per\_second and a list of voices.  Each voice has a [[voice]] section,
containing its volume, its instrument, its notes, and its envelope.

### Volume

Volume is a number from 0 to 255 giving the volume of the voice.

### Instrument

Instrument is either Sawtooth, Sine, Square, or Triangle.  More instruments and
percussion may come in the future.

### Notes

A string of notes.  Each note is composed of a length (in ticks), the note name,
and the octave.  So a middle-C quarter note might be 2c4, if each tick is an
eighth note.  Rests are just a length and the letter r.

### Envelope

The envelope is how an individual note's volume is modulated.  It is composed of
points.  Each point has a *stop* and an *amplitude*.  The stop is in hundredths
of seconds, and when positive, indicates a time length from the beginning of the
note, and when negative, indicates a time length from the end of the note.
Amplitude is a volume value from 0 to 255, similarly to the volume above.

When a note is being processed, the envelope is first put into terms of absolute
time, and the stops are taken from left to right.  Any stops before the
beginning of the note, after the end of the note, or out of order are removed.

You can use the envelope to get quite complex musical effects.

# I've found a bug!

Please report it!  This repository has its home on
[GitLab](https://gitlab.com/Taywee/imagemusic), which is the preferred place to
report issues, but it is cloned on
[GitHub](https://github.com/Taywee/imagemusic) and issues are being watched as
well.  If you can provide files to replicate it, that would greatly help.  The
smaller you can get these files, the better.
