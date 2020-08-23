//
// negotiator
// Copyright(c) 2012 Isaac Z. Schlueter
// Copyright(c) 2014 Federico Romero
// Copyright(c) 2014-2015 Douglas Christopher Wilson
// Copyright(c) 2020 Jeremiah Senkpiel
// MIT Licensed
//

use std::cmp::Ordering;

use regex::Regex;

const SIMPLE_CHARSET: &'static str = r"^\s*([^\s;]+)\s*(?:;(.*))?$";

struct Charset {
    charset: String,
    q: isize,
    i: usize,
}

#[derive(PartialEq)]
struct Priority {
    i: Option<usize>,
    o: isize,
    q: isize,
    s: isize,
}

impl Default for Priority {
    fn default() -> Self {
        Self {
            i: None,
            o: -1,
            q: 0,
            s: 0,
        }
    }
}

/// Parse the Accept-Charset header.
fn parse_accept_charset(accept: &str) -> Vec<Charset> {
    let accepts = accept.split(',');
    let mut parsed = Vec::new();

    let mut i = 0;
    for set in accepts {
        if let Some(charset) = parse_charset(set, i) {
            parsed.push(charset);
        }

        i += 1;
    }

    parsed
}

/// Parse a charset from the Accept-Charset header.
fn parse_charset(set: &str, i: usize) -> Option<Charset> {
    let charset_match = Regex::new(SIMPLE_CHARSET).unwrap();
    let captures = charset_match.captures(set)?;

    let charset = captures.get(0)?.as_str().to_string();
    let mut q = 1;
    if let Some(opts) = captures.get(1) {
        for param in opts.as_str().split(';') {
            let parts: Vec<&str> = param.trim().split('=').collect();
            if parts.len() == 2 && parts[0] == "q" {
                q = parts[1].parse().unwrap_or(1);
            }
        }
    }

    Some(Charset { charset, q, i })
}

/// Get the priority of a charset.
fn get_charset_priority(charset: &str, accepted: &Vec<Charset>, index: usize) -> Priority {
    let mut priority = Priority::default();

    for accept in accepted {
        if let Some(spec) = specify(charset, &accept, index) {
            if priority.s - spec.s < 0 || priority.q - spec.q < 0 || priority.o - spec.o < 0 {
                priority = spec
            }
        }
    }

    priority
}

/// Get the specificity of the charset.
fn specify(charset: &str, spec: &Charset, index: usize) -> Option<Priority> {
    let mut s = 0;
    if spec.charset.to_lowercase() == charset.to_lowercase() {
        s |= 1;
    } else if spec.charset != "*" {
        return None;
    }

    Some(Priority {
        i: Some(index),
        o: spec.i as isize,
        q: spec.q,
        s,
    })
}

/// Get the preferred charsets from an Accept-Charset header.
pub fn preferred_charsets(accept: Option<&str>, provided: &[&str]) -> Vec<String> {
    // RFC 2616 sec 14.2: no header = *
    let accept = accept.unwrap_or("*");

    let accepts = parse_accept_charset(accept);

    if provided.len() == 0 {
        // sorted list of all charsets
        let mut filtered = accepts
            .iter()
            .filter(|spec| spec.q > 0) // Does the spec have any quality?
            .collect::<Vec<&Charset>>();
        filtered.sort_by(compare_charsets);
        return filtered.iter().map(get_full_charset).collect();
    }

    let mut priorities: Vec<Priority> = provided
        .iter()
        .enumerate()
        .map(|(index, prov)| {
            return get_charset_priority(prov, &accepts, index);
        })
        .filter(|spec| spec.q > 0) // Does the spec have any quality?
        .collect();

    // sorted list of accepted charsets
    priorities.sort_by(compare_priority);
    priorities
        .iter()
        .map(|priority| {
            return provided[priorities.iter().position(|p| p == priority).unwrap()].to_owned();
        })
        .collect()
}

/// Compare two Charsets.
fn compare_charsets<'l, 'r>(a: &'l &Charset, b: &'r &Charset) -> Ordering {
    // (b.q - a.q) || (b.s - a.s) || (a.o - b.o) || (a.i - b.i) || 0;

    let q = (b.q - a.q).cmp(&0);
    let i = (a.i - b.i).cmp(&0);

    if q != Ordering::Equal {
        q
    } else if i != Ordering::Equal {
        i
    } else {
        Ordering::Equal
    }
}

/// Compare two Priorities.
fn compare_priority<'l, 'r>(a: &'l Priority, b: &'r Priority) -> Ordering {
    // (b.q - a.q) || (b.s - a.s) || (a.o - b.o) || (a.i - b.i) || 0;

    let q = (b.q - a.q).cmp(&0);
    let s = (b.s - a.s).cmp(&0);
    let o = (a.o - b.o).cmp(&0);
    let i = (a.i.unwrap_or(0) - b.i.unwrap_or(0)).cmp(&0);

    if q != Ordering::Equal {
        q
    } else if s != Ordering::Equal {
        s
    } else if o != Ordering::Equal {
        o
    } else if i != Ordering::Equal {
        i
    } else {
        Ordering::Equal
    }
}

/// Get full charset string.
fn get_full_charset(spec: &&Charset) -> String {
    spec.charset.to_owned()
}
