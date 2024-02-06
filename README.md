# Reglite

A fast regular expression operator SQLite extension inspired by [sqlite-regex](https://github.com/asg017/sqlite-regex).
Supports `Blob` and `Text` nullable/not-null columns.

## Motivation

We started our regex support journey by trying the [sqlite-regex](https://github.com/asg017/sqlite-regex), it is a good library that is likely to support all your needs. Unfortunately, it only supports non-null `Text` columns and we could not use it in Caido since we needed nullable `Blob` column support. We also needed to not fail on bad regex patterns and instead just ignore the condition.

We first tried to fork it to adapt it to our needs, but it proved difficult since the library has many features that have implicit reliance on `str`. We decided it would be easier to build another extension since we only needed the `regexp` operator.

## Usage

This is intended to be statically compiled and loaded at runtime in your application:

TODO
