# Burn Language Specification

## Symbols

- IDENT => Identifier. Examples: "Person", "PrimaryKey"
- [] => a list. Examples: [FIELD] = a list of fields, [STRING, ","] = comma seperated list of strings
- {} => Optional

- "export" after script definition will allow the script to be passed to the final file.
    - Reasons to not use export may include: preamples for tests which require
      some basic navigation logic.

## Table Definition
1. TABLE => "table" -> IDENT -> ":\n" -> [FIELD] -> "\nend table;"
2. FIELD => within table scope -> IDENT -> [FIELD\_OPTION, ","] -> {","}
3. FIELD\_OPTION => {DESCRIPTION = "description"}, {"Unique"}, {"Required"},
    DATA\_TYPE = one of ["Text", "Number", "Date", "Time", "Timestamp", "Container"],
    {AUTO\_CALC = "{calculation goes here}"},
    {VALIDATION\_CALC = "!{validation goes here}"},
    {VALIDATION\_MESSAGE = !"failed validation message goes here"},
4. RELATIONSHIP => "relationship" -> IDENTIFIER -> ":\n" -> TABLE1 -> ":" -> TABLE2 ->
    [COMPARISON] -> "\nend relationship;"
5. COMPARISON => FIELDNAME from TABLE1 
    -> one of ["==", "<=", ">=", "<", ">", "!=", "O"] 
    -> FIELDNAME from TABLE2
6. VALUE\LIST => "value\_list" -> IDENT -> ":\n" -> [STRING, ","] -> "\nend value\_list;"

7. SCRIPT => "script" -> {export} -> ":" -> scripting language
