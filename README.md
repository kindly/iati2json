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

### Custom Iati Schemas

By default will work with 2.03 version of standard. You can provide your own versions of schemas:

```
import iati2json

schemas = [
    "https://raw.githubusercontent.com/IATI/IATI-Schemas/version-2.04-beta/iati-activities-schema.xsd",
    "https://raw.githubusercontent.com/IATI/IATI-Schemas/version-2.04-beta/iati-organisations-schema.xsd",
]

iati2json.convert("my_iati_file.xml", schemas=schemas)
```