local naming = require("naming")
-- config.lua
Person = {
    name = naming.name(),
    age = 24,
    city = "Bologna - Italy"
}

function PrintPerson(person)
    return "Hello, my name is " .. person.name ..
        ". I am " .. person.age ..
        " years old and live in " .. person.city .. "."
end
