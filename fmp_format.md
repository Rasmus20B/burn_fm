# FMP12 File Format Findings

- Tables: [3].[16].[5]
- Relationships: [3].[17].[5]
- Fields: [table].[3].[5]
- Layout info: [4].[1].[7]
- Scripts: [17].[5].[script]
- value lists: [33].[5].[valuelist]


## Field type switches

idx

0:

1:  Simple Data-types.  
- 'X' = Number,
- '[' = Text,
- 'Y' = Date,
- '^' = Time,
- '\_' = Timestamp,
- '\\' = Container, 

2:

3:

4:  Auto-Enter preset Options.  
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


5:

10:                     
- 2: Serial Number **On Commit**,
- 4: Set in conjunction with idx 11: 128 to signify lookup,


11:                     
- 1 = Options flag from idx 4,
- 2 = Serial Number **On Creation**,
- 4 = Data textbox,
- 8 = Auto-Enter Calculation (**does not** replace existing value),
- 16 = Value from last visited record,
- 128 = Set with idx 10: 4 to signify lookup,
- 136 = Auto-Enter Calculation (**does** replace existing value),




        

