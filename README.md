# ac-graphviz

## Installation

```shell
$ cargo install --git https://github.com/emanon001/ac-graphviz
```

## Usage

#### format

```
<vertex_number> <graph_option>
<from> <to> [<weight>]
...
```

- `graph_option`
  - if it contains `d`, it is directed graph

#### undirected graph

```shell
ac-graphviz > output.dot
2 _
1 2 2
2 1 1
```

```shell
$ cat output.dot
graph {
    1
    2
    
    1 -- 2 [label = "2"]
    2 -- 1 [label = "1"]
    
}
```

#### directed graph

```shell
ac-graphviz > output.dot
2 d
1 2 2
2 1 1
```

```shell
$ cat output.dot
digraph {
    1
    2
    
    1 -> 2 [label = "2"]
    2 -> 1 [label = "1"]
    
}
```