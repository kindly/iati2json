import iati2json
import pathlib
import tempfile

def test_example_output_string():
    example_json = iati2json.convert("example/example.xml", pretty=True)
    expected_json = pathlib.Path("example/example.json").read_text()

    assert example_json == expected_json

def test_example_path():
    tmp_dir = tempfile.gettempdir()

    tmp_file = tmp_dir+"/xml2json_example.json"

    iati2json.convert("example/example.xml", pretty=True, file=tmp_file)

    example_json = pathlib.Path("example/example.json").read_text()
    expected_json = pathlib.Path(tmp_file).read_text()

    assert example_json == expected_json

def test_example_input_string():

    example_xml = pathlib.Path("example/example.xml").read_text()

    example_json =iati2json.convert(example_xml, pretty=True)

    expected_json = pathlib.Path("example/example.json").read_text()

    assert example_json == expected_json