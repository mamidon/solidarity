use wasmer::{imports, Function, ImportObject, Instance, Module, Store, Value};

fn main() {
    let store = Store::default();

    let module_add_one = r#"
    (module
      (type $t0 (func (param i32) (result i32)))
      (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
        get_local $p0
        i32.const 1
        i32.add))
    "#;

    let module_get_number = r#"
    (module
        (import "add" "number" (func $add (param i32) (result i32)))
        (type $t0 (func (param i32) (result i32)))
        (func $call_add (export "call_add") (type $t0) (param $p0 i32) (result i32)
        get_local $p0
        call $add)
    )
    "#;

    let a = Module::new(&store, &module_add_one).expect("compile error a");
    let b = Module::new(&store, &module_get_number).expect("compile error b");

    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {
        "env.foo.bar" => {
            "print.foo" => Function::new_native(&store, print)
        }
    };

    let instance = Instance::new(&a, &ImportObject::new()).expect("initialize error");
    let add_one_export: Function = instance
        .exports
        .get_function("add_one")
        .expect("export error")
        .clone();

    let import = imports! {
        "add" => {
            "number" => add_one_export
        }
    };
    let instance_b = Instance::new(&b, &import).expect("initialize error b");

    let call_add = instance_b
        .exports
        .get_function("call_add")
        .expect("export error");
    let result = call_add.call(&[Value::I32(3)]).expect("execute error");
    assert_eq!(result[0], Value::I32(4));
}

fn print(x: i32) -> () {
    println!("Hello, {}", x);
}
