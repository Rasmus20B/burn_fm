# FMP12 File Format Findings

## File Tree Structure

- Tables: [3].[16].[5]
- Relationships: [3].[17].[5]
- Fields: [table].[3].[5]
- Layout info: [4].[1].[7]
- Scripts: [17].[5].[script]
- value lists: [33].[5].[valuelist]


## Field type switches (Found at key 2 for field definition)

### Byte Index

### 0: Type of field (not data type, but simple, calculation, or summary)
- 0 = Simple field,
- 2 = Calculation field,
- 3 = Summary field,

### 1:  Simple Data-types.  
- 1 = Text,
- 2 = Number,
- 3 = Date,
- 4 = Time,
- 5 = Timestamp,
- 6 = Container, 

### 1: Summary Field Data-types.
- 1 = List of,
- 2 = Total of || Count of || Standard Deviation || Fraction of Total of,
- 5 = Average || Minimum || Maximum,

### 4:  Auto-Enter preset Options.  
- 0 = Creation Date,
- 1 = Creation Time,
- 2 = Creation TimeStamp,
- 3 = Create Name,
- 4 = Creation Account Name,
- 5 = Modification Date,
- 6 = Modification Time,
- 7 = Modification TimeStamp,
- 8 = Modification Name,
- 9 = Modification Account Name,

### 5:

### 7: Default language

2. Unicode,
3. Default,
16. Catalan,
17. Croatian,
18. Czech,
19. Danish,
20. Dutch,
21. English,
22. Finnish
23. Finnish (v\<\>w),
24. French,
25. German,
26. German (ä=a),
27. Greek,
28. Hungarian,
29. Icelandic,
30. Italian,
31. Japanese,
32. Norwegian,
33. Polish,
34. Portuguese,
35. Romanian,
36. Russian,
37. Slovak,
38. Slovenian,
39. Spanish (Modern),
40. Spanish,
41. Swedish,
42. Swedish (v\<\>w),
43. Turkish,
44. Ukrainian,
45. Chinese (Pinyin),
46. Chinese (Stroke),
47. Hebrew,
48. Hindi,
49. Arabic,
50. Estonian,
51. Lithuanian,
52. Latvian,
53. Serbian (Latin),
54. Farsi, 
55. Bulgarian,
56. Vietnamese,
57. Thai,
58. Greek (Mixed),
59. Bengali,
60. Telugu,
61. Marathi,
62. Tamil,
63. Gujarati,
64. Kannada,
65. Malayalam
67. Panjabi,
76. Korean,


### 8:
- 64 = Don't automatically create index,
- 128 = ALways index this field,

### 9:
- 0 = regular storage,
- 1 = Global Field,
- 8 = Calculation field,
- 10 = Unstored Calculation,

### 10:                     
- 1: Prohibit modification of value during data-entry
- 2: Serial Number **On Commit**,
- 4: Set in conjunction with idx 11: 128 to signify lookup,

### 11:
- 1 = Options flag from idx 4,
- 2 = Serial Number **On Creation**,
- 4 = Data textbox,
- 8 = Auto-Enter Calculation (**does not** replace existing value),
- 16 = Value from last visited record,
- 32 = Evaluate Calculation even if all referenced fields are empty, 
- 128 = with idx 10 = 4 it is a lookup that is active, otherwise inactive but data saved elsewhere,
- 136 = Auto-Enter Calculation (**does** replace existing value),

### 14:
- 0 = Only validate during data entry,
- 1 = Member of value list,
- 2 = maximum number of characters,
- 4 = always validate,
- 16 = strict data-type: Numeric only,
- 32 = strict data-type: 4 digit year,
- 64 = strict data-type: Time of Day,
        
### 15:
- 0 = User can override,
- 1 = Validated by calculation,
- 4 = User cannot override,
- 8 = Required value,
- 16 = Unique Value,
- 32 = Existing Value,
- 64 = within a range of values,
- 128 = Display a validation error message,

### 25:
- byte 25 simply states how many repeitiions the field has

## Scripting Structure

### [17].[5].[script]::4 
- This path stores the instructions from the script, some metadata, as well as
the file value to append to the path above.
- Each step is stored as a 24 byte subarray, most commonly starting with '2, 1'. 
- The 3rd byte of each subarray can be used to index the "instruction directory"

#### List of Instructions

1. Perform Script

4. Go to Next Field

5. Go to Previous Field

6. Go to Layout
    - bytes 7, 8, and 9: Specify layout. If "original layout" these are all zero.

11. Insert From Index 
13. Insert Current Date
14. Insert Current Time
16. Go to Record/Request/Page
17. Go to Field
22. Enter Find Mode
41. Enter Preview Mode
45. Undo/Redo
46. Cut
47. Copy
48. Paste
49. Clear
50. Select All
55. Enter Browser Mode
60. Insert Current User Name
62. Pause/Resume Script
68. If
69. Else
70. End If
71. Loop 
72. Exit Loop If
73. End Loop
74. Go to Related Record
77. Insert Calculated Result
85. Allow User abort. 26th byte option
    - 26: 1 = Off, 3 = On. 
86. Set Error Capture
89. Blank Line
90. Halt Script
99. Go to Portal Row
103. Exit Script 
125. Else If
128. Perform Find/Replace
130. Set Selection
131. Insert File
132. Export Field Contents
141. Set Variable
145. Go to Object
148. Install OnTimer Script
159. Insert Audio/Video
161. Insert From Device
164. Perform Script on Server
168. Set Layout Object Animation
169. Close Popover
185. Configure Region Monitor Script
187. Configure Local Notification 
200. Set Error Logging
201. Configure NFC Reading
202. Configure Machine Learning Model
205. Open Transaction
206. Commit Transaction
207. Revert Transaction
210. Perform Script on Server with Callback
211. Trigger Claris Connect Flow

### [17].[5].[script].[5] - The Instruction Directory
- The "data" for each script step is located in this folder.

