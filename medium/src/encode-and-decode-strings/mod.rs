pub struct Codec {

}

impl Codec {
    fn new() -> Self {
        Self{}
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut vec = vec![];

        for str in strs.iter() {
            vec.push(format!("{:020}{}", str.len(), str))
        }

        vec.join("")
    }

    fn decode(&self, s: String) -> Vec<String> {
        let mut vec = vec![];
        let mut i = 0;

        while i < s.len() {
            let num = (&s[i..i+20]).parse::<usize>().unwrap();
            let ss = s[i+20..i+20+num].to_string();
            vec.push(ss);
            i = i + 20 + num;
        }

        vec
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;

    let obj = Codec::new();
    let strs = vec_stringify!(vec!["123", "321"]);
    let encoded = obj.encode(strs);
    let decoded = obj.decode(encoded);
    let res = vec_stringify!(vec!["123", "321"]);
    assert_eq!(decoded, res);
}
