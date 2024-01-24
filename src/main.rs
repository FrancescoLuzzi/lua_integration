use mlua::{self, IntoLua, Lua, Value};

use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Person {
    pub name: String,
    pub age: i32,
    pub city: String,
}

impl Person {
    fn new(name: String, age: i32, city: String) -> Self {
        Person { name, age, city }
    }
}

impl TryFrom<mlua::Table<'_>> for Person {
    type Error = mlua::Error;
    fn try_from(value: mlua::Table) -> std::result::Result<Self, Self::Error> {
        let name: String = value.get("name")?;
        let age: i32 = value.get("age")?;
        let city: String = value.get("city")?;
        Ok(Self { name, age, city })
    }
}

impl<'a> IntoLua<'a> for Person {
    fn into_lua(self, lua: &'a Lua) -> Result<mlua::Value<'a>, mlua::Error> {
        let table = lua.create_table()?;
        table.set("name", self.name)?;
        table.set("age", self.age)?;
        table.set("city", self.city)?;
        Ok(Value::Table(table))
    }
}

fn main() -> Result<(), mlua::Error> {
    // Create a Lua context.
    let lua = Lua::new();
    let globals = lua.globals();

    // Open and read the Lua script from a file.
    let mut lua_script = String::new();
    File::open("config.lua")
        .unwrap()
        .read_to_string(&mut lua_script)
        .unwrap();
    let mut global_person = Person::new("default".to_string(), 0, "default".to_string());

    lua.load(lua_script).exec()?;

    // Get the global 'config' table from Lua.
    let tmp_person: Person = globals.get::<_, mlua::Table>("Person")?.try_into()?;

    global_person.name = tmp_person.name.clone();
    global_person.age = tmp_person.age;
    global_person.city = tmp_person.city.clone();

    let printo: mlua::Function = lua.globals().get("PrintPerson")?;
    let result: String = printo.call(tmp_person)?;
    println!("Result from lua:\n{result}");

    // Modify the global configuration if needed.
    // For example, you can update it with new values or provide access to other threads.

    // Access the global_config and print it.
    println!("Printing from Rust:\n{:?}", global_person);

    Ok(())
}
