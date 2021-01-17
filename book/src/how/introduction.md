# How OpenPowerlifting Works

This chapter gives a high-level introduction to the OpenPowerlifting system.

Although OpenPowerlifting is a data project, we don't use a database.
Instead, we use a novel design based on simple text spreadsheets and a compiler.
This allows contributors to work with data much more efficiently than is possible
with a traditional database system.

Specifically, we'll explain how competition data change from source documents into rankings.
We'll also look at the problems we identified with historical approaches and how we solved them.
