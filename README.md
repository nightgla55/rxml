ABOUT:
=======


This is a project designed to test the speed of Rust processing capabilities. Currently, the project has two modules- one parses XML to JSON, the other parses JSON to XML. The results are still being tweaked as some extra elements are added sometimes, and I'd like to be able to preserve the JSON's order after parsing. The currently supplied .whl was compiled with `maturin build`, and compiled on a raspberry pi inside a python3 venv- no guarantees of compatability are made. The wheel may be downloaded and then installed locally with `pip install /path/to/wheel`.








USAGE:
=======







![Image 3-25-24 at 7 56 PM](https://github.com/nightgla55/rxml/assets/38857821/94a633c8-58a9-47c4-bef2-479b56fa5f7d)


### The output needs to be tweaked. If there is no explicitly declared root, a null root is added



Here is an example of processing the SWISS-PROT Protein Sequence [XML Data Repository](https://aiweb.cs.washington.edu/research/projects/xmltk/xmldata/www/repository.html#SwissProt), its file size is about 110MB:


![Image 3-25-24 at 8 14 PM 2](https://github.com/nightgla55/rxml/assets/38857821/6bbcdd20-2d07-4c0e-91ab-62793d9ae9cd)







The first timed process is without writing to a file with json.dumps(), the second with. As can be seen, there is significant overhead induced when writing to a file, as expected. Maybe this can be a later feature of the project to further speed it up!
