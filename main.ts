import init, { song_from_toml, song_free, song_samples } from './pkg/imagemusic.js';

const bloody_tears = `
ticks_per_second = 8

[[voice]]
instrument = 'Sawtooth'
notes = '''
1bes4 1f4 1f5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1des5 1f4 1c5 1f4 1bes4 1f4 1c5 1f4 1des5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1as4 1f4 1c5 1f4 1bes4 1f4 1bes4 1f4 1f5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1des5 1f4 1c5 1f4 1bes4 1f4 1c5 1f4 1des5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1as4 1f4 1c5 1f4 1bes4 1f4 2c5 1es5 2des5 1c4 1bes3 1c4 2des4 2es4 4f4 3c5 3f5 6des5 2c5 2bes4 2c5 1es5 2des5 1es4 1des4 1es4 2f4 2ges4 2as4 2bes4 3c5 5c5 3bes4 5bes4 2c5 1es5 2des5 1c4 1bes3 1c4 2des4 2es4 4f4 3c5 3f5 6des5 2c5 2bes4 2c5 1es5 2des5 1es4 1des4 1es4 2f4 2ges4 2as4 2bes4 3c5 5c5 2a4 2bes4 2c5 2es5 3es4 3des4 2des5 3es4 3des4 2des5 3es4 3des4 2des5 1des4 1des5 1c4 1c5 1bes3 1bes4 1as3 1as4 2es4 1des4 5des5 2es4 1des4 5des5 2es4 1des4 5des5 2des5 2es5 1c5 3des5 1bes4 1f4 1f5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1des5 1f4 1c5 1f4 1bes4 1f4 1c5 1f4 1des5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1as4 1f4 1c5 1f4 1bes4 1f4 1bes4 1f4 1f5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1des5 1f4 1c5 1f4 1bes4 1f4 1c5 1f4 1des5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1as4 1f4 1c5 1f4 1bes4 1f4 2c5 1es5 2des5 1c4 1bes3 1c4 2des4 2es4 4f4 3c5 3f5 6des5 2c5 2bes4 2c5 1es5 2des5 1es4 1des4 1es4 2f4 2ges4 2as4 2bes4 3c5 5c5 3bes4 5bes4 2c5 1es5 2des5 1c4 1bes3 1c4 2des4 2es4 4f4 3c5 3f5 6des5 2c5 2bes4 2c5 1es5 2des5 1es4 1des4 1es4 2f4 2ges4 2as4 2bes4 3c5 5c5 2a4 2bes4 2c5 2es5 3es4 3des4 2des5 3es4 3des4 2des5 3es4 3des4 2des5 1des4 1des5 1c4 1c5 1bes3 1bes4 1as3 1as4 2es4 1des4 5des5 2es4 1des4 5des5 2es4 1des4 5des5 2des5 2es5 1c5 3des5 1bes4 1f4 1f5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1des5 1f4 1c5 1f4 1bes4 1f4 1c5 1f4 1des5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1as4 1f4 1c5 1f4 1bes4 1f4 1bes4 1f4 1f5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1des5 1f4 1c5 1f4 1bes4 1f4 1c5 1f4 1des5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1as4 1f4 1c5 1f4 1bes4 1f4 2c5 1es5 2des5 1c4 1bes3 1c4 2des4 2es4 4f4 3c5 3f5 6des5 2c5 2bes4 2c5 1es5 2des5 1es4 1des4 1es4 2f4 2ges4 2as4 2bes4 3c5 5c5 3bes4 5bes4 2c5 1es5 2des5 1c4 1bes3 1c4 2des4 2es4 4f4 3c5 3f5 6des5 2c5 2bes4 2c5 1es5 2des5 1es4 1des4 1es4 2f4 2ges4 2as4 2bes4 3c5 5c5 2a4 2bes4 2c5 2es5 3es4 3des4 2des5 3es4 3des4 2des5 3es4 3des4 2des5 1des4 1des5 1c4 1c5 1bes3 1bes4 1as3 1as4 2es4 1des4 5des5 2es4 1des4 5des5 2es4 1des4 5des5 2des5 2es5 1c5 3des5
'''

[[voice]]
instrument = 'Sawtooth'
notes = '''
1bes5 1f5 1f6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1des6 1f5 1c6 1f5 1bes5 1f5 1c6 1f5 1des6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1as5 1f5 1c6 1f5 1bes5 1f5 1bes5 1f5 1f6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1des6 1f5 1c6 1f5 1bes5 1f5 1c6 1f5 1des6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1as5 1f5 1c6 1f5 1bes5 1f5 2es5 1as5 9f5 2es5 2des5 3es5 3as5 6f5 2es5 2des5 2es5 1as5 9f5 2es5 2f5 3ges5 5as5 3f5 5ges5 2es5 1as5 9f5 2es5 2des5 3es5 3as5 6f5 2es5 2des5 2es5 1as5 9f5 2es5 2f5 3ges5 5as5 2f5 2g5 2a5 2c6 3c5 3bes4 2bes5 3c5 3bes4 2bes5 3c5 3bes4 2bes5 1des5 1des6 1c5 1c6 1bes4 1bes5 1as4 1as5 2c5 1bes4 5bes5 2c5 1bes4 5bes5 2c5 1bes4 5bes5 2des6 2es6 1c6 3des6 1bes5 1f5 1f6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1des6 1f5 1c6 1f5 1bes5 1f5 1c6 1f5 1des6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1as5 1f5 1c6 1f5 1bes5 1f5 1bes5 1f5 1f6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1des6 1f5 1c6 1f5 1bes5 1f5 1c6 1f5 1des6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1as5 1f5 1c6 1f5 1bes5 1f5 2es5 1as5 9f5 2es5 2des5 3es5 3as5 6f5 2es5 2des5 2es5 1as5 9f5 2es5 2f5 3ges5 5as5 3f5 5ges5 2es5 1as5 9f5 2es5 2des5 3es5 3as5 6f5 2es5 2des5 2es5 1as5 9f5 2es5 2f5 3ges5 5as5 2f5 2g5 2a5 2c6 3c5 3bes4 2bes5 3c5 3bes4 2bes5 3c5 3bes4 2bes5 1des5 1des6 1c5 1c6 1bes4 1bes5 1as4 1as5 2c5 1bes4 5bes5 2c5 1bes4 5bes5 2c5 1bes4 5bes5 2des6 2es6 1c6 3des6 1bes5 1f5 1f6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1des6 1f5 1c6 1f5 1bes5 1f5 1c6 1f5 1des6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1as5 1f5 1c6 1f5 1bes5 1f5 1bes5 1f5 1f6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1des6 1f5 1c6 1f5 1bes5 1f5 1c6 1f5 1des6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1as5 1f5 1c6 1f5 1bes5 1f5 2es5 1as5 9f5 2es5 2des5 3es5 3as5 6f5 2es5 2des5 2es5 1as5 9f5 2es5 2f5 3ges5 5as5 3f5 5ges5 2es5 1as5 9f5 2es5 2des5 3es5 3as5 6f5 2es5 2des5 2es5 1as5 9f5 2es5 2f5 3ges5 5as5 2f5 2g5 2a5 2c6 3c5 3bes4 2bes5 3c5 3bes4 2bes5 3c5 3bes4 2bes5 1des5 1des6 1c5 1c6 1bes4 1bes5 1as4 1as5 2c5 1bes4 5bes5 2c5 1bes4 5bes5 2c5 1bes4 5bes5 2des6 2es6 1c6 3des6
'''

[[voice]]
instrument = 'Sawtooth'
notes = '''
16bes3 8f3 8f4 16bes3 8f3 8f4 2bes2 1bes3 1bes2 1r 1bes2 2bes3 1bes2 1bes2 2bes3 2bes2 2bes3 2as2 1as3 1as2 1r 1as2 2as3 2as2 2as3 1as2 1as2 2as3 2ges2 1ges3 1ges2 1r 1ges2 2ges3 1ges2 1ges2 2ges3 2ges2 2ges3 1f2 1f2 1f3 1f2 1r 1f2 2f3 2f2 2f3 1f2 1f2 2f3 2bes2 1bes3 1bes2 1r 1bes2 2bes3 1bes2 1bes2 2bes3 2bes2 2bes3 2as2 1as3 1as2 1r 1as2 2as3 2as2 2as3 1as2 1as2 2as3 2ges2 1ges3 1ges2 1r 1ges2 2ges3 1ges2 1ges2 2ges3 2ges2 2ges3 1f2 1f2 1f3 1f2 1r 1f2 2f3 1f2 1f3 1g2 1g3 1a2 1a3 1c3 1c4 3bes2 3bes2 1bes2 1bes3 3as2 3as2 1as2 1as3 3ges2 3ges2 1ges2 1ges3 3as2 3as2 1as2 1as3 3bes2 3bes2 1bes2 1bes3 3as2 3as2 1as2 1as3 3ges2 3ges2 1ges2 1ges3 2as2 2as3 1as2 1as2 2as3 16bes3 8f3 8f4 16bes3 8f3 8f4 2bes2 1bes3 1bes2 1r 1bes2 2bes3 1bes2 1bes2 2bes3 2bes2 2bes3 2as2 1as3 1as2 1r 1as2 2as3 2as2 2as3 1as2 1as2 2as3 2ges2 1ges3 1ges2 1r 1ges2 2ges3 1ges2 1ges2 2ges3 2ges2 2ges3 1f2 1f2 1f3 1f2 1r 1f2 2f3 2f2 2f3 1f2 1f2 2f3 2bes2 1bes3 1bes2 1r 1bes2 2bes3 1bes2 1bes2 2bes3 2bes2 2bes3 2as2 1as3 1as2 1r 1as2 2as3 2as2 2as3 1as2 1as2 2as3 2ges2 1ges3 1ges2 1r 1ges2 2ges3 1ges2 1ges2 2ges3 2ges2 2ges3 1f2 1f2 1f3 1f2 1r 1f2 2f3 1f2 1f3 1g2 1g3 1a2 1a3 1c3 1c4 3bes2 3bes2 1bes2 1bes3 3as2 3as2 1as2 1as3 3ges2 3ges2 1ges2 1ges3 3as2 3as2 1as2 1as3 3bes2 3bes2 1bes2 1bes3 3as2 3as2 1as2 1as3 3ges2 3ges2 1ges2 1ges3 2as2 2as3 1as2 1as2 2as3 16bes3 8f3 8f4 16bes3 8f3 8f4 2bes2 1bes3 1bes2 1r 1bes2 2bes3 1bes2 1bes2 2bes3 2bes2 2bes3 2as2 1as3 1as2 1r 1as2 2as3 2as2 2as3 1as2 1as2 2as3 2ges2 1ges3 1ges2 1r 1ges2 2ges3 1ges2 1ges2 2ges3 2ges2 2ges3 1f2 1f2 1f3 1f2 1r 1f2 2f3 2f2 2f3 1f2 1f2 2f3 2bes2 1bes3 1bes2 1r 1bes2 2bes3 1bes2 1bes2 2bes3 2bes2 2bes3 2as2 1as3 1as2 1r 1as2 2as3 2as2 2as3 1as2 1as2 2as3 2ges2 1ges3 1ges2 1r 1ges2 2ges3 1ges2 1ges2 2ges3 2ges2 2ges3 1f2 1f2 1f3 1f2 1r 1f2 2f3 1f2 1f3 1g2 1g3 1a2 1a3 1c3 1c4 3bes2 3bes2 1bes2 1bes3 3as2 3as2 1as2 1as3 3ges2 3ges2 1ges2 1ges3 3as2 3as2 1as2 1as3 3bes2 3bes2 1bes2 1bes3 3as2 3as2 1as2 1as3 3ges2 3ges2 1ges2 1ges3 2as2 2as3 1as2 1as2 2as3
'''

[[voice]]
instrument = 'Sawtooth'
notes = '''
1r 1bes4 1f4 1f5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1des5 1f4 1c5 1f4 1bes4 1f4 1c5 1f4 1des5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1as4 1f4 1c5 1f4 1bes4 1f4 1bes4 1f4 1f5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1des5 1f4 1c5 1f4 1bes4 1f4 1c5 1f4 1des5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1as4 1f4 1c5 1f4 1bes4 2c5 1es5 2des5 1c4 1bes3 1c4 2des4 2es4 4f4 3c5 3f5 6des5 2c5 2bes4 2c5 1es5 2des5 1es4 1des4 1es4 2f4 2ges4 2as4 2bes4 3c5 5c5 3bes4 5bes4 2c5 1es5 2des5 1c4 1bes3 1c4 2des4 2es4 4f4 3c5 3f5 6des5 2c5 2bes4 2c5 1es5 2des5 1es4 1des4 1es4 2f4 2ges4 2as4 2bes4 3c5 5c5 2a4 2bes4 2c5 2es5 3es4 3des4 2des5 3es4 3des4 2des5 3es4 3des4 2des5 1des4 1des5 1c4 1c5 1bes3 1bes4 1as3 1as4 2es4 1des4 5des5 2es4 1des4 5des5 2es4 1des4 5des5 2des5 2es5 1c5 3des5 1r 1bes4 1f4 1f5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1des5 1f4 1c5 1f4 1bes4 1f4 1c5 1f4 1des5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1as4 1f4 1c5 1f4 1bes4 1f4 1bes4 1f4 1f5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1des5 1f4 1c5 1f4 1bes4 1f4 1c5 1f4 1des5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1as4 1f4 1c5 1f4 1bes4 2c5 1es5 2des5 1c4 1bes3 1c4 2des4 2es4 4f4 3c5 3f5 6des5 2c5 2bes4 2c5 1es5 2des5 1es4 1des4 1es4 2f4 2ges4 2as4 2bes4 3c5 5c5 3bes4 5bes4 2c5 1es5 2des5 1c4 1bes3 1c4 2des4 2es4 4f4 3c5 3f5 6des5 2c5 2bes4 2c5 1es5 2des5 1es4 1des4 1es4 2f4 2ges4 2as4 2bes4 3c5 5c5 2a4 2bes4 2c5 2es5 3es4 3des4 2des5 3es4 3des4 2des5 3es4 3des4 2des5 1des4 1des5 1c4 1c5 1bes3 1bes4 1as3 1as4 2es4 1des4 5des5 2es4 1des4 5des5 2es4 1des4 5des5 2des5 2es5 1c5 3des5 1r 1bes4 1f4 1f5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1des5 1f4 1c5 1f4 1bes4 1f4 1c5 1f4 1des5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1as4 1f4 1c5 1f4 1bes4 1f4 1bes4 1f4 1f5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1des5 1f4 1c5 1f4 1bes4 1f4 1c5 1f4 1des5 1f4 1es5 1f4 1des5 1f4 1c5 1f4 1as4 1f4 1c5 1f4 1bes4 2c5 1es5 2des5 1c4 1bes3 1c4 2des4 2es4 4f4 3c5 3f5 6des5 2c5 2bes4 2c5 1es5 2des5 1es4 1des4 1es4 2f4 2ges4 2as4 2bes4 3c5 5c5 3bes4 5bes4 2c5 1es5 2des5 1c4 1bes3 1c4 2des4 2es4 4f4 3c5 3f5 6des5 2c5 2bes4 2c5 1es5 2des5 1es4 1des4 1es4 2f4 2ges4 2as4 2bes4 3c5 5c5 2a4 2bes4 2c5 2es5 3es4 3des4 2des5 3es4 3des4 2des5 3es4 3des4 2des5 1des4 1des5 1c4 1c5 1bes3 1bes4 1as3 1as4 2es4 1des4 5des5 2es4 1des4 5des5 2es4 1des4 5des5 2des5 2es5 1c5 3des5
'''

[[voice]]
instrument = 'Sawtooth'
notes = '''
1r 1bes5 1f5 1f6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1des6 1f5 1c6 1f5 1bes5 1f5 1c6 1f5 1des6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1as5 1f5 1c6 1f5 1bes5 1f5 1bes5 1f5 1f6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1des6 1f5 1c6 1f5 1bes5 1f5 1c6 1f5 1des6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1as5 1f5 1c6 1f5 1bes5 2es5 1as5 9f5 2es5 2des5 3es5 3as5 6f5 2es5 2des5 2es5 1as5 9f5 2es5 2f5 3ges5 5as5 3f5 5ges5 2es5 1as5 9f5 2es5 2des5 3es5 3as5 6f5 2es5 2des5 2es5 1as5 9f5 2es5 2f5 3ges5 5as5 2f5 2g5 2a5 2c6 3c5 3bes4 2bes5 3c5 3bes4 2bes5 3c5 3bes4 2bes5 1des5 1des6 1c5 1c6 1bes4 1bes5 1as4 1as5 2c5 1bes4 5bes5 2c5 1bes4 5bes5 2c5 1bes4 5bes5 2des6 2es6 1c6 3des6 1r 1bes5 1f5 1f6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1des6 1f5 1c6 1f5 1bes5 1f5 1c6 1f5 1des6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1as5 1f5 1c6 1f5 1bes5 1f5 1bes5 1f5 1f6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1des6 1f5 1c6 1f5 1bes5 1f5 1c6 1f5 1des6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1as5 1f5 1c6 1f5 1bes5 2es5 1as5 9f5 2es5 2des5 3es5 3as5 6f5 2es5 2des5 2es5 1as5 9f5 2es5 2f5 3ges5 5as5 3f5 5ges5 2es5 1as5 9f5 2es5 2des5 3es5 3as5 6f5 2es5 2des5 2es5 1as5 9f5 2es5 2f5 3ges5 5as5 2f5 2g5 2a5 2c6 3c5 3bes4 2bes5 3c5 3bes4 2bes5 3c5 3bes4 2bes5 1des5 1des6 1c5 1c6 1bes4 1bes5 1as4 1as5 2c5 1bes4 5bes5 2c5 1bes4 5bes5 2c5 1bes4 5bes5 2des6 2es6 1c6 3des6 1r 1bes5 1f5 1f6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1des6 1f5 1c6 1f5 1bes5 1f5 1c6 1f5 1des6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1as5 1f5 1c6 1f5 1bes5 1f5 1bes5 1f5 1f6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1des6 1f5 1c6 1f5 1bes5 1f5 1c6 1f5 1des6 1f5 1es6 1f5 1des6 1f5 1c6 1f5 1as5 1f5 1c6 1f5 1bes5 2es5 1as5 9f5 2es5 2des5 3es5 3as5 6f5 2es5 2des5 2es5 1as5 9f5 2es5 2f5 3ges5 5as5 3f5 5ges5 2es5 1as5 9f5 2es5 2des5 3es5 3as5 6f5 2es5 2des5 2es5 1as5 9f5 2es5 2f5 3ges5 5as5 2f5 2g5 2a5 2c6 3c5 3bes4 2bes5 3c5 3bes4 2bes5 3c5 3bes4 2bes5 1des5 1des6 1c5 1c6 1bes4 1bes5 1as4 1as5 2c5 1bes4 5bes5 2c5 1bes4 5bes5 2c5 1bes4 5bes5 2des6 2es6 1c6 3des6
'''

[[voice]]
instrument = 'Sawtooth'
notes = '''
16bes3 8f3 8f4 16bes3 8f3 8f4 16r 16r 16r 16r 16r 16r 16r 16r 8bes4 8as4 8ges4 8as4 8bes5 8as5 8ges5 8as5 16bes3 8f3 8f4 16bes3 8f3 8f4 16r 16r 16r 16r 16r 16r 16r 16r 8bes4 8as4 8ges4 8as4 8bes5 8as5 8ges5 8as5 16bes3 8f3 8f4 16bes3 8f3 8f4 16r 16r 16r 16r 16r 16r 16r 16r 8bes4 8as4 8ges4 8as4 8bes5 8as5 8ges5 8as5
'''
`;

async function run() {
    await init();

    const audioCtx = new window.AudioContext();

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
    source.start();
}

run();