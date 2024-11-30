#!/usr/bin/env node

const fs = require('fs')
const path = require('path')

const jsewts = require('jsewts')
const tibEwtsConverterMod = require('tibetan-ewts-converter')
const ewtsWasm = require('../ewts-wasm/pkg/nodejs')

const ewtsText = fs.readFileSync(path.join(__dirname, './sample_ewts_text.txt')).toString()
const fSize = fs.statSync('./sample_ewts_text.txt').size
const ITERATION_COUNT = 33

run('jsewts', s => jsewts.fromWylie(s))

const tibEwtsConverter = new tibEwtsConverterMod.EwtsConverter()
run('tibetan-ewts-converter', s => tibEwtsConverter.to_unicode(s))

const ewtsWasmConverter = new ewtsWasm.EwtsConverter()
run('ewts-rs (wasm)', s => ewtsWasmConverter.ewtsToUnicode(s))


function run(name, convertFn) {
  let totalCharsCount = 0

  const start = performance.now()
  for(let i = 0; i < ITERATION_COUNT; i++) {
    totalCharsCount += convertFn(ewtsText).length
  }
  const elapsedMs = performance.now() - start
  const elapsedS = elapsedMs / 1000
  const speed = (fSize * ITERATION_COUNT / 1024) / elapsedS;
  console.log(`${name}: speed - ${speed.toFixed(3)} Kb/s; launches - ${ITERATION_COUNT}; time - ${Math.round(elapsedMs)} ms`)
}

