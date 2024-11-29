#!/usr/bin/env node

const fs = require('fs')
const path = require('path')

const pckgPath = path.join(__dirname, 'pkg/package.json')
const pckg = require(pckgPath)

pckg.name = "ewts";

["ewts.js", "ewts.d.ts"].forEach(p => pckg.files.splice(pckg.files.indexOf(p), 1) );
["index.js", "index.d.ts", "index.node.js", "index.web.js"]
  .forEach(p => pckg.files.push(p) );

pckg.main = "index.js"
pckg.types = "index.d.ts"
pckg.sideEffects = ["index.js"]

fs.writeFileSync("pkg/package.json", JSON.stringify(pckg, null, 2))

