use kes::parser::parse;

fn main() {
    let scripts = vec!["foo.kes", "bar.kes", "baz.kes"];

    // 여기서 Instruction이 어떻게 수명을 갖는가?
    let compiled_sources: Vec<Vec<Instruction<'_>>> = scripts
        .map(|script| {
            // ...
            parse(todo!())
        })
        .collect();
}
