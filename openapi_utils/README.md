# OpenAPI Utils

A crate for working with OpenAI specifications.

Currently there's only one feature implented:

## Merge

The merge command can be used to merge multiple OpenAPI spec files into one.

This might be useful, for instance, if you have multiple services behind the same API and you want to keep the OpenAPI spec next to the code for each service.

### Usage

```
Usage: openapi_utils merge [OPTIONS]

Options:
  -f, --file <FILE>
          The filename of the main template file
  -j, --json <JSON>
          The template as json
  -y, --yaml <YAML>
          The template  as yaml
  -e, --encoding <ENCODING>
          The template encoding (defaults to the file extension) [possible values: json, yaml, yml]
  -v, --var <TEMPLATE_VARS>
          Template variables
  -o, --output-file <OUTPUT>
          Output file path (if not specified, the result will print to stdout)
  -E, --output-encoding <OUTPUT_FORMAT>
          Output encoding (default is json) [possible values: json, yaml, yml]
  -w, --working-dir <WORKING_DIRECTORY>
          Paths in the template will be resolved relative to this directory (defaults to cwd)
  -h, --help
          Print help                          Print help
```

This command requires a template, which can be either specified using the `file` argument, or passed directoy using the `json` or `yaml` argument.

Here the template is like a normal OpenAPI spec, except that it has an extra `sources` dictionary, which can be used to specify other OpenAPI specs to merge into the final result.

So for instance, if we had this structure:

```
monorepo/
  service1/
    api-spec.yaml
  service2/
    api-spec.yaml
  api-spec.template.yaml
```

And our openapi specifications look like this:

#### `service1/api-spec.yaml`:

```
openapi: "3.0.0"
info:
  title: "Service 1"
  version: "1.0.0"
paths:
  /foo:
    get:
      responses:
        "200":
          description: OK
```

#### `service1/api-spec.yaml`:
```
openapi: "3.0.0"
info:
  title: "Service 2"
  version: "1.0.0"
paths:
  /bar:
    get:
      responses:
        "200":
          description: OK
```

#### `api-spec.template.yaml`:

```
openapi: "3.0.0"
info:
  title: "Combined Service"
  version: "1.0.0"
sources:
  ./service1/api-spec.yaml:
  ./service2/api-spec.yaml:
```

Running this command:

```
$ openapi_utils merge --file api-spec.template.yaml -F yaml
```

Will produce this result:

```
openapi: 3.0.3
info:
  title: 'Combined Service'
  version: '1.0.0'
paths:
  /foo:
    get:
      responses:
        '200':
          description: OK
  /bar:
    get:
      responses:
        '200':
          description: OK
```
