# Kana learning

This (quick and dirty) tool aims to ease learning to read Hiragana and Katakana.

## Requirements

* you must install fonts that support displaying kanas in your terminal. For example for Arch:

```yay -Sy noto-fonts-cjk noto-fonts-emoji noto-fonts```

## Usage

Start by building the tool:

`cargo build`

Next, the files `hiragana.csv` and `katakana.csv` must contains the symbols list you want to learn. You can add or remove symbols depending on your learning step.
You can add new symbols to the file by copy-pasting characters from corresponding Unicode tables (you can find those tables [here](https://en.wikipedia.org/wiki/Hiragana_(Unicode_block)) and [here](https://en.wikipedia.org/wiki/Katakana_(Unicode_block))).
Once you lists are tuned, you can run the tool:

- `./target/debug/kana_learning` to learn Hiraganas
- `./target/debug/kana_learning -k` to learn Katakanas

The tool will display a random kana, and wait for your input, which must be the corresponding "mora" (a,i, u, e, o, ka, ki...). The tool will keep questioning you in an infinite loop
