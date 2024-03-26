ABOUT:
=======


This is a project designed to test the speed of Rust processing capabilities. Currently, the project has two modules- one parses XML to JSON, the other parses JSON to XML. The results are still being tweaked as some extra elements are added sometimes, and I'd like to be able to preserve the JSON's order after parsing.




USAGE:
=======



![Image 3-25-24 at 7 56 PM](https://github.com/nightgla55/rxml/assets/38857821/94a633c8-58a9-47c4-bef2-479b56fa5f7d)


Here is an example of processing the SWISS-PROT Protein Sequence [XML Data Repository](https://aiweb.cs.washington.edu/research/projects/xmltk/xmldata/www/repository.html#SwissProt), its file size is about 110MB:

<img width="728" alt="Screen Shot 2024-03-25 at 6 59 10 PM" src="https://github.com/nightgla55/rxml/assets/38857821/88d174a6-b7f7-4eb1-b745-c4b6a603cf6e">

The first timed process is without writing to a file with json.dumps(), the second with. As can be seen, there is significant overhead induced when writing to a file, as expected. Maybe this can be a later feature of the project to further speed it up!
