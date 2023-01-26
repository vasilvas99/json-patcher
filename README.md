# JSON Merge-Patcher

A cli tool that implements IETF-RFC7396 (the json-merge-patch). Takes two file paths as arguments and
either applies a patch or diffs the two jsons to generate the patch:

```shell
$ json-patcher diff --help
json-patcher-diff 

USAGE:
    json-patcher diff <SOURCE> <TARGET>

ARGS:
    <SOURCE>    <--------- The "source" template
    <TARGET>    <--------- The expected output after the generated patch is applied to the template

OPTIONS:
    -h, --help    Print help information
```


```shell
$ json-patcher patch --help
json-patcher-patch 

USAGE:
    json-patcher patch <TEMPLATE> <PATCH>

ARGS:
    <TEMPLATE>    <---------- The "source" template
    <PATCH>       <---------- The patch that would be applied to the source

OPTIONS:
    -h, --help    Print help information
```