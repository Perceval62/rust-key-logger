mod input_system;
mod macros;
extern crate libc;

fn test_callback()
{
    println!("Callback 1");
}

fn test_callback_2()
{
    println!("Callback 2");
}

fn main() {
    let mut input = input_system::InputSystem::new().unwrap();

    // attaching a fonction to a certain key press
    input.attach_listener(macros::KEY_Y, &test_callback);
    input.attach_listener(macros::KEY_U, &test_callback_2);

    /* Example without using the listener feature */
    loop{
        let key = input.read_key();
        let key = match key
        {
            Some(character) => character,
            None => continue,
        };

        /* example of usage*/
        if key == macros::KEY_A
        {
            println!("A was pressed !");
        }

        if key == macros::KEY_ESC
        {
            println!("Escaping !");
            break;
        }
    }
}