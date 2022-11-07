import xmlschema
from .iati2json import convert as convert_rs
from functools import lru_cache

current_schemas = [
    "https://raw.githubusercontent.com/IATI/IATI-Schemas/version-2.03/iati-activities-schema.xsd",
    "https://raw.githubusercontent.com/IATI/IATI-Schemas/version-2.03/iati-organisations-schema.xsd",
]

@lru_cache
def get_arrays_from_schemas(schemas=None):
    list_paths = []

    if not schemas:
        schemas = current_schemas

    def iterate(schema, current_path):
        if schema.occurs[1] is None:
            list_paths.append(current_path)

        for child in schema.iterchildren():
            if child.name:
                iterate(child, current_path + '/' + child.name)
    

    for schema_url in schemas:
        schema = xmlschema.XMLSchema(
            schema_url
        )
        activities = schema.findall('iati-activities')

        if activities:
            iterate(activities[0], '/iati-activities')

        organisation = schema.findall('iati-organisations')

        if organisation:
            iterate(organisation[0], '/iati-organisations')
    
    return list_paths

def convert(input, file=None, pretty=False, schemas=None, arrays=None):

    if not arrays:
        arrays = get_arrays_from_schemas(schemas)
    return convert_rs(input, file, pretty, arrays)