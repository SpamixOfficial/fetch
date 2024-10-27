# Display

## Example display section in config file

```toml
...
[display]
# Gap between TextField and Logofield
#gap = INT

[display.textfield]
# Define the gap between the title and the data in the textfield
#gap = INT
walls = "|"
separator = ": "
...
```

## Parameters

| Key       | Type             | Optional           | Description                         |
| --------- | ---------------- | ------------------ | ----------------------------------- |
| gap       | Int              | :white_check_mark: | Gap between textfield and logofield |
| textfield | DisplayTextField | :x:                | Display textfield settings          |

## DisplayTextField

| Key       | Type | Optional           | Description                                |
| --------- | ---- | ------------------ | ------------------------------------------ |
| separator | Char | :white_check_mark: | Separator between key and value            |
| walls     | Char | :white_check_mark: | Walls on the side of the textfield         |
| gap       | Int  | :white_check_mark: | Gap between key and value in the textfield |
