import init, {
    song_bake_image,
    song_free,
    song_from_image,
    song_from_toml,
    song_samples,
    song_to_toml,
    song_from_musicxml,
} from './pkg/imagemusic.js';

async function sleep(ms: number): Promise<any> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function image_to_data(image: HTMLImageElement): Promise<Uint8ClampedArray> {
    let i = 0;
    while (!image.complete) {
        console.log("waiting");
        await sleep(10);
        if (++i > 100) {
            break;
        }
    }
    const canvas = <HTMLCanvasElement>document.createElement("canvas");
    canvas.width = image.naturalWidth;
    canvas.height = image.naturalHeight;
    const ctx = <CanvasRenderingContext2D>canvas.getContext('2d');
    ctx.drawImage(image, 0, 0);
    return ctx.getImageData(0, 0, image.naturalWidth, image.naturalHeight).data;
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

    const inputImage = <HTMLImageElement>document.getElementById("input-image")!;
    inputImage.crossOrigin = 'Anonymous';

    const sample_image = document.getElementById("sample-image")!;
    sample_image.addEventListener("input", event => {
        let target = <HTMLInputElement>event.target;
        inputImage.src = target.value;
    });

    const input_song = <HTMLTextAreaElement>document.getElementById("input-song")!;
    const sample_song = document.getElementById("sample-song")!;
    sample_song.addEventListener("input", event => {
        let target = <HTMLInputElement>event.target;
        fetch(target.value)
        .then(response => response.text().then(text => input_song.value = text).catch(alert))
            .catch(alert);
    });

    const bake = <HTMLButtonElement>document.getElementById("bake")!;
    const musicImage = <HTMLImageElement>document.getElementById("music-image")!;
    musicImage.crossOrigin = 'Anonymous';

    bake.addEventListener("click", async () => {
        try {
            bake.innerText = 'Baking Song Into Image...';
            const song = song_from_toml(input_song.value);
            try {
                const input_image_data = await image_to_data(inputImage);
                const music_image_data = song_bake_image(song, inputImage.naturalWidth, inputImage.naturalHeight, input_image_data);
                const data_url = data_to_url(inputImage.naturalWidth, inputImage.naturalHeight, music_image_data);
                musicImage.src = data_url;
            } finally {
                song_free(song);
            }
        }
        catch (e) {
            alert(e);
        }
        finally {
            bake.innerText = 'Bake Song Into Image';
        }
    });

    const audioCtx = new window.AudioContext();
    let source: AudioBufferSourceNode | null = null;

    const play = <HTMLButtonElement>document.getElementById("play")!;


    play.addEventListener("click", async () => {
        try {
            play.innerText = 'Processing Music Image...';

            // First disable onload so that we don't play this again the next
            // time an image is baked.  Baking shouldn't autoplay, but loading
            // and drag-n-drop should.
            musicImage.onload = null;
            const music_image_data = await image_to_data(musicImage);
            const song = song_from_image(musicImage.naturalWidth, musicImage.naturalHeight, music_image_data);
            try {
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
                console.log('amplitude: ' + max_amplitude);

                const myArrayBuffer = audioCtx.createBuffer(1, samples.length, audioCtx.sampleRate);
                const channel = myArrayBuffer.getChannelData(0);

                /*Find the reciprocal so that we can get the samples through multiplication
     instead of division*/
                const gain = 1.0 / max_amplitude;
                console.log('gain: ' + gain);
                let i = 0;
                console.log('Filling buffer');
                samples.forEach(sample => {
                    channel[i] = sample * gain;
                    i += 1;
                });
                console.log('Filled buffer');

                try {
                    source?.stop();
                }
                catch (_) {
                    // Do nothing, source might not be playing yet
                }

                source = audioCtx.createBufferSource();
                source.connect(audioCtx.destination);


                console.log('playing song')
                source.buffer = myArrayBuffer;
                console.log('starting song')
                source.start();
                console.log('started song')
            }
            finally {
                song_free(song);
            }
        }
        catch (e) {
            alert(`error: ${e}`);
        }
        finally {
            play.innerText = 'Play Music Image';
        }
    });

    const stop = <HTMLButtonElement>document.getElementById("stop")!;

    stop.addEventListener("click", _ => {
        try {
            source?.stop();
        }
        catch (_) {
            // Do nothing, source might not be playing yet
        }
        finally {
            source = null;
        }
    });

    const loadMusicImage = <HTMLButtonElement>document.getElementById("load-music-image")!;

    loadMusicImage.addEventListener("change", event => {
        try {
            let files = (<HTMLInputElement>event.target).files!;
            if (files.length > 0) {
                // Load all the files into data
                let url = URL.createObjectURL(files[0]);
                console.log('url ' + url);
                musicImage.src = url;
            }
        } catch (e) {
            alert(e);
        }
    });

    let dragHandler = (event: Event) => {
        event.stopPropagation();
        event.preventDefault();
    };

    musicImage.addEventListener("dragenter", dragHandler, false);
    musicImage.addEventListener("dragexit", dragHandler, false);
    musicImage.addEventListener("dragover", dragHandler, false);

    musicImage.addEventListener("drop", event => {
        event.stopPropagation();
        event.preventDefault();
        try {
            let files = event.dataTransfer?.files;
            console.log(event);
            if (files !== undefined && files!.length > 0) {
                const url = URL.createObjectURL(files[0]);
                console.log('url ' + url);
                musicImage.src = url;
            } else {
                const imageUrl = event.dataTransfer?.getData("text/uri-list");
                console.log('fetching ' + imageUrl);
                fetch(imageUrl!)
                .then(async response => {
                    try {
                        const blob = await response.blob();
                        console.log(blob);
                        const url = URL.createObjectURL(blob);
                        musicImage.src = url;
                    }
                    catch (e) {
                        alert(e);
                    }
                })
                .catch(e => {
                    alert(e + "\nTry downloading the image and uploading it manually.");
                });
            }
        }
        catch (e) {
            alert(e);
        }
    }, false);

    inputImage.addEventListener("dragenter", dragHandler, false);
    inputImage.addEventListener("dragexit", dragHandler, false);
    inputImage.addEventListener("dragover", dragHandler, false);

    inputImage.addEventListener("drop", event => {
        event.stopPropagation();
        event.preventDefault();
        try {
            let files = event.dataTransfer?.files;
            console.log(event);
            if (files !== undefined && files!.length > 0) {
                const url = URL.createObjectURL(files[0]);
                console.log('url ' + url);
                inputImage.src = url;
            } else {
                const imageUrl = event.dataTransfer?.getData("text/uri-list");
                console.log('fetching ' + imageUrl);
                fetch(imageUrl!)
                .then(async response => {
                    try {
                        const blob = await response.blob();
                        console.log(blob);
                        const url = URL.createObjectURL(blob);
                        inputImage.src = url;
                    }
                    catch (e) {
                        alert(e);
                    }
                })
                .catch(e => {
                    alert(e + "\nTry downloading the image and uploading it manually.");
                });
            }
        }
        catch (e) {
            alert(e);
        }
    }, false);

    const extract = <HTMLButtonElement>document.getElementById("extract")!;


    extract.addEventListener("click", async () => {
        try {
            const music_image_data = await image_to_data(musicImage);
            const song = song_from_image(musicImage.naturalWidth, musicImage.naturalHeight, music_image_data);
            try {
                input_song.value = song_to_toml(song);
            } finally {
                song_free(song);
            }
        }
        catch (e) {
            alert(e)
        }
    });

    const import_musicxml = <HTMLButtonElement>document.getElementById("import-musicxml")!;

    import_musicxml.addEventListener("change", event => {
        try {
            let files = (<HTMLInputElement>event.target).files!;
            if (files.length > 0) {
                let fileReader = new FileReader();
                fileReader.onload = musicxml => {
                    try {
                        const song = song_from_musicxml(musicxml.target!.result as string);
                        try {
                            input_song.value = song_to_toml(song);
                        }
                        finally {
                            song_free(song);
                        }
                    }
                    catch (e) {
                        alert('failed to load xml: ' + e);
                    }
                };
                fileReader.readAsText(files[0]);
            }
        } catch (e) {
            alert(e);
        }
    });

    const loadInputImage = <HTMLButtonElement>document.getElementById("load-input-image")!;

    loadInputImage.addEventListener("change", event => {
        try {
            let files = (<HTMLInputElement>event.target).files!;
            if (files.length > 0) {
                // Load all the files into data
                let url = URL.createObjectURL(files[0]);
                console.log('url ' + url);
                inputImage.src = url;
            }
        } catch (e) {
            alert(e);
        }
    });
}

run();
