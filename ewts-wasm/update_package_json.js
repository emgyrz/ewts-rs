#!/usr/bin/env node

const fs = require('fs')
const path = require('path');


['./pkg', './pkg/nodejs', './pkg/web']
  .forEach(dir => {
    const pckgPath = path.join(__dirname, dir, 'package.json')
    const pckg = require(pckgPath);

    renameMainFilesAsIndex(pckg)

    fs.writeFileSync(pckgPath, JSON.stringify(pckg, null, 2))
  })


function renameMainFilesAsIndex(pckg) {
  pckg.name = 'ewts'
  pckg.files = renameFilesInArr(pckg.files)

  pckg.main = "index.js"
  pckg.types = "index.d.ts"

  if (pckg.sideEffects) {
    pckg.sideEffects = renameFilesInArr(pckg.sideEffects)
  }

  function renameFilesInArr(arr) {
    return arr.map(fName => {
      if (fName.match(/((ewts\.js)|(ests\.d\.ts))$/)) {
        return fName.replace('ewts', 'index')
      }
      return fName
    })
  }
}
