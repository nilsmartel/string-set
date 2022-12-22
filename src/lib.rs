#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

const N: usize = 4;

type Index = u32;

pub struct StringSet {
    /// Pointer to the concatenated byte array, representing the set of all strings
    dictionary: Vec<u8>,

    /// Pointer to start of every 4th String inside dictionary.
    values: Vec<Index>,
}

impl StringSet {
    pub fn new(mut values: Vec<String>) -> StringSet {
        values.sort();

        todo!()
    }

    fn encode_strs(values: &[String]) -> Vec<u8> {
        let mut buffer = Vec::new();

        let str1 = values[0].as_bytes();

        // write length of first str to buffer
        buffer.extend(StringSet::encode_number(str1.len() as u16));

        buffer.extend(str1);

        for s in &values[1..] {
            let s = s.as_bytes();

            let prefixlen = str1.iter().zip(s).take_while(|(a, b)| **a == **b).count();

            // cut of common part from s
            let s = &s[prefixlen..];
            let prefixlen = prefixlen as u16;
            let strlen = s.len() as u16;

            // now we write
            // [common prefix length][length of remaining str][remaining str]
            // to the buffer
            buffer.extend(StringSet::encode_number(prefixlen));
            buffer.extend(StringSet::encode_number(strlen));
            buffer.extend(s);
        }

        buffer
    }

    fn encode_number(n: u16) -> Vec<u8> {
        if let Ok(n) = n.try_into() {
            vec![n]
        } else {
            let a = (n >> 8) as u8;
            let b = (n & 0xff) as u8;
            vec![0xff, a, b]
        }
    }

    fn decode_number(input: &[u8]) -> (u16, &[u8]) {
        match input.get(0) {
            Some(255) => {
                let a = input[1];
                let b = input[2];
                let result = (a as u16) << 8 | b as u16;
                (result, &input[2..])
            }
            // no input left
            _ => (0, input),
        }
    }
}
