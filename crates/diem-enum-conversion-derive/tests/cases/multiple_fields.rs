// Copyright © Diem Foundation

use diem_enum_conversion_derive::EnumConversion;

#[derive(EnumConversion)]
enum Messages {
    Test(String, String)
}

fn main() {

}
