# Logged

Database system built specifically to store events

## Usage

### Meta command

 - `@db |> %exit`: exit Logged command-line

### Insert event

```
INSERT error (at='YYYY-DD-MM hh:mm:ss') code='xxx' metadata='{"hello": "world"}'
```

```
INSERT MANY [
    error (at='YYYY-DD-MM hh:mm:ss') code='yy' metadata='{"another": "log"}',
    error (at='YYYY-DD-MM hh:mm:ss') code='bb' metadata='{"andagain", "anotherlog}'
]
```

### Search for events

```
SELECT error
```

```
SELECT (error, warning)
```

### Delete events

**same than `SELECT` but prefix the query with `TO DELETE`**

```
SELECT TO DELETE info WHERE at < d'2019-01-01'
```
