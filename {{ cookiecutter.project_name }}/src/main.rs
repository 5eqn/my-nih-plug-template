use nih_plug::prelude::*;

use gain_gui_vizia::{{ cookiecutter.struct_name }};

fn main() {
    nih_export_standalone::<{{ cookiecutter.struct_name }}>();
}
