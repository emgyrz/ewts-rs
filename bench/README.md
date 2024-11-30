
## Speed comparison with other converters

I do not know who will need to transliterate large amounts of text, 
but still I want to mention the difference in the speed of some implementations. 
Especially considering that the current implementation is several times faster than others 
(from 12 to almost 100 times).

| Tool                                                                 | Speed               | Launch code                                                                                        |
| -------------------------------------------------------------------- | ------------------- | -------------------------------------------------------------------------------------------------- |
| ewts-rs (in rust)                                                    | ~26491 Kb/s (1.00x) | [bench/main.rs](https://github.com/emgyrz/ewts-rs/blob/master/bench/src/main.rs)                   |
| [jsewts](https://github.com/buda-base/jsewts)                        | ~2141 Kb/s (12.4x)  | [js_tools_bench.js](https://github.com/emgyrz/ewts-rs/blob/master/bench/js_tools_bench.js)         |
| [ewts-js](https://github.com/rogerespel/ewts-js)                     | ~1941 Kb/s (13.6x)  | [js_tools_bench.js](https://github.com/emgyrz/ewts-rs/blob/master/bench/js_tools_bench.js)         |
| ewts-rs (as wasm)                                                    | ~14598 Kb/s (1.81x) | [js_tools_bench.js](https://github.com/emgyrz/ewts-rs/blob/master/bench/js_tools_bench.js)         |
| [ewts-converter (java)](https://github.com/buda-base/ewts-converter) | ~1822 Kb/s (14.54x) | [java_bench.java](https://github.com/emgyrz/ewts-rs/blob/master/bench/java_bench.java)             |
| [pyewts](https://github.com/OpenPecha/pyewts)                        | ~274 Kb/s (96.7x)   | [python_tools_bench.py](https://github.com/emgyrz/ewts-rs/blob/master/bench/python_tools_bench.py) |


All libraries participating in the comparison converted text into Tibetan 33 times. 
The text was taken from [this](https://github.com/emgyrz/ewts-rs/blob/master/bench/sample_ewts_text.txt) file


## How to run it yourself

The following programs should be available from your terminal:
- cargo
- git
- node, npm
- python3, pip
- java
- curl

If you have it all, then clone this repo, go to `bench` directory and simply run `./run_bench.sh`
```sh
git clone https://github.com/emgyrz/ewts-rs.git
cd ./ewts-rs/bench
./run_bench.sh
```

On my machine it looks like this:
![bench screenshot](https://github.com/user-attachments/assets/8b88c21c-6881-490a-a168-fa412e4e5dc2)

