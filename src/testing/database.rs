use std::collections::HashMap;

use crate::{component::FMComponentTable, file::FmpFile};

#[derive(Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub records: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub struct Table {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Table {
    pub fn new(table_name: String) -> Self {
        Self {
            name: table_name,
            fields: vec![]
        }
    }

    pub fn add_blank_record(&mut self) {
        for field in &mut self.fields {
            field.records.push(String::new());
        }
    }

    pub fn get_records_n(&self) -> usize {
        self.fields[0].records.len()
    }

    pub fn delete_record(&mut self, record_id: u16) {
        self.fields.remove(record_id.into());
    }
}

#[derive(Clone)]
pub struct TableOccurrence {
    pub found_set: Vec<usize>,
    pub table_ptr: u16,
    pub record_ptr: usize,
}

impl TableOccurrence {
    pub fn new() -> Self {
        Self {
            found_set: vec![],
            table_ptr: 0,
            record_ptr: 0,
        }
    }
    fn get_current_record(&self) -> usize {
        self.found_set[self.record_ptr]
    }
}

/* Database will keep:
 * - Base level records (A list of Records),
 * - Table Occurences which have their own found_set and record handles. */
pub struct Database {
    pub table_occurrences: Vec<TableOccurrence>,
    occurrence_indices: HashMap<String, u16>,
    occurrence_handle: u16,
    pub tables: Vec<Table>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            table_occurrences: vec![],
            occurrence_indices: HashMap::new(),
            occurrence_handle: 0,
            tables: vec![],
        }
    }

    pub fn clear_records(&mut self) {
        for table in &mut self.tables {
            for field in &mut table.fields {
                field.records.clear();
            }
        }
    }

    pub fn generate_from_fmp12(&mut self, file: &FmpFile) {
        self.tables.resize(file.tables.len(), Table::new("".to_string()));
        for (i, table) in file.tables.clone().into_iter().enumerate() {
            let tmp = Table {
                name: table.1.table_name.clone(),
                fields: vec![],
            };
            self.tables.insert(table.0, tmp);
            for f in &table.1.fields {
                self.tables[table.0].fields
                    .push(
                        Field {
                            name: f.1.field_name.to_string(),
                            records: vec![]
                        }
                );
            }
        }

        self.table_occurrences.resize(file.table_occurrences.len(), TableOccurrence::new());
        for (i, occurrence) in file.table_occurrences.iter().enumerate() {


                self.occurrence_indices.insert(occurrence.1.table_occurence_name.clone(),
                                           *occurrence.0 as u16);

                let tmp = TableOccurrence {
                    found_set: vec![],
                    record_ptr: 0,
                    table_ptr: occurrence.1.table_actual,
                };
                self.table_occurrences.insert(*occurrence.0, tmp);
        }

        self.occurrence_handle = self.table_occurrences.iter()
            .enumerate()
            .filter(|x| x.1.table_ptr != 0)
            .map(|x| x.0)
            .collect::<Vec<_>>()[0] as u16;
    }

    pub fn create_record(&mut self) {
        /* Rules: 
         * - When creating a record, all table_occurences with same table will add it to the found
         * set (even if it doesn't match), and update their record_ptr to look at the new record.
         */ 
        
        let table_idx = self.get_current_occurrence().table_ptr;
        let table = self.get_current_table_mut();
        table.add_blank_record();
        let n = table.get_records_n() - 1;
        for occurrence in &mut self.table_occurrences {
            if occurrence.table_ptr == table_idx {
                occurrence.found_set.push(n);
                occurrence.record_ptr = occurrence.found_set.len() - 1;
            }
        }
    }

    pub fn get_current_occurrence(&self) -> &TableOccurrence {
        &self.table_occurrences[self.occurrence_handle as usize]
    }

    pub fn get_current_occurrence_mut(&mut self) -> &mut TableOccurrence {
        &mut self.table_occurrences[self.occurrence_handle as usize]
    }

    pub fn get_current_table(&self) -> &Table {
        &self.tables[self.get_current_occurrence().table_ptr as usize]
    }

    pub fn get_current_table_mut(&mut self) -> &mut Table {
        let id = self.get_current_occurrence().table_ptr;
        &mut self.tables[id as usize]
    }

    pub fn get_table(&self, name: &str) -> Option<&Table> {
        for t in &self.tables {
            if t.name == name {
                return Some(&t);
            }
        }

        return None;
        // &self.tables[self.get_current_occurrence().table_ptr as usize]
    }

    pub fn get_table_mut(&mut self, name: &str) -> Option<&mut Table> {
        for t in &mut self.tables {
            if t.name == name {
                return Some(t);
            }
        }

        return None;
        // &self.tables[self.get_current_occurrence().table_ptr as usize]
    }

    pub fn get_records_for_current_table(&self) -> &Vec<Field> {
        &self.tables[self.get_current_occurrence().table_ptr as usize].fields
    }

    pub fn get_current_record_field(&self, field: &str) -> &str {
        let occurrence = self.get_current_occurrence();
        let id = occurrence.get_current_record();
        let table = occurrence.table_ptr;

        let field = self.tables[table as usize].fields
            .iter()
            .filter(|x| x.name == field)
            .collect::<Vec<&Field>>();

        &field[0].records[id]
    }

    pub fn get_record_by_field(&self, field: &str, record_id: usize) -> &str {
        let occurrence = self.get_current_occurrence();
        let id = occurrence.get_current_record();
        let table = occurrence.table_ptr;

        let field = self.tables[table as usize].fields
            .iter()
            .filter(|x| x.name == field)
            .collect::<Vec<&Field>>();

        &field[0].records[record_id]
    }

    pub fn get_current_record_by_field_mut(&mut self, field: &str) -> &mut str {
        let occurrence = self.get_current_occurrence();
        let id = occurrence.get_current_record();
        let table = occurrence.table_ptr;

        let field = self.tables[table as usize].fields
            .iter_mut()
            .enumerate()
            .filter(|x| x.1.name == field)
            .collect::<Vec<_>>()[0].0;

        &mut self.tables[table as usize].fields[field].records[id]
    }

    pub fn get_current_record_by_table_field(&mut self, occurrence: &str, field: &str) -> &str {

        let occurrence = &self.table_occurrences[self.occurrence_indices[occurrence] as usize];
        let id = occurrence.get_current_record();
        let table = occurrence.table_ptr;

        let field = self.tables[table as usize].fields
            .iter_mut()
            .enumerate()
            .filter(|x| x.1.name == field)
            .collect::<Vec<_>>()[0].0;

        &self.tables[table as usize].fields[field].records[id]
    }

    pub fn get_current_record_by_table_field_mut(&mut self, occurrence: &str, field: &str) -> &mut String {

        let occurrence = &self.table_occurrences[self.occurrence_indices[occurrence] as usize];
        let id = occurrence.get_current_record();
        let table = occurrence.table_ptr;

        let field = self.tables[table as usize].fields
            .iter_mut()
            .enumerate()
            .filter(|x| x.1.name == field)
            .collect::<Vec<_>>()[0].0;

        &mut self.tables[table as usize].fields[field].records[id]
    }

    /* called after a "perform_find" type script step */
    pub fn update_found_set(&mut self, records: &Vec<usize>) {
        self.get_current_occurrence_mut().found_set = records.to_vec();
    }

    pub fn goto_record(&mut self, record_id: usize) {
        let mut set = self.get_current_occurrence_mut();
        if record_id as usize >= set.found_set.len() {
            set.record_ptr = set.found_set.len() - 1;
        } else {
            set.record_ptr = record_id;
        }
    }
}

