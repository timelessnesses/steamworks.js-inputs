const path = require('node:path');
const fs = require('node:fs');
const child_process = require('node:child_process');

const targets = {
    ['x86_64-pc-windows-msvc']: {
        folder: 'win64',
        files: ['steam_api64.dll', 'steam_api64.lib'],
        platform: 'win32',
        arch: 'x64'
    },
    ['x86_64-unknown-linux-gnu']: {
        folder: 'linux64',
        files: ['libsteam_api.so'],
        platform: 'linux',
        arch: 'x64'
    },
    ['x86_64-apple-darwin']: {
        folder: 'osx',
        files: ['libsteam_api.dylib'],
        platform: 'darwin',
        arch: 'x64'
    },
    ['aarch64-apple-darwin']: {
        folder: 'osx',
        files: ['libsteam_api.dylib'],
        platform: 'darwin',
        arch: 'arm64'
    }
}

const target = targets[process.argv.at(-1)] || Object.values(targets).find(t => t.platform === process.platform && t.arch === process.arch)

const dist = path.join(__dirname, 'dist', target.folder)
const redist = path.join(__dirname, 'sdk/redistributable_bin', target.folder)
target.files.forEach(file => {
    const [source, dest] = [path.join(redist, file), path.join(dist, file)]
    try { fs.mkdirSync(path.dirname(dest), { recursive: true }) } catch { }
    fs.copyFileSync(source, dest)
})

const relative = path.relative(process.cwd(), dist)
const params = [
    'build',
    '--platform',
    '--no-dts-header',
    '--dts', '../../client.d.ts',
    '-o', relative,
    process.argv.slice(2).join(' ')
]

child_process.spawn('napi', params, { stdio: 'inherit', shell: true })
    .on('exit', err => {
        if (err) {
            throw err;
        }
        
fs.copyFileSync(path.join(__dirname, "./index.d.ts"), path.join(dist, "index.d.ts"));
fs.copyFileSync(path.join(__dirname, "index.js"), path.join(dist, "index.js"));
fs.copyFileSync(path.join(__dirname, "client.d.ts"), path.join(dist, "client.d.ts"));
    })
