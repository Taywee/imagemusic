import init, { song_from_toml, song_free, song_samples } from './pkg/imagemusic.js';

async function run() {
    await init();

    /*const audioCtx = new window.AudioContext();

    const song = song_from_toml(bloody_tears);

    let max_amplitude: number = 0.0;
    const samples = song_samples(song, audioCtx.sampleRate);
    console.log('Analyzing samples');
    samples.forEach(sample => {
        let amplitude = Math.abs(sample);
        if (amplitude > max_amplitude) {
            max_amplitude = amplitude;
        }
    });
    console.log('Done analyzing samples');

    const myArrayBuffer = audioCtx.createBuffer(1, samples.length, audioCtx.sampleRate);
    const channel = myArrayBuffer.getChannelData(0);

    // Find the reciprocal so that we can get the samples through multiplication
    // instead of division
    const gain = 1.0 / max_amplitude;
    let i = 0;
    console.log('Filling buffer');
    samples.forEach(sample => {
        channel[i] = sample * gain;
        i += 1;
    });
    console.log('Filled buffer');

    song_free(song);

    const source = audioCtx.createBufferSource();
    source.buffer = myArrayBuffer;
    source.connect(audioCtx.destination);
    source.start();*/
}

run();
