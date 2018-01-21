/* eslint-disable no-console */

const fs = require('fs');
const os = require('os');
const archiver = require('archiver'); // eslint-disable-line import/no-extraneous-dependencies

function getExecutableExtension() {
  if (os.platform() === 'win32') {
    return '.exe';
  }
  return '';
}

const executableName = `yore${getExecutableExtension()}`;

function createArchive() {
  const output = fs.createWriteStream('dist/yore.zip');
  const archive = archiver('zip', {
    zlib: { level: 9 }
  });

  output.on('close', () => {
    console.log(`${archive.pointer()} total bytes`);
    console.log(
      'archiver has been finalized and the output file descriptor has closed.'
    );
  });

  output.on('end', () => {
    console.log('Data has been drained');
  });

  archive.on('warning', err => {
    if (err.code === 'ENOENT') {
      console.warn(err);
    } else {
      // throw error
      throw err;
    }
  });

  archive.on('error', err => {
    throw err;
  });

  archive.pipe(output);

  archive.file(`target/release/${executableName}`, { name: executableName });
  archive.file('dist/app.bundle.js', { name: 'app.bundle.js' });
  archive.file('dist/index.html', { name: 'index.html' });
  archive.file('dist/style.css', { name: 'style.css' });

  archive.finalize();
}

createArchive();
