[![PyPI](https://img.shields.io/pypi/v/promptml)](https://pypi.org/project/promptml/)
[![license](https://img.shields.io/github/license/AlongWY/promptml.svg?maxAge=86400)](LICENSE)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/AlongWY/promptml/CI)

# [PromptML](https://github.com/AlongWY/promptml)

Prompt Markup Language Parser.

# PromptML

+ PromptML is a simple markup language.
+ It inserts control strings in common string, wrapped by `[]`, (e. g. `[mask]`, `[sep]`)
+ The control strings can have some options, seperated with string by `|`, (e. g. `[title|upper]`, `[text|lower]`)
+ The control options can be multiple, seperated by `,`, (e. g. `[title|upper,rmpunt]`)

# Examples will be prased

1. `[cls]A [mask] news : [sent_0|lower,fix][sep|+]`
    1. String: `cls`       Control Options: {}
    2. String: `A `        Control Options: None
    3. String: `mask`      Control Options: {}
    4. String: ` news : `  Control Options: None
    5. String: `sent_0`    Control Options: {`lower`, `fix`}
    6. String: `sep`       Control Options: {`+`}
2. `[cls]\\[ Topic : [mask] \\][sent_0][sep|+]`
    1. String: `cls`       Control Options: {}
    2. String: `[ Topic : `Control Options: None
    3. String: `mask`      Control Options: {}
    4. String: ` ]`        Control Options: None
    5. String: `sent_0`    Control Options: {}
    6. String: `sep`       Control Options: {`+`}