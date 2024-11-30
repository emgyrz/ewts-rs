#!/usr/bin/python3

import time
from pathlib import Path

from deps.pyewts import pyewts


fpath = Path('./sample_ewts_text.txt')
ewts_text = fpath.read_text()
fsize=fpath.stat().st_size

ITERATION_COUNT = 33

converter = pyewts()

start = time.time()

total_chars_count = 0

for x in range(0, ITERATION_COUNT):
    total_chars_count += len(converter.toUnicode(ewts_text))

elapsed = time.time() - start

speed = (fsize * float(ITERATION_COUNT) / 1024.0) / float(elapsed)

print(f'pyewts: speed - {str(round(speed, 3))} Kb/s ; launches - {ITERATION_COUNT}; time: {round(elapsed * 1000)} ms')


