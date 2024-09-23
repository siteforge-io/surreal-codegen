import https from 'https';
import fs from 'fs/promises';
import path from 'path';
import { exec } from 'child_process';

const version = process.env.npm_package_version;
const platform = process.platform;
const arch = process.arch;

let binaryName;
switch(platform) {
  case 'win32':
    binaryName = arch === 'x64' ? 'surreal-codegen-x86_64-pc-windows-gnu.tar.gz' : 'surreal-codegen-i686-pc-windows-gnu.tar.gz';
    break;
  case 'linux':
    binaryName = 'surreal-codegen-x86_64-unknown-linux-gnu.tar.gz';
    break;
  case 'darwin':
    binaryName = 'surreal-codegen-x86_64-apple-darwin.tar.gz';
    break;
  default:
    console.error('Unsupported platform:', platform);
    process.exit(1);
}

const url = `https://github.com/siteforge-io/surreal-codegen/releases/download/v${version}/${binaryName}`;
const outputPath = path.join(__dirname, binaryName);

const downloadBinary = async () => {
  try {
    const response = await new Promise((resolve, reject) => {
      https.get(url, resolve).on('error', reject);
    });

    if (response.statusCode === 302) {
      const redirectResponse = await new Promise((resolve, reject) => {
        https.get(response.headers.location, resolve).on('error', reject);
      });

      await new Promise((resolve, reject) => {
        redirectResponse.pipe(fs.createWriteStream(outputPath))
          .on('finish', resolve)
          .on('error', reject);
      });

      console.log('Binary downloaded successfully');

      await new Promise((resolve, reject) => {
        exec(`tar -xzf ${outputPath} -C ${__dirname}`, (error) => {
          if (error) {
            console.error('Error extracting binary:', error);
            reject(error);
          } else {
            resolve();
          }
        });
      });

      await fs.unlink(outputPath);
      console.log('Binary extracted successfully');
    } else {
      throw new Error('Failed to download binary');
    }
  } catch (err) {
    console.error('Error:', err.message);
    process.exit(1);
  }
};

downloadBinary();
