#[cfg(test)]
mod test;

pub mod syntax;

peg_file! peg("peg.rustpeg");

pub fn try_parse(program: &str) -> Result<Vec<syntax::Command>, peg::ParseError> {
    peg::program(program)
}