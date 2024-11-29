const fs = require('fs')
const path = require('path')

// Normally here should be `require('ewts/nodejs')`
const {EwtsConverter} = require('../pkg/nodejs')

const converter = new EwtsConverter()

fs.promises.readFile(path.resolve(__dirname, './example_ewts_txt.txt'))
  .then((src) => {
    const ewtsText = src.toString()
    const tibetanText = converter.ewtsToUnicode(ewtsText)

    console.log('Input text is:\n', ewtsText)
    console.log('And converted is:\n', tibetanText)
  })

