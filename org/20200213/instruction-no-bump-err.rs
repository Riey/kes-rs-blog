use kes::parser::parse;
use std::fs::read_to_string;

fn main() {
    let scripts = vec!["foo.kes", "bar.kes", "baz.kes"];

    // 여기서 Instruction이 어떻게 수명을 갖는가?
    let compiled_sources: Vec<Vec<Instruction<'_>>> = scripts
        .map(|script| {
            let source = read_to_string(script).expect("Can't read source");
            parse(&source) // 로컬변수 source의 레퍼런스를 리턴하려고함 E0515
        })
        .collect();
}
