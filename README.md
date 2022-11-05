# Convert IATI XML to JSON

##  Install

```
pip install iati2json
```

##  Usage

### Save to file (fastest option)

```
import iati2json
iati2json.convert("my_iati_file.xml", file="converted.json")

```

### Convert to String

```
import iati2json
import json
iati_json_string = iati2json.convert("my_iati_file.xml")
iati_dict = json.loads(iati_json_string)
```

### Pretty (indented output)

```
import iati2json
iati2json.convert("my_iati_file.xml", file="converted.json", pretty=True)

```
