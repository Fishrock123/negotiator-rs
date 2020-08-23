
macro_rules! assert_negotiate {
    ($header:expr, $accepted:expr, $expected:expr) => ({
        match ($header, $accepted, $expected) {
            (header, accepted, expected) => {
                assert_eq!(
                    negotiator::charset(header, accepted),
                    Some(expected.to_string()),
                    "header: {:?}, wanted: {:?}, expected: {:?}",
                    header,
                    accepted,
                    expected
                ); 
            }
        }
    });
}

macro_rules! assert_negotiate_none {
    ($header:expr, $accepted:expr) => ({
        match ($header, $accepted) {
            (header, accepted) => {
                assert_eq!(
                    negotiator::charset(header, accepted),
                    None,
                    "header: {:?}, wanted: {:?}, expected: {:?}",
                    header,
                    accepted,
                    None
                ); 
            }
        }
    });
}

#[test]
fn negotiator_charset () {
    assert_negotiate!(None, &[], "*");

    assert_eq!(
        negotiator::charset(None, &[]),
        Some("*".to_string()),
        "header: (none), wanted: (none), expected: \"*\""
    );

    assert_negotiate!(Some("*, UTF-8;"), &[], "*");

    assert_eq!(
        negotiator::charset(Some("*, UTF-8;"), &[]),
        Some("*".to_string()),
        "header: \"*, UTF-8;\", wanted: (none), expected: \"*\""
    );
    
    assert_negotiate!(Some("*, UTF-8;q=0"), &[], "*");

    assert_eq!(
        negotiator::charset(Some("*, UTF-8;q=0"), &[]),
        Some("*".to_string()),
        "header: \"*, UTF-8;\", wanted: (none), expected: \"*\""
    );

    assert_negotiate!(Some("ISO-8859-1"), &[], "ISO-8859-1");
    // assert_negotiate_none!(Some("UTF-8;q=0"), &[]);
    assert_negotiate!(Some("UTF-8"), &[], "UTF-8");
    assert_negotiate!(Some("UTF-8;q=0.8, ISO-8859-1"), &[], "ISO-8859-1");
    assert_negotiate!(Some("UTF-8;q=0.9, ISO-8859-1;q=0.8, UTF-8;q=0.7"), &[], "UTF-8");
    assert_negotiate!(None, &["UTF-8"], "UTF-8");
    assert_negotiate!(None, &["UTF-8", "ISO-8859-1"], "UTF-8");
    // assert_negotiate_none!(Some("*"), &[]);
    assert_negotiate!(Some("*"), &["UTF-8"], "UTF-8");
    assert_negotiate!(Some("*"), &["UTF-8", "ISO-8859-1"], "UTF-8");
    assert_negotiate!(Some("*, UTF-8"), &["UTF-8"], "UTF-8");
    assert_negotiate!(Some("*, UTF-8"), &["UTF-8", "ISO-8859-1"], "UTF-8");
    assert_negotiate!(Some("*, UTF-8;q=0"), &["UTF-8", "ISO-8859-1"], "ISO-8859-1");
    // assert_negotiate_none!(Some("*, UTF-8;q=0"), &["UTF-8"]);
    assert_negotiate!(Some("ISO-8859-1"), &["ISO-8859-1"], "ISO-8859-1");
    assert_negotiate!(Some("ISO-8859-1"), &["UTF-8", "ISO-8859-1"], "ISO-8859-1");
}


//   whenAcceptCharset('ISO-8859-1', function () {
//     it('should return matching charset', function () {
//       assert.strictEqual(this.negotiator.charset(['ISO-8859-1']), 'ISO-8859-1')
//       assert.strictEqual(this.negotiator.charset(['UTF-8', 'ISO-8859-1']), 'ISO-8859-1')
//     })

//     it('should be case insensitive, returning provided casing', function () {
//       assert.strictEqual(this.negotiator.charset(['iso-8859-1']), 'iso-8859-1')
//       assert.strictEqual(this.negotiator.charset(['iso-8859-1', 'ISO-8859-1']), 'iso-8859-1')
//       assert.strictEqual(this.negotiator.charset(['ISO-8859-1', 'iso-8859-1']), 'ISO-8859-1')
//     })

//     it('should return undefined when no matching charsets', function () {
//       assert.strictEqual(this.negotiator.charset(['utf-8']), undefined)
//     })
//   })

//   whenAcceptCharset('UTF-8;q=0', function () {
//     it('should always return undefined', function () {
//       assert.strictEqual(this.negotiator.charset(['ISO-8859-1']), undefined)
//       assert.strictEqual(this.negotiator.charset(['UTF-8', 'KOI8-R', 'ISO-8859-1']), undefined)
//       assert.strictEqual(this.negotiator.charset(['KOI8-R']), undefined)
//     })
//   })

