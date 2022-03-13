# PromptML

Prompt Markup Language Parser.

# Examples

1. `[cls]A [mask] news : [sent_0|lower,fix][sep|+]`
    1. Control Key:  `cls`
    2. String:       `A `
    3. Control Key:  `mask`
    4. String:       ` news : `
    5. Control Key:  `sent_0` Control Options: `lower`, `fix`
    6. Control Key:  `sep`    Control Options: `+`
2. `[cls]\\[ Topic : [mask] \\][sent_0][sep|+]`
    1. Control Key:  `cls`
    2. String:       `[ Topic : `
    3. Control Key:  `mask`
    4. String:       ` ]`
    5. Control Key:  `sent_0`
    6. Control Key:  `sep`    Control Options: `+`