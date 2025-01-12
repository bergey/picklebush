# picklebush

## one-shot mode

Include the text of the Cucumber step on the command line.
`picklebush` will print matching lines and exit.

```
$ picklebush --dir test_data  dill
test_data/patterns.js::3
```

## interactive mode

If no Cucumber text is given on the command line, `picklebush` will listen on `stdin`.  Each line of input is matched against the Cucumber regexen.  This is faster if you want to match multiple steps, because `picklebush` only loads the regexen from disk once.  (This also means that changes to the regexen after `picklebush` starts will not be detected.)

```
$ picklebush --dir test_data 
enter text to match against regex patterns
gherkin
test_data/patterns.js::1
dill
test_data/patterns.js::3
half sour
test_data/patterns.js::2
```

## picklebush --help

```
find regexen that would match a given text

Usage: picklebush [OPTIONS] [TEXT]

Arguments:
  [TEXT]  

Options:
      --debug            
  -d, --dir <DIRECTORY>  [default: .]
  -h, --help             Print help
  -V, --version          Print version
```
