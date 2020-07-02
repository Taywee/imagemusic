import init, { get_bloody_tears, song_free, song_samples, samples_free, samples_next } from './pkg/imagemusic.js';

async function run() {
    await init();

    const song = get_bloody_tears();
    const samples = song_samples(song);
    var audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    var myArrayBuffer = audioCtx.createBuffer(1, 44100 * 10, 44100);
    var channel = myArrayBuffer.getChannelData(0);

    let sample;
    let i = 0;
    while ((sample = samples_next(samples)) !== null) {
        channel[i] = sample;
        i += 1;
        if (i >= 441000) {
            break;
        }
    }

    samples_free(samples);
    song_free(song);

    var source = audioCtx.createBufferSource();
    source.buffer = myArrayBuffer;
    source.connect(audioCtx.destination);
    source.start();
}

run();
