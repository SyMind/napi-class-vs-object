/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'napi-class-vs-object.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi-class-vs-object.android-arm64.node')
          } else {
            nativeBinding = require('napi-class-vs-object-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'napi-class-vs-object.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi-class-vs-object.android-arm-eabi.node')
          } else {
            nativeBinding = require('napi-class-vs-object-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'napi-class-vs-object.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi-class-vs-object.win32-x64-msvc.node')
          } else {
            nativeBinding = require('napi-class-vs-object-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'napi-class-vs-object.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi-class-vs-object.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('napi-class-vs-object-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'napi-class-vs-object.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi-class-vs-object.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('napi-class-vs-object-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'napi-class-vs-object.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./napi-class-vs-object.darwin-universal.node')
      } else {
        nativeBinding = require('napi-class-vs-object-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'napi-class-vs-object.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi-class-vs-object.darwin-x64.node')
          } else {
            nativeBinding = require('napi-class-vs-object-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'napi-class-vs-object.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi-class-vs-object.darwin-arm64.node')
          } else {
            nativeBinding = require('napi-class-vs-object-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'napi-class-vs-object.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./napi-class-vs-object.freebsd-x64.node')
      } else {
        nativeBinding = require('napi-class-vs-object-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'napi-class-vs-object.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./napi-class-vs-object.linux-x64-musl.node')
            } else {
              nativeBinding = require('napi-class-vs-object-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'napi-class-vs-object.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./napi-class-vs-object.linux-x64-gnu.node')
            } else {
              nativeBinding = require('napi-class-vs-object-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'napi-class-vs-object.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./napi-class-vs-object.linux-arm64-musl.node')
            } else {
              nativeBinding = require('napi-class-vs-object-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'napi-class-vs-object.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./napi-class-vs-object.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('napi-class-vs-object-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'napi-class-vs-object.linux-arm-musleabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./napi-class-vs-object.linux-arm-musleabihf.node')
            } else {
              nativeBinding = require('napi-class-vs-object-linux-arm-musleabihf')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'napi-class-vs-object.linux-arm-gnueabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./napi-class-vs-object.linux-arm-gnueabihf.node')
            } else {
              nativeBinding = require('napi-class-vs-object-linux-arm-gnueabihf')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'riscv64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'napi-class-vs-object.linux-riscv64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./napi-class-vs-object.linux-riscv64-musl.node')
            } else {
              nativeBinding = require('napi-class-vs-object-linux-riscv64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'napi-class-vs-object.linux-riscv64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./napi-class-vs-object.linux-riscv64-gnu.node')
            } else {
              nativeBinding = require('napi-class-vs-object-linux-riscv64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 's390x':
        localFileExisted = existsSync(
          join(__dirname, 'napi-class-vs-object.linux-s390x-gnu.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi-class-vs-object.linux-s390x-gnu.node')
          } else {
            nativeBinding = require('napi-class-vs-object-linux-s390x-gnu')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { createObject1, setObject1, createObject2, setObject2, createObject3, setObject3, createObject4, setObject4, createObject5, setObject5, createObject6, setObject6, createObject7, setObject7, createObject8, setObject8, createObject9, setObject9, createObject10, setObject10, Class1, createClass1, setClass1, Class2, createClass2, setClass2, Class3, createClass3, setClass3, Class4, createClass4, setClass4, Class5, createClass5, setClass5, Class6, createClass6, setClass6, Class7, createClass7, setClass7, Class8, createClass8, setClass8, Class9, createClass9, setClass9, Class10, createClass10, setClass10 } = nativeBinding

module.exports.createObject1 = createObject1
module.exports.setObject1 = setObject1
module.exports.createObject2 = createObject2
module.exports.setObject2 = setObject2
module.exports.createObject3 = createObject3
module.exports.setObject3 = setObject3
module.exports.createObject4 = createObject4
module.exports.setObject4 = setObject4
module.exports.createObject5 = createObject5
module.exports.setObject5 = setObject5
module.exports.createObject6 = createObject6
module.exports.setObject6 = setObject6
module.exports.createObject7 = createObject7
module.exports.setObject7 = setObject7
module.exports.createObject8 = createObject8
module.exports.setObject8 = setObject8
module.exports.createObject9 = createObject9
module.exports.setObject9 = setObject9
module.exports.createObject10 = createObject10
module.exports.setObject10 = setObject10
module.exports.Class1 = Class1
module.exports.createClass1 = createClass1
module.exports.setClass1 = setClass1
module.exports.Class2 = Class2
module.exports.createClass2 = createClass2
module.exports.setClass2 = setClass2
module.exports.Class3 = Class3
module.exports.createClass3 = createClass3
module.exports.setClass3 = setClass3
module.exports.Class4 = Class4
module.exports.createClass4 = createClass4
module.exports.setClass4 = setClass4
module.exports.Class5 = Class5
module.exports.createClass5 = createClass5
module.exports.setClass5 = setClass5
module.exports.Class6 = Class6
module.exports.createClass6 = createClass6
module.exports.setClass6 = setClass6
module.exports.Class7 = Class7
module.exports.createClass7 = createClass7
module.exports.setClass7 = setClass7
module.exports.Class8 = Class8
module.exports.createClass8 = createClass8
module.exports.setClass8 = setClass8
module.exports.Class9 = Class9
module.exports.createClass9 = createClass9
module.exports.setClass9 = setClass9
module.exports.Class10 = Class10
module.exports.createClass10 = createClass10
module.exports.setClass10 = setClass10
