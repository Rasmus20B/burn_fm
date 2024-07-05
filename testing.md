# Testing with Burn FM

Need to emulate filemaker. Because of this, Testing with Burn is intended to
verify *Business Logic*, and does not necessarily cover possible bugs or
limitations within FileMaker itself.

Take in the "BurnFM Attributes" in the "fmp\_file" struct.

```
for each test:
    copy handles to connected attributes from fmp\_file
    generate tables for input data and immutable tables for output data
    perform operations specified in test, using input as a base
    Compare input and output data stores
    return if they are the same of not.

```


An example of a test might be:

```
test BasicTest:
    input: {
        basic_table_occurence: [
            
        ],
        [

        ],
    },
    scripts: {
        adding(),
        removing(),
    },
    output: {
        basic_table_occurence: [

        ],
        extended_table_occurence: [

        ]
    }
```
