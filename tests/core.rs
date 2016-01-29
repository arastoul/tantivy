extern crate tantivy;
extern crate itertools;
extern crate byteorder;
extern crate regex;

use tantivy::core::postings::{VecPostings, intersection};
use tantivy::core::postings::Postings;
use tantivy::core::analyzer::tokenize;
use tantivy::core::serial::*;
use tantivy::core::schema::*;
use tantivy::core::codec::SimpleCodec;
use tantivy::core::global::*;
use tantivy::core::writer::IndexWriter;
use tantivy::core::directory::{Directory, generate_segment_name, SegmentId};
use std::ops::DerefMut;
use tantivy::core::writer::SimplePostingsWriter;
use tantivy::core::postings::PostingsWriter;
use tantivy::core::reader::SegmentIndexReader;
use std::io::{ BufWriter, Write};
use regex::Regex;
use std::convert::From;
use std::path::PathBuf;
use tantivy::core::query;
use tantivy::core::query::{parse_query, BoolExpr};
#[test]
fn test_parse_query() {
    // let left = VecPostings::new(vec!(1, 3, 9));
    // let right = VecPostings::new(vec!(3, 4, 9, 18));
    // let inter = intersection(&left, &right);
    // let vals: Vec<DocId> = inter.iter().collect();
    // assert_eq!(vals, vec!(3, 9));
    {
        let (parsed_query, _) = parse_query("toto:titi toto:tutu").unwrap();
        assert_eq!(parsed_query, BoolExpr::Conjunction(vec!(query::Term(String::from("toto"), String::from("titi")), query::Term(String::from("toto"), String::from("tutu")))));
    }
}

#[test]
fn test_intersection() {
    let left = VecPostings::new(vec!(1, 3, 9));
    let right = VecPostings::new(vec!(3, 4, 9, 18));
    let inter = intersection(&left, &right);
    let vals: Vec<DocId> = inter.iter().collect();
    assert_eq!(vals, vec!(3, 9));
}

#[test]
fn test_tokenizer() {
    let words: Vec<&str> = tokenize("hello happy tax payer!").collect();
    assert_eq!(words, vec!("hello", "happy", "tax", "payer"));
}

#[test]
fn test_indexing() {
    let directory = Directory::from_tempdir().unwrap();
    {
        // writing the segment
        let mut index_writer = IndexWriter::open(&directory);
        {
            let mut doc = Document::new();
            doc.set(Field(1), "af b");
            index_writer.add(doc);
        }
        {
            let mut doc = Document::new();
            doc.set(Field(1), "a b c");
            index_writer.add(doc);
        }
        {
            let mut doc = Document::new();
            doc.set(Field(1), "a b c d");
            index_writer.add(doc);
        }

        let commit_result = index_writer.commit();
        println!("{:?}", commit_result);
        assert!(commit_result.is_ok());
        // reading the segment
        println!("------");
        {
            let segment = commit_result.unwrap();
            let index_reader = SegmentIndexReader::open(segment).unwrap();
            let mut term_cursor = index_reader.term_cursor();
            loop {
                match term_cursor.next() {
                    Some((term, doc_cursor)) => {
                        println!("{:?}", term);
                        for doc in doc_cursor {
                            println!("  Doc {}", doc);
                        }
                    },
                    None => {
                        break;
                    },
                }
            }
        }
    }
    {
        // TODO add index opening stuff
        // let index_reader = IndexReader::open(&directory);
    }
}




#[test]
fn test_new_segment() {
    let SegmentId(segment_name) = generate_segment_name();
    let segment_ptn = Regex::new(r"^_[a-z0-9]{8}$").unwrap();
    assert!(segment_ptn.is_match(&segment_name));
}
