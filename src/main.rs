use rlua::{self, Lua, ToLua, Value};

use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

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

impl TryFrom<rlua::Table<'_>> for Person {
    type Error = rlua::Error;
    fn try_from(value: rlua::Table) -> std::result::Result<Self, Self::Error> {
        let name: String = value.get("name")?;
        let age: i32 = value.get("age")?;
        let city: String = value.get("city")?;
        Ok(Self { name, age, city })
    }
}

impl ToLua<'_> for Person {
    fn to_lua(self, ctx: rlua::Context) -> Result<rlua::Value<'_>, rlua::Error> {
        let table = ctx.create_table()?;
        table.set("name", self.name)?;
        table.set("age", self.age)?;
        table.set("city", self.city)?;
        Ok(Value::Table(table))
    }
}
fn main() -> Result<(), rlua::Error> {
    // Create a Lua context.
    let lua = Lua::new();

    // Open and read the Lua script from a file.
    let mut lua_script = String::new();
    File::open("config.lua")
        .unwrap()
        .read_to_string(&mut lua_script)
        .unwrap();
    let global_person = Rc::new(RefCell::new(Person::new(
        "default".to_string(),
        0,
        "default".to_string(),
    )));

    // Execute the Lua script.
    lua.context(|lua| {
        lua.load(&lua_script).exec()?;

        // Get the global 'config' table from Lua.
        let tmp_person: Person = lua.globals().get::<_, rlua::Table>("Person")?.try_into()?;

        let mut conf = global_person.borrow_mut();
        conf.name = tmp_person.name.clone();
        conf.age = tmp_person.age;
        conf.city = tmp_person.city.clone();

        let printo: rlua::Function = lua.globals().get("PrintPerson")?;
        let result: String = printo.call(tmp_person)?;
        println!("Result from lua:\n{result}");

        // Modify the global configuration if needed.
        // For example, you can update it with new values or provide access to other threads.

        // Return the global_config wrapped in Result.
        Ok(())
    })?;

    // Access the global_config and print it.
    let config = global_person.borrow();
    println!("Printing from Rust:\n{:?}", *config);

    Ok(())
}
