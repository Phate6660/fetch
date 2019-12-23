use std::vec::Vec;
use prettytable::{ cell, format, row, Table };

#[derive(Clone)]
struct KeyValue {
    key: String,
    val: String,
}

#[derive(PartialEq, Clone, Debug)]
pub enum OutputType {
    Minimal,
    Rsfetch,
    Neofetch,
}

pub struct OutputOptions {
    pub output_type: OutputType,
    pub caps:        bool,
    pub bold:        bool,
    pub use_borders: bool,
    pub borders:     char,
}

pub struct OutputHelper {
    table:   Table,
    ascii:   String,
    options: OutputOptions,
    data:    Vec<KeyValue>,
}

pub fn bold(text: &str) -> String {
    let e: char = 0x1B as u8 as char;
    format!("{}[1m{}{}[0m", e, text, e)
}

impl OutputHelper {

    // initialize new OutputHelper
    pub fn new(options: OutputOptions) -> OutputHelper {
        let mut table = Table::new();
        let bdr = if options.output_type == OutputType::Minimal {
            ' '
        } else { '│' };

        let sep = if options.output_type == OutputType::Minimal {
            format::LineSeparator::new(' ', ' ', 
                                       options.borders, options.borders)
        } else {
            format::LineSeparator::new('─', '─',
                                       options.borders, options.borders)
        };

        let format = format::FormatBuilder::new()
            .column_separator(' ')
            .borders(bdr)
            .separators(
                &[format::LinePosition::Top, format::LinePosition::Bottom],
                sep,
            )
            .padding(1, 1)
            .build();
        table.set_format(format);

        OutputHelper {
            table:   table,
            ascii:   String::new(),
            options: options,
            data:    Vec::new(),
        }
    }

    pub fn add(&mut self, key: &str, val: &str) {
        let item = KeyValue {
            key: key.to_owned(),
            val: val.to_owned(),
        };

        self.data.push(item);
    }

    pub fn ascii(&mut self, ascii: String) {
        self.ascii = ascii;
    }

    pub fn output(&mut self) {
        // minimal output style
        if self.options.output_type == OutputType::Minimal {
            for thing in self.data.clone() {
                println!("{}", thing.val);
            }
        } else if self.options.output_type == OutputType::Rsfetch {
            // print logo
            println!("{}", bold(&self.ascii));

            // convert self.data to table, then print
            for thing in self.data.clone() {
                let mut key = thing.key.clone();
                let val = thing.val.clone();

                if !self.options.caps {
                    key = key.to_lowercase();
                }

                if self.options.bold {
                    key = bold(&key);
                }

                if !self.options.use_borders {
                    self.table.add_row(row![key, val]);
                } else {
                    self.table.add_row(row![key, "=", val]);
                }
            }
            self.table.printstd();
        } else if self.options.output_type == OutputType::Neofetch {
            // don't do anything
            // TODO: implement
        }
    }
}