//   whenAcceptCharset('UTF-8, ISO-8859-1', function () {
//     it('should return first matching charset', function () {
//       assert.strictEqual(this.negotiator.charset(['ISO-8859-1']), 'ISO-8859-1')
//       assert.strictEqual(this.negotiator.charset(['UTF-8', 'KOI8-R', 'ISO-8859-1']), 'UTF-8')
//     })

//     it('should return undefined when no matching charsets', function () {
//       assert.strictEqual(this.negotiator.charset(['KOI8-R']), undefined)
//     })
//   })

//   whenAcceptCharset('UTF-8;q=0.8, ISO-8859-1', function () {
//     it('should return most client-preferred charset', function () {
//       assert.strictEqual(this.negotiator.charset(['ISO-8859-1']), 'ISO-8859-1')
//       assert.strictEqual(this.negotiator.charset(['UTF-8', 'KOI8-R', 'ISO-8859-1']), 'ISO-8859-1')
//       assert.strictEqual(this.negotiator.charset(['UTF-8', 'KOI8-R']), 'UTF-8')
//     })
//   })

//   whenAcceptCharset('UTF-8;q=0.9, ISO-8859-1;q=0.8, UTF-8;q=0.7', function () {
//     it('should use highest perferred order on duplicate', function () {
//       assert.strictEqual(this.negotiator.charset(['ISO-8859-1']), 'ISO-8859-1')
//       assert.strictEqual(this.negotiator.charset(['UTF-8', 'ISO-8859-1']), 'UTF-8')
//       assert.strictEqual(this.negotiator.charset(['ISO-8859-1', 'UTF-8']), 'UTF-8')
//     })
//   })
// })

// describe('negotiator.charsets()', function () {
//   whenAcceptCharset(undefined, function () {
//     it('should return *', function () {
//       assert.deepEqual(this.negotiator.charsets(), ['*'])
//     })
//   })

//   whenAcceptCharset('*', function () {
//     it('should return *', function () {
//       assert.deepEqual(this.negotiator.charsets(), ['*'])
//     })
//   })

//   whenAcceptCharset('*, UTF-8', function () {
//     it('should return client-preferred charsets', function () {
//       assert.deepEqual(this.negotiator.charsets(), ['*', 'UTF-8'])
//     })
//   })

//   whenAcceptCharset('*, UTF-8;q=0', function () {
//     it('should exclude UTF-8', function () {
//       assert.deepEqual(this.negotiator.charsets(), ['*'])
//     })
//   })

//   whenAcceptCharset('UTF-8;q=0', function () {
//     it('should return empty list', function () {
//       assert.deepEqual(this.negotiator.charsets(), [])
//     })
//   })

//   whenAcceptCharset('ISO-8859-1', function () {
//     it('should return client-preferred charsets', function () {
//       assert.deepEqual(this.negotiator.charsets(), ['ISO-8859-1'])
//     })
//   })

//   whenAcceptCharset('UTF-8, ISO-8859-1', function () {
//     it('should return client-preferred charsets', function () {
//       assert.deepEqual(this.negotiator.charsets(), ['UTF-8', 'ISO-8859-1'])
//     })
//   })

//   whenAcceptCharset('UTF-8;q=0.8, ISO-8859-1', function () {
//     it('should return client-preferred charsets', function () {
//       assert.deepEqual(this.negotiator.charsets(), ['ISO-8859-1', 'UTF-8'])
//     })
//   })

//   whenAcceptCharset('UTF-8;foo=bar;q=1, ISO-8859-1;q=1', function () {
//     it('should return client-preferred charsets', function () {
//       assert.deepEqual(this.negotiator.charsets(), ['UTF-8', 'ISO-8859-1'])
//     })
//   })

//   whenAcceptCharset('UTF-8;q=0.9, ISO-8859-1;q=0.8, UTF-8;q=0.7', function () {
//     it.skip('should use highest perferred order on duplicate', function () {
//       assert.deepEqual(this.negotiator.charsets(), ['UTF-8', 'ISO-8859-1'])
//     })
//   })
// })

// describe('negotiator.charsets(array)', function () {
//   whenAcceptCharset(undefined, function () {
//     it('should return empty list for empty list', function () {
//       assert.deepEqual(this.negotiator.charsets([]), [])
//     })

//     it('should return original list', function () {
//       assert.deepEqual(this.negotiator.charsets(['UTF-8']), ['UTF-8'])
//       assert.deepEqual(this.negotiator.charsets(['UTF-8', 'ISO-8859-1']), ['UTF-8', 'ISO-8859-1'])
//     })
//   })

