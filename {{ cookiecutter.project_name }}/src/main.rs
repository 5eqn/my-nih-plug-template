use nih_plug::prelude::*;

use {{ cookiecutter.project_name }}::{{ cookiecutter.struct_name }};

fn main() {
    nih_export_standalone::<{{ cookiecutter.struct_name }}>();
}
