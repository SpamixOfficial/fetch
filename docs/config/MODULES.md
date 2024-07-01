# Modules

## Example modules section in config file

```toml
...

[modules]
# Define the modules, essentially tells the program "Okay here are the modules, and put them in this order
modules = ["userhost", "separator", "shell", "os", "kernel", "roof", "test"]

# Definitions for the modules to make them usable. Yes, even inbuilt needs definitions
# Order doesnt matter here, just that they have a correct syntax!
definitions = [
  { name = "roof", separator_char = '-', format = "{1}", type = "separator", disable_walls = true },
  { name = "separator", separator_char = '-', type = "separator" },
  { name = "kernel", key = "KERNEL", type = "kernel" },
  { name = "shell", key = "SHELL", type = "shell" },
  { name = "userhost", format = "{1}@{2}", type = "userhost" },
  { name = "os", key = "OS", type = "os" },
  { name = "test", key = "TEST", type = "custom", execute = [
    "uname",
    "-a",
  ], format = "{1}!{2}" },
]
```

## Modules table structure

In the module **_table_** we have 2 important variables: "modules" and "definitions"

The modules **_variable_** contains the modules you want to use in order, in form of an array of the module names. The names are those you specify using the _"name"_ variable in every definitions table

The definitions variable is an array of inline tables. The definitions are literally definitions, they tell the program what the names in the modules **_variable_** actually means!

These inline tables are pretty customizable. The documentation for them is located down below.

Please take a look at the TOML example located [here](#example-modules-section-in-config-file) to fully understand how everything should look. The example includes some comments to make this easier

## Currently implemented modules

| Name              | Type      | Format                                   | Exclusive Parameters |
| ----------------- | --------- | ---------------------------------------- | -------------------- |
| Operating System  | os        | {PRETTY_NAME(1)}{VERSION_ID(2)}{Arch(3)} | :x:                  |
| Kernel            | kernel    | {kernel(1)}                              | :x:                  |
| User and Hostname | userhost  | {user(1)}{hostname(1)}                   | :x:                  |
| Shell             | shell     | {shell_path(1)}                          | :x:                  |
| Separator         | separator | {separator_character(1)}                 | separator_char       |
| Custom            | custom    |                                          | execute              |

> [!IMPORTANT]  
> At the moment theres no format for custom modules

How to read format string:

```text
{ x (i) }
^ ^  ^  ^
1 2  3  4

1: Opening character
2: Name of the format part
3: the index you use to use the format
4: Closing character
```

## Parameters

| Key            | Type          | Optional           | Description                                                                       |
| -------------- | ------------- | ------------------ | --------------------------------------------------------------------------------- |
| name           | String        | :x:                | Name of the module                                                                |
| key            | String        | :white_check_mark: | Key of module in output                                                           |
| format         | String        | :white_check_mark: | Format string - docs [here](#currently-implemented-modules)                       |
| separator_char | Char          | :white_check_mark: | Separator character - exclusive to separator module                               |
| walls          | bool          | :white_check_mark: | If walls should be enabled. Defaults to true                                      |
| type           | ModuleType    | :x:                | Type of module. List of modules [here](#module-types)                             |
| execute        | Array[String] | :white_check_mark: | Execute command, split by space into array of string - exclusive to custom module |