//   whenAcceptCharset('*', function () {
//     it('should return empty list for empty list', function () {
//       assert.deepEqual(this.negotiator.charsets([]), [])
//     })

//     it('should return original list', function () {
//       assert.deepEqual(this.negotiator.charsets(['UTF-8']), ['UTF-8'])
//       assert.deepEqual(this.negotiator.charsets(['UTF-8', 'ISO-8859-1']), ['UTF-8', 'ISO-8859-1'])
//     })
//   })

//   whenAcceptCharset('*, UTF-8', function () {
//     it('should return matching charsets', function () {
//       assert.deepEqual(this.negotiator.charsets(['UTF-8']), ['UTF-8'])
//       assert.deepEqual(this.negotiator.charsets(['UTF-8', 'ISO-8859-1']), ['UTF-8', 'ISO-8859-1'])
//     })
//   })

//   whenAcceptCharset('*, UTF-8;q=0', function () {
//     it('should exclude UTF-8', function () {
//       assert.deepEqual(this.negotiator.charsets(['UTF-8']), [])
//       assert.deepEqual(this.negotiator.charsets(['UTF-8', 'ISO-8859-1']), ['ISO-8859-1'])
//     })
//   })

//   whenAcceptCharset('UTF-8;q=0', function () {
//     it('should always return empty list', function () {
//       assert.deepEqual(this.negotiator.charsets(['ISO-8859-1']), [])
//       assert.deepEqual(this.negotiator.charsets(['UTF-8', 'KOI8-R', 'ISO-8859-1']), [])
//       assert.deepEqual(this.negotiator.charsets(['KOI8-R']), [])
//     })
//   })

//   whenAcceptCharset('ISO-8859-1', function () {
//     it('should return matching charsets', function () {
//       assert.deepEqual(this.negotiator.charsets(['ISO-8859-1']), ['ISO-8859-1'])
//       assert.deepEqual(this.negotiator.charsets(['UTF-8', 'ISO-8859-1']), ['ISO-8859-1'])
//     })

//     it('should be case insensitive, returning provided casing', function () {
//       assert.deepEqual(this.negotiator.charsets(['iso-8859-1']), ['iso-8859-1'])
//       assert.deepEqual(this.negotiator.charsets(['iso-8859-1', 'ISO-8859-1']), ['iso-8859-1', 'ISO-8859-1'])
//       assert.deepEqual(this.negotiator.charsets(['ISO-8859-1', 'iso-8859-1']), ['ISO-8859-1', 'iso-8859-1'])
//     })

//     it('should return empty list when no matching charsets', function () {
//       assert.deepEqual(this.negotiator.charsets(['utf-8']), [])
//     })
//   })

//   whenAcceptCharset('UTF-8, ISO-8859-1', function () {
//     it('should return matching charsets', function () {
//       assert.deepEqual(this.negotiator.charsets(['ISO-8859-1']), ['ISO-8859-1'])
//       assert.deepEqual(this.negotiator.charsets(['UTF-8', 'KOI8-R', 'ISO-8859-1']), ['UTF-8', 'ISO-8859-1'])
//     })

//     it('should return empty list when no matching charsets', function () {
//       assert.deepEqual(this.negotiator.charsets(['KOI8-R']), [])
//     })
//   })

//   whenAcceptCharset('UTF-8;q=0.8, ISO-8859-1', function () {
//     it('should return matching charsets in client-preferred order', function () {
//       assert.deepEqual(this.negotiator.charsets(['ISO-8859-1']), ['ISO-8859-1'])
//       assert.deepEqual(this.negotiator.charsets(['UTF-8', 'KOI8-R', 'ISO-8859-1']), ['ISO-8859-1', 'UTF-8'])
//     })

//     it('should return empty list when no matching charsets', function () {
//       assert.deepEqual(this.negotiator.charsets(['KOI8-R']), [])
//     })
//   })

//   whenAcceptCharset('UTF-8;q=0.9, ISO-8859-1;q=0.8, UTF-8;q=0.7', function () {
//     it('should use highest perferred order on duplicate', function () {
//       assert.deepEqual(this.negotiator.charsets(['ISO-8859-1']), ['ISO-8859-1'])
//       assert.deepEqual(this.negotiator.charsets(['UTF-8', 'ISO-8859-1']), ['UTF-8', 'ISO-8859-1'])
//       assert.deepEqual(this.negotiator.charsets(['ISO-8859-1', 'UTF-8']), ['UTF-8', 'ISO-8859-1'])
//     })
//   })
// })

// function createRequest(headers) {
//   var request = {
//     headers: {}
//   }

//   if (headers) {
//     Object.keys(headers).forEach(function (key) {
//       request.headers[key.toLowerCase()] = headers[key]
//     })
//   }

//   return request
// }
