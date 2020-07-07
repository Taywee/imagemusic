import init, { song_from_toml, song_free, song_samples, song_bake_image } from './pkg/imagemusic.js';

function image_to_data(image: HTMLImageElement): Uint8ClampedArray {
    const canvas = <HTMLCanvasElement>document.createElement("canvas");
    canvas.width = image.width;
    canvas.height = image.height;
    const ctx = <CanvasRenderingContext2D>canvas.getContext('2d');
    ctx.drawImage(image, 0, 0);
    return ctx.getImageData(0, 0, image.width, image.height).data;
}

function data_to_url(width: number, height: number, data: Uint8Array): string {
    const canvas = <HTMLCanvasElement>document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;
    const ctx = <CanvasRenderingContext2D>canvas.getContext('2d');

    for (let y = 0; y < height; ++y) {
        for (let x = 0; x < width; ++x) {
            let pixel = y * width + x;
            let offset = pixel * 4;
            let r = data[offset];
            let g = data[offset + 1];
            let b = data[offset + 2];
            let a = data[offset + 3];

            ctx.fillStyle = `rgba(${r}, ${g}, ${b}, ${a})`;
            ctx.fillRect(x, y, 1, 1);
        }
    }
    return canvas.toDataURL('image/png');
}

async function run() {
    await init();

    let input_image = <HTMLImageElement>document.getElementById("input-image")!;
    let sample_image = document.getElementById("sample-image")!;
    sample_image.addEventListener("input", event => {
        let target = <HTMLInputElement>event.target;
        input_image.src = target.value;
    });

    let input_song = <HTMLTextAreaElement>document.getElementById("input-song")!;
    let sample_song = document.getElementById("sample-song")!;
    sample_song.addEventListener("input", event => {
        let target = <HTMLInputElement>event.target;
        fetch(target.value)
        .then(response => response.text().then(text => input_song.value = text));
    });

    let bake = <HTMLButtonElement>document.getElementById("bake")!;

    let output_image = <HTMLImageElement>document.getElementById("output-image")!;
    bake.addEventListener("click", event => {
        const song = song_from_toml(input_song.value);
        const input_image_data = image_to_data(input_image);
        const output_image_data = song_bake_image(song, input_image.width, input_image.height, input_image_data);
        const data_url = data_to_url(input_image.width, input_image.height, output_image_data);
        output_image.src = data_url;
    });

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
