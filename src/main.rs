mod input_system;
mod lookup_tables;
extern crate libc;

fn test_callback()
{
    println!("YEEETTT");
}

fn test_callback_2()
{
    println!("YEEETTT");
}

fn main() {
    let mut input = input_system::InputSystem::new().unwrap();
    input.attach_listener(lookup_tables::KEY_Y, &test_callback);
    input.attach_listener(lookup_tables::KEY_U, &test_callback_2);
    loop{
        let key = input.read_key();
        let key = match key
        {
            Some(character) => character,
            None => continue,
        };

        if key == lookup_tables::KEY_A
        {
            println!("A was pressed !");
        }

        if key == lookup_tables::KEY_ESC
        {
            println!("Escaping !");
            break;
        }
    }
}