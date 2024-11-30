#!/usr/bin/env node

const fs = require('fs')
const path = require('path');


const pckgs = ['.', 'nodejs', 'web']
  .map(dir => {
    const pckg = require(pckgPath(dir));

    renameMainFilesAsIndex(pckg)
    
    return { dir, pckg }
  })

pckgs.slice(1).forEach(({dir, pckg}) => {
  pckg.files.forEach(f => {
    pckgs[0].pckg.files.push(dir + '/' + f)
  })
  pckgs[0].pckg.files.push(dir + '/package.json')
})

pckgs.forEach(({dir, pckg}) => {
  fs.writeFileSync(pckgPath(dir), JSON.stringify(pckg, null, 2))
})

function pckgPath(dir) {
  return path.join(__dirname, './pkg', dir, 'package.json')
}

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
      if (fName.match(/((ewts\.js)|(ewts\.d\.ts))$/)) {
        return fName.replace('ewts', 'index')
      }
      return fName
    })
  }
}
