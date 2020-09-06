use cycloneddscodegen as codegen;

fn main() {

  let idls = vec!["idl/idl1.idl"];
  codegen::generate_and_compile_datatypes(idls);

}
