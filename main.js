import init, { parse_song } from './pkg/asciimusic.js';

async function run() {
    await init();

    const song = parse_song('Gb0RCBwJNq5wO8V321jbhkdCeVPMd9ur5EaiJmr7BGDeR0Mb4hxF3W9_f6q-n41jYjQYsHjQVufiYnWdr4E1G8sLXITiNs5Ea61dGVvkoWjbNEnuMx-v0wAg_ZySmxrzRxtdSB5SQzGSZfYkrRPO9F7omZticeLs6Nyl2_sYL_0kjpjv_Obi1qNn72h8eHRon-vTluLddkncg12IM3ZE9t7dXI6kfw1SjCfaWi8MhuV0k4nwQ-jCSmpsqEftcuPJal3LmYqFnrlWrPilxcniFV-QXh0zXXaKpSej7ygtr6wZj6rcg4kWa5DKWxGjU6Vf6bZes1bJ5EcivwPb4w3uI3cyHRUApqONXFB1ewBc--r0oK1iCADMLLQtG3QUVDAA4GZBNYr56oLTL2u4YziwnWdOgD1OB-x-KmApjKfihq7HyjomP0_6rxjkGqpINGibAUG20EAsrWQyLLJnwx-mrg3d931WrbrFzXy5M_aj');
    console.log(song);
}

run();